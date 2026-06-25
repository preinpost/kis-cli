//! 해외주식분봉조회 — GET /uapi/overseas-price/v1/quotations/inquire-time-itemchartprice
//!
//! 스펙: .agent/specs/overseas_stock__quotations__inquire_time_itemchartprice.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/inquire-time-itemchartprice";
pub const TR_ID: &str = "HHDFS76950200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub auth: String,
    pub excd: String,
    pub symb: String,
    pub nmin: String,
    pub pinc: String,
    pub next: String,
    pub nrec: String,
    pub fill: String,
    pub keyb: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub stim: String,
    #[serde(default)]
    pub etim: String,
    #[serde(default)]
    pub sktm: String,
    #[serde(default)]
    pub ektm: String,
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub more: String,
    #[serde(default)]
    pub nrec: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bar {
    #[serde(default)]
    pub tymd: String,
    #[serde(default)]
    pub xymd: String,
    #[serde(default)]
    pub xhms: String,
    #[serde(default)]
    pub kymd: String,
    #[serde(default)]
    pub khms: String,
    #[serde(default)]
    pub open: String,
    #[serde(default)]
    pub high: String,
    #[serde(default)]
    pub low: String,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub evol: String,
    #[serde(default)]
    pub eamt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub bars: Vec<Bar>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식분봉조회는 모의투자 미지원");
    }
    let params = [
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
        ("SYMB", req.symb.as_str()),
        ("NMIN", req.nmin.as_str()),
        ("PINC", req.pinc.as_str()),
        ("NEXT", req.next.as_str()),
        ("NREC", req.nrec.as_str()),
        ("FILL", req.fill.as_str()),
        ("KEYB", req.keyb.as_str()),
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
