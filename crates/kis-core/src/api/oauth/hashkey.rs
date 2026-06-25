//! Hashkey — POST /uapi/hashkey
//!
//! 스펙: .agent/specs/oauth__hashkey.md
//!
//! L06 규칙: OAuth 엔드포인트는 Bearer 없이 호출.
//! L08 규칙: 응답은 `ApiResponse` 래퍼가 아닌 평면 JSON (JsonBody + HASH).
//!
//! 이 API는 임의의 JSON body를 받아 해시값을 돌려주므로 `Request`가 고정 구조체가
//! 아니라 generic `T: Serialize`로 받는다.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::{BASE_URL_MOCK, BASE_URL_PROD};

pub const ENDPOINT: &str = "/uapi/hashkey";

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "HASH")]
    pub hash: String,
    #[serde(default, rename = "JsonBody")]
    pub json_body: serde_json::Value,
}

pub async fn call<T: Serialize + ?Sized>(
    is_mock: bool,
    app_key: &str,
    app_secret: &str,
    body: &T,
) -> Result<Response> {
    crate::rate_limit::acquire(is_mock).await?;
    let base = if is_mock { BASE_URL_MOCK } else { BASE_URL_PROD };
    let url = format!("{base}{ENDPOINT}");
    let resp = reqwest::Client::new()
        .post(&url)
        .header("content-type", "application/json; charset=utf-8")
        .header("appkey", app_key)
        .header("appsecret", app_secret)
        .json(body)
        .send()
        .await
        .context("hashkey 요청 실패")?;
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        bail!("HTTP {status}: {text}");
    }
    serde_json::from_str(&text).context("hashkey 응답 파싱 실패")
}
