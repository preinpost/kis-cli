//! 해외주식 업종별코드조회 — GET /uapi/overseas-price/v1/quotations/industry-price
//!
//! 스펙: .agent/specs/overseas_stock__quotations__industry_price.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/industry-price";
pub const TR_ID: &str = "HHDFS76370100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub auth: String,
    pub excd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub nrec: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Industry {
    #[serde(default)]
    pub icod: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub industries: Vec<Industry>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 업종별코드조회는 모의투자 미지원");
    }
    let params = [
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let industries = resp
        .output2
        .map(serde_json::from_value::<Vec<Industry>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, industries })
}
