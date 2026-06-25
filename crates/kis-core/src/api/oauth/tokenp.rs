//! 접근토큰발급(P) — POST /oauth2/tokenP
//!
//! 스펙: .agent/specs/oauth__tokenp.md
//!
//! NOTE: OAuth 토큰 발급은 Bearer 토큰 없이 호출해야 하므로 `KisClient::post_json`을
//! 쓸 수 없다. 자체 `reqwest::Client`로 POST한다.
//! 실제 토큰 캐싱/재사용은 `crate::token::TokenManager`가 담당한다 — 이 모듈은 순수
//! 요청/응답 타입과 1회성 호출 헬퍼만 제공한다.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::{BASE_URL_MOCK, BASE_URL_PROD};

pub const ENDPOINT: &str = "/oauth2/tokenP";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub grant_type: String,
    pub appkey: String,
    pub appsecret: String,
}

impl Request {
    pub fn client_credentials(app_key: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            grant_type: "client_credentials".to_string(),
            appkey: app_key.into(),
            appsecret: app_secret.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub access_token: String,
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(default)]
    pub access_token_token_expired: String,
}

pub async fn call(is_mock: bool, req: &Request) -> Result<Response> {
    crate::rate_limit::acquire(is_mock).await?;
    let base = if is_mock { BASE_URL_MOCK } else { BASE_URL_PROD };
    let url = format!("{base}{ENDPOINT}");
    let resp = reqwest::Client::new()
        .post(&url)
        .header("content-type", "application/json; charset=UTF-8")
        .json(req)
        .send()
        .await
        .context("토큰 발급 요청 실패")?;
    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        bail!("HTTP {status}: {body}");
    }
    serde_json::from_str(&body).context("토큰 응답 파싱 실패")
}
