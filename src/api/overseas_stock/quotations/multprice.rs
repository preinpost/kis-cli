//! 해외주식 복수종목 시세조회 — GET /uapi/overseas-price/v1/quotations/multprice
//!
//! 스펙: .agent/specs/overseas_stock__quotations__multprice.md
//! 모의투자 미지원. EXCD_01~10, SYMB_01~10 (최대 10종목).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/multprice";
pub const TR_ID: &str = "HHDFS76220000";

/// 종목 1개 (excd: 거래소코드, symb: 종목코드)
#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub excd: String,
    pub symb: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub auth: String,
    /// 최대 10개. nrec은 자동 계산.
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quote {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub excd: String,
    #[serde(default)]
    pub symb: String,
    #[serde(default)]
    pub knam: String,
    #[serde(default)]
    pub exnm: String,
    #[serde(default)]
    pub nnam: String,
    #[serde(default)]
    pub stat1: String,
    #[serde(default)]
    pub stat2: String,
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub last: String,
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
    pub pbid: String,
    #[serde(default)]
    pub pask: String,
    #[serde(default)]
    pub vbid: String,
    #[serde(default)]
    pub vask: String,
    #[serde(default)]
    pub bvol: String,
    #[serde(default)]
    pub avol: String,
    #[serde(default)]
    pub evol: String,
    #[serde(default)]
    pub tvol: String,
    #[serde(default)]
    pub tamt: String,
    #[serde(default)]
    pub powx: String,
    #[serde(default)]
    pub xhms: String,
    #[serde(default)]
    pub khms: String,
    #[serde(default)]
    pub curr: String,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub pvol: String,
    #[serde(default)]
    pub pamt: String,
    #[serde(default)]
    pub popen: String,
    #[serde(default)]
    pub phigh: String,
    #[serde(default)]
    pub plow: String,
    #[serde(default)]
    pub shar: String,
    #[serde(default)]
    pub mcap: String,
    #[serde(default)]
    pub tomv: String,
    #[serde(default)]
    pub h52p: String,
    #[serde(default)]
    pub l52p: String,
    #[serde(default)]
    pub h52d: String,
    #[serde(default)]
    pub l52d: String,
    #[serde(default)]
    pub hanp: String,
    #[serde(default)]
    pub lanp: String,
    #[serde(default)]
    pub hand: String,
    #[serde(default)]
    pub land: String,
    #[serde(default)]
    pub bnit: String,
    #[serde(default)]
    pub t_xprc: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Quote>> {
    if client.is_mock() {
        bail!("해외주식 복수종목 시세조회는 모의투자 미지원");
    }
    let nrec = req.items.len().to_string();
    let mut params: Vec<(String, String)> = vec![
        ("AUTH".into(), req.auth.clone()),
        ("NREC".into(), nrec),
    ];
    for (idx, item) in req.items.iter().enumerate().take(10) {
        let n = idx + 1;
        params.push((format!("EXCD_{:02}", n), item.excd.clone()));
        params.push((format!("SYMB_{:02}", n), item.symb.clone()));
    }
    let params_ref: Vec<(&str, &str)> = params
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let resp = client.get(ENDPOINT, TR_ID, &params_ref).await?;
    let quotes = resp
        .output2
        .map(serde_json::from_value::<Vec<Quote>>)
        .transpose()?
        .unwrap_or_default();
    Ok(quotes)
}
