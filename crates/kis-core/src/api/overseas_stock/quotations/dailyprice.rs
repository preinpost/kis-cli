//! 해외주식 기간별시세 — GET /uapi/overseas-price/v1/quotations/dailyprice
//!
//! 스펙: .agent/specs/overseas_stock__quotations__dailyprice.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/dailyprice";
pub const TR_ID: &str = "HHDFS76240000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub auth: String,
    pub excd: String,
    pub symb: String,
    pub gubn: String,
    pub bymd: String,
    pub modp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub nrec: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bar {
    #[serde(default)]
    pub xymd: String,
    #[serde(default)]
    pub clos: String,
    #[serde(default)]
    pub sign: String,
    #[serde(default)]
    pub diff: String,
    #[serde(default)]
    pub rate: String,
    #[serde(default)]
    pub open: String,
    #[serde(default)]
    pub high: String,
    #[serde(default)]
    pub low: String,
    #[serde(default)]
    pub tvol: String,
    #[serde(default)]
    pub tamt: String,
    #[serde(default)]
    pub pbid: String,
    #[serde(default)]
    pub vbid: String,
    #[serde(default)]
    pub pask: String,
    #[serde(default)]
    pub vask: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub bars: Vec<Bar>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
        ("SYMB", req.symb.as_str()),
        ("GUBN", req.gubn.as_str()),
        ("BYMD", req.bymd.as_str()),
        ("MODP", req.modp.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let bars = resp
        .output2
        .map(serde_json::from_value::<Vec<Bar>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, bars })
}
