//! 해외주식 거래량급증 — GET /uapi/overseas-stock/v1/ranking/volume-surge
//!
//! 스펙: .agent/specs/overseas_stock__market_analysis__volume_surge.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/ranking/volume-surge";
pub const TR_ID: &str = "HHDFS76270000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub keyb: String,
    pub auth: String,
    pub excd: String,
    pub minx: String,
    pub vol_rang: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub stat: String,
    #[serde(default)]
    pub nrec: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub excd: String,
    #[serde(default)]
    pub symb: String,
    #[serde(default)]
    pub knam: String,
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
    pub pask: String,
    #[serde(default)]
    pub pbid: String,
    #[serde(default)]
    pub n_tvol: String,
    #[serde(default)]
    pub n_diff: String,
    #[serde(default)]
    pub n_rate: String,
    #[serde(default)]
    pub enam: String,
    #[serde(default)]
    pub e_ordyn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 거래량급증은 모의투자 미지원");
    }
    let params = [
        ("KEYB", req.keyb.as_str()),
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
        ("MINX", req.minx.as_str()),
        ("VOL_RANG", req.vol_rang.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let rows = resp
        .output2
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, rows })
}
