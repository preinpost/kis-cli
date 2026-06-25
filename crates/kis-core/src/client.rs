use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context, Result};
use reqwest::{Client, Method};
use serde::Serialize;

use crate::config::Credentials;
use crate::models::ApiResponse;
use crate::token::TokenManager;

pub const BASE_URL_PROD: &str = "https://openapi.koreainvestment.com:9443";
pub const BASE_URL_MOCK: &str = "https://openapivts.koreainvestment.com:29443";

pub struct KisClient {
    pub token_manager: Arc<TokenManager>,
    credentials: Credentials,
    is_mock: bool,
    http: Client,
}

impl KisClient {
    pub fn new(credentials: Credentials) -> Self {
        Self::with_mock(credentials, false)
    }

    pub fn with_mock(credentials: Credentials, is_mock: bool) -> Self {
        let token_manager = Arc::new(TokenManager::new(Credentials {
            app_key: credentials.app_key.clone(),
            app_secret: credentials.app_secret.clone(),
            account_number: credentials.account_number.clone(),
        }));
        Self {
            token_manager,
            credentials,
            is_mock,
            http: Client::new(),
        }
    }

    pub fn is_mock(&self) -> bool {
        self.is_mock
    }

    pub fn base_url(&self) -> &'static str {
        if self.is_mock {
            BASE_URL_MOCK
        } else {
            BASE_URL_PROD
        }
    }

    async fn headers(&self, tr_id: &str) -> Result<HashMap<String, String>> {
        let access_token = self.token_manager.get_access_token_string().await?;
        let mut h = HashMap::new();
        h.insert("Content-Type".into(), "application/json; charset=utf-8".into());
        h.insert("authorization".into(), format!("Bearer {access_token}"));
        h.insert("appkey".into(), self.credentials.app_key.clone());
        h.insert("appsecret".into(), self.credentials.app_secret.clone());
        h.insert("tr_id".into(), tr_id.into());
        Ok(h)
    }

    /// GET 요청. params의 키는 자동으로 대문자로 변환된다.
    pub async fn get(
        &self,
        endpoint: &str,
        tr_id: &str,
        params: &[(&str, &str)],
    ) -> Result<ApiResponse> {
        let mut attempt = 0u32;
        loop {
            crate::rate_limit::acquire(self.is_mock).await?;
            let headers = self.headers(tr_id).await?;
            let url = format!("{}{endpoint}", self.base_url());

            let mut req = self.http.get(&url);
            for (k, v) in &headers {
                req = req.header(k.as_str(), v.as_str());
            }

            let upper_params: Vec<(String, String)> = params
                .iter()
                .map(|(k, v)| (k.to_uppercase(), v.to_string()))
                .collect();
            let refs: Vec<(&str, &str)> = upper_params
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            req = req.query(&refs);

            let resp = req.send().await.context("API 요청 실패")?;
            match parse_response(resp).await {
                Ok(ok) => return Ok(ok),
                Err(e) if attempt == 0 && is_rate_limited(&e) => {
                    attempt += 1;
                    tokio::time::sleep(backoff_duration()).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// POST 요청 (JSON body). extra_headers는 tr_cont, hashkey 등 API별 추가 헤더.
    pub async fn post_json<B: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        tr_id: &str,
        body: &B,
        extra_headers: &[(&str, &str)],
    ) -> Result<ApiResponse> {
        self.send_json(Method::POST, endpoint, tr_id, body, extra_headers).await
    }

    /// 임의의 HTTP method + JSON body 요청 (DELETE, PUT 등 범용).
    pub async fn send_json<B: Serialize + ?Sized>(
        &self,
        method: Method,
        endpoint: &str,
        tr_id: &str,
        body: &B,
        extra_headers: &[(&str, &str)],
    ) -> Result<ApiResponse> {
        let mut attempt = 0u32;
        loop {
            crate::rate_limit::acquire(self.is_mock).await?;
            let headers = self.headers(tr_id).await?;
            let url = format!("{}{endpoint}", self.base_url());

            let mut req = self.http.request(method.clone(), &url);
            for (k, v) in &headers {
                req = req.header(k.as_str(), v.as_str());
            }
            for (k, v) in extra_headers {
                req = req.header(*k, *v);
            }
            req = req.json(body);

            let resp = req.send().await.context("API 요청 실패")?;
            match parse_response(resp).await {
                Ok(ok) => return Ok(ok),
                Err(e) if attempt == 0 && is_rate_limited(&e) => {
                    attempt += 1;
                    tokio::time::sleep(backoff_duration()).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub fn cano(&self) -> &str {
        self.credentials.cano()
    }

    pub fn product_code(&self) -> &str {
        self.credentials.product_code()
    }

    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }
}

async fn parse_response(resp: reqwest::Response) -> Result<ApiResponse> {
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        bail!("HTTP {status}: {body}");
    }

    let data: ApiResponse = resp.json().await.context("응답 파싱 실패")?;

    if data.rt_cd != "0" {
        bail!("API 오류 [{}]: {}", data.msg_cd, data.msg1);
    }

    Ok(data)
}

/// KIS 초당 거래건수 초과(EGW00201) 판정 — HTTP 500 body 또는 rt_cd!=0 메시지 어디서나 커버.
fn is_rate_limited(err: &anyhow::Error) -> bool {
    let msg = format!("{err}");
    msg.contains("EGW00201") || msg.contains("초당 거래건수")
}

/// 250~500ms 범위의 결정론 아닌 backoff — 다중 프로세스가 동시에 재시도해도 분산되도록 시간 기반 지터.
fn backoff_duration() -> Duration {
    let jitter = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| (d.subsec_nanos() as u64) % 250)
        .unwrap_or(0);
    Duration::from_millis(250 + jitter)
}
