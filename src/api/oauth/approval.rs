//! 실시간 (웹소켓) 접속키 발급 — POST /oauth2/Approval
//!
//! 스펙: .agent/specs/oauth__approval.md
//!
//! L06 규칙: OAuth 엔드포인트는 Bearer 없이 호출.
//! L08 규칙: 응답은 `ApiResponse` 래퍼가 아닌 평면 JSON.
//!
//! NOTE: body 필드 이름이 `secretkey`다 (다른 API의 `appsecret`가 아님). 스펙 주의.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::{BASE_URL_MOCK, BASE_URL_PROD};

pub const ENDPOINT: &str = "/oauth2/Approval";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub grant_type: String,
    pub appkey: String,
    pub secretkey: String,
}

impl Request {
    pub fn client_credentials(app_key: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            grant_type: "client_credentials".to_string(),
            appkey: app_key.into(),
            secretkey: app_secret.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub approval_key: String,
}

pub async fn call(is_mock: bool, req: &Request) -> Result<Response> {
    crate::rate_limit::acquire(is_mock).await?;
    let base = if is_mock { BASE_URL_MOCK } else { BASE_URL_PROD };
    let url = format!("{base}{ENDPOINT}");
    let resp = reqwest::Client::new()
        .post(&url)
        .header("content-type", "application/json; utf-8")
        .json(req)
        .send()
        .await
        .context("웹소켓 접속키 발급 요청 실패")?;
    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        bail!("HTTP {status}: {body}");
    }
    serde_json::from_str(&body).context("웹소켓 접속키 응답 파싱 실패")
}
