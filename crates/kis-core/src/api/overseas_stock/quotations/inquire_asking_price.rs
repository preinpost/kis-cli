//! 해외주식 현재가 호가 — GET /uapi/overseas-price/v1/quotations/inquire-asking-price
//!
//! 스펙: .agent/specs/overseas_stock__quotations__inquire_asking_price.md
//! 모의투자 미지원. output3는 ApiResponse 미노출 → 무시.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/inquire-asking-price";
pub const TR_ID: &str = "HHDFS76200100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub auth: String,
    pub excd: String,
    pub symb: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub curr: String,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub open: String,
    #[serde(default)]
    pub high: String,
    #[serde(default)]
    pub low: String,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub dymd: String,
    #[serde(default)]
    pub dhms: String,
    #[serde(default)]
    pub bvol: String,
    #[serde(default)]
    pub avol: String,
    #[serde(default)]
    pub bdvl: String,
    #[serde(default)]
    pub advl: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub ropen: String,
    #[serde(default)]
    pub rhigh: String,
    #[serde(default)]
    pub rlow: String,
    #[serde(default)]
    pub rclose: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Quote {
    #[serde(default)]
    pub pbid1: String,
    #[serde(default)]
    pub pask1: String,
    #[serde(default)]
    pub vbid1: String,
    #[serde(default)]
    pub vask1: String,
    #[serde(default)]
    pub dbid1: String,
    #[serde(default)]
    pub dask1: String,
    #[serde(default)]
    pub pbid2: String,
    #[serde(default)]
    pub pask2: String,
    #[serde(default)]
    pub vbid2: String,
    #[serde(default)]
    pub vask2: String,
    #[serde(default)]
    pub dbid2: String,
    #[serde(default)]
    pub dask2: String,
    #[serde(default)]
    pub pbid3: String,
    #[serde(default)]
    pub pask3: String,
    #[serde(default)]
    pub vbid3: String,
    #[serde(default)]
    pub vask3: String,
    #[serde(default)]
    pub dbid3: String,
    #[serde(default)]
    pub dask3: String,
    #[serde(default)]
    pub pbid4: String,
    #[serde(default)]
    pub pask4: String,
    #[serde(default)]
    pub vbid4: String,
    #[serde(default)]
    pub vask4: String,
    #[serde(default)]
    pub dbid4: String,
    #[serde(default)]
    pub dask4: String,
    #[serde(default)]
    pub pbid5: String,
    #[serde(default)]
    pub pask5: String,
    #[serde(default)]
    pub vbid5: String,
    #[serde(default)]
    pub vask5: String,
    #[serde(default)]
    pub dbid5: String,
    #[serde(default)]
    pub dask5: String,
    #[serde(default)]
    pub pbid6: String,
    #[serde(default)]
    pub pask6: String,
    #[serde(default)]
    pub vbid6: String,
    #[serde(default)]
    pub vask6: String,
    #[serde(default)]
    pub dbid6: String,
    #[serde(default)]
    pub dask6: String,
    #[serde(default)]
    pub pbid7: String,
    #[serde(default)]
    pub pask7: String,
    #[serde(default)]
    pub vbid7: String,
    #[serde(default)]
    pub vask7: String,
    #[serde(default)]
    pub dbid7: String,
    #[serde(default)]
    pub dask7: String,
    #[serde(default)]
    pub pbid8: String,
    #[serde(default)]
    pub pask8: String,
    #[serde(default)]
    pub vbid8: String,
    #[serde(default)]
    pub vask8: String,
    #[serde(default)]
    pub dbid8: String,
    #[serde(default)]
    pub dask8: String,
    #[serde(default)]
    pub pbid9: String,
    #[serde(default)]
    pub pask9: String,
    #[serde(default)]
    pub vbid9: String,
    #[serde(default)]
    pub vask9: String,
    #[serde(default)]
    pub dbid9: String,
    #[serde(default)]
    pub dask9: String,
    #[serde(default)]
    pub pbid10: String,
    #[serde(default)]
    pub pask10: String,
    #[serde(default)]
    pub vbid10: String,
    #[serde(default)]
    pub vask10: String,
    #[serde(default)]
    pub dbid10: String,
    #[serde(default)]
    pub dask10: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub quotes: Vec<Quote>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 현재가 호가는 모의투자 미지원");
    }
    let params = [
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
        ("SYMB", req.symb.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let quotes = resp
        .output2
        .map(serde_json::from_value::<Vec<Quote>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, quotes })
}
