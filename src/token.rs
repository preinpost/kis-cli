use std::fs;
use std::sync::Mutex;

use anyhow::{Context, Result};
use chrono::{Duration, Local, NaiveDateTime};
use reqwest::Client;

use crate::config::{self, Credentials};
use crate::models::{KisAccessToken, KisApprovalKey};

const BASE_URL: &str = "https://openapi.koreainvestment.com:9443";
const EXPIRY_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct TokenManager {
    credentials: Credentials,
    http: Client,
    token: Mutex<Option<KisAccessToken>>,
    ws_token: Mutex<Option<KisApprovalKey>>,
}

impl TokenManager {
    pub fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            http: Client::new(),
            token: Mutex::new(None),
            ws_token: Mutex::new(None),
        }
    }

    // ── REST 토큰 ──

    /// 유효한 액세스 토큰을 반환 (3단계 캐싱)
    pub async fn get_token(&self) -> Result<KisAccessToken> {
        // 1) 메모리 캐시
        {
            let guard = self.token.lock().unwrap();
            if let Some(ref t) = *guard {
                if !Self::is_expired(&t.access_token_token_expired) {
                    return Ok(t.clone());
                }
            }
        }

        // 2) 파일 캐시
        if let Ok(path) = config::token_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(t) = serde_json::from_str::<KisAccessToken>(&content) {
                    if !Self::is_expired(&t.access_token_token_expired) {
                        *self.token.lock().unwrap() = Some(t.clone());
                        return Ok(t);
                    }
                }
            }
        }

        // 3) API 요청
        let token = self.request_new_token().await?;
        *self.token.lock().unwrap() = Some(token.clone());
        Ok(token)
    }

    /// 액세스 토큰 문자열만 반환
    pub async fn get_access_token_string(&self) -> Result<String> {
        let t = self.get_token().await?;
        Ok(t.access_token)
    }

    async fn request_new_token(&self) -> Result<KisAccessToken> {
        let url = format!("{BASE_URL}/oauth2/tokenP");
        let body = serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": self.credentials.app_key,
            "appsecret": self.credentials.app_secret,
        });

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("토큰 요청 실패")?;

        let token: KisAccessToken = resp
            .json()
            .await
            .context("토큰 응답 파싱 실패")?;

        // 파일에 캐싱
        if let Ok(path) = config::token_path() {
            if let Ok(dir) = config::config_dir() {
                let _ = fs::create_dir_all(dir);
            }
            let _ = fs::write(&path, serde_json::to_string_pretty(&token)?);
        }

        Ok(token)
    }

    // ── WebSocket Approval Key ──

    /// 유효한 WebSocket approval key를 반환 (3단계 캐싱)
    pub async fn get_ws_token(&self) -> Result<KisApprovalKey> {
        // 1) 메모리
        {
            let guard = self.ws_token.lock().unwrap();
            if let Some(ref t) = *guard {
                if !Self::is_expired(&t.approval_key_expired) {
                    return Ok(t.clone());
                }
            }
        }

        // 2) 파일
        if let Ok(path) = config::ws_token_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(t) = serde_json::from_str::<KisApprovalKey>(&content) {
                    if !Self::is_expired(&t.approval_key_expired) {
                        *self.ws_token.lock().unwrap() = Some(t.clone());
                        return Ok(t);
                    }
                }
            }
        }

        // 3) API 요청
        let token = self.request_new_ws_token().await?;
        *self.ws_token.lock().unwrap() = Some(token.clone());
        Ok(token)
    }

    pub async fn get_ws_approval_key_string(&self) -> Result<String> {
        let t = self.get_ws_token().await?;
        Ok(t.approval_key)
    }

    async fn request_new_ws_token(&self) -> Result<KisApprovalKey> {
        let url = format!("{BASE_URL}/oauth2/Approval");
        let body = serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": self.credentials.app_key,
            "secretkey": self.credentials.app_secret,
        });

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("WebSocket approval key 요청 실패")?;

        let data: serde_json::Value = resp
            .json()
            .await
            .context("WebSocket approval key 응답 파싱 실패")?;

        let approval_key = data["approval_key"]
            .as_str()
            .context("approval_key 필드 없음")?
            .to_string();

        // 24시간 유효
        let expired = (Local::now() + Duration::hours(24))
            .format(EXPIRY_FORMAT)
            .to_string();

        let token = KisApprovalKey {
            approval_key,
            approval_key_expired: expired,
        };

        // 파일에 캐싱
        if let Ok(path) = config::ws_token_path() {
            if let Ok(dir) = config::config_dir() {
                let _ = fs::create_dir_all(dir);
            }
            let _ = fs::write(&path, serde_json::to_string_pretty(&token)?);
        }

        Ok(token)
    }

    // ── 유틸 ──

    /// 만료 여부 확인 (5분 버퍼)
    fn is_expired(expiry_str: &str) -> bool {
        let Ok(expiry) = NaiveDateTime::parse_from_str(expiry_str, EXPIRY_FORMAT) else {
            return true;
        };
        let now = Local::now().naive_local();
        now >= expiry - Duration::minutes(5)
    }

    /// 토큰 무효화 (캐시 삭제)
    pub fn invalidate(&self) {
        *self.token.lock().unwrap() = None;
        if let Ok(path) = config::token_path() {
            let _ = fs::remove_file(path);
        }
    }
}
