//! 접근토큰폐기(P) — POST /oauth2/revokeP
//!
//! 스펙: .agent/specs/oauth__revokep.md
//!
//! L06 규칙: OAuth 엔드포인트는 Bearer 없이 호출 → `reqwest::Client`로 직접 POST.
//! L08 규칙: 응답은 `ApiResponse` 래퍼가 아닌 평면 JSON.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::{BASE_URL_MOCK, BASE_URL_PROD};

pub const ENDPOINT: &str = "/oauth2/revokeP";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub appkey: String,
    pub appsecret: String,
    pub token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub message: String,
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
        .context("토큰 폐기 요청 실패")?;
    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        bail!("HTTP {status}: {body}");
    }
    serde_json::from_str(&body).context("토큰 폐기 응답 파싱 실패")
}
