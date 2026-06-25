//! 해외주식 현재체결가 — GET /uapi/overseas-price/v1/quotations/price
//!
//! 스펙: .agent/specs/overseas_stock__quotations__price.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/price";
pub const TR_ID: &str = "HHDFS00000300";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub auth: String,
    pub excd: String,
    pub symb: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub pvol: String,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub sign: String,
    #[serde(default)]
    pub diff: String,
    #[serde(default)]
    pub rate: String,
    #[serde(default)]
    pub tvol: String,
    #[serde(default)]
    pub tamt: String,
    #[serde(default)]
    pub ordy: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
        ("SYMB", req.symb.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
