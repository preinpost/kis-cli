//! 해외주식 체결추이 — GET /uapi/overseas-price/v1/quotations/inquire-ccnl
//!
//! 스펙: .agent/specs/overseas_stock__quotations__inquire_ccnl.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/inquire-ccnl";
pub const TR_ID: &str = "HHDFS76200300";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub excd: String,
    pub auth: String,
    pub keyb: String,
    pub tday: String,
    pub symb: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub rsym: String,
    #[serde(default, rename = "ZDIV")]
    pub zdiv: String,
    #[serde(default, rename = "NREC")]
    pub nrec: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tick {
    #[serde(default)]
    pub khms: String,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub sign: String,
    #[serde(default)]
    pub diff: String,
    #[serde(default)]
    pub rate: String,
    #[serde(default)]
    pub evol: String,
    #[serde(default)]
    pub tvol: String,
    #[serde(default)]
    pub mtyp: String,
    #[serde(default)]
    pub pbid: String,
    #[serde(default)]
    pub pask: String,
    #[serde(default)]
    pub vpow: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub ticks: Vec<Tick>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 체결추이는 모의투자 미지원");
    }
    let params = [
        ("EXCD", req.excd.as_str()),
        ("AUTH", req.auth.as_str()),
        ("KEYB", req.keyb.as_str()),
        ("TDAY", req.tday.as_str()),
        ("SYMB", req.symb.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let ticks = resp
        .output2
        .map(serde_json::from_value::<Vec<Tick>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, ticks })
}
