//! 해외주식 권리종합 — GET /uapi/overseas-price/v1/quotations/rights-by-ice
//!
//! 스펙: .agent/specs/overseas_stock__market_analysis__rights_by_ice.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/rights-by-ice";
pub const TR_ID: &str = "HHDFS78330900";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub ncod: String,
    pub symb: String,
    pub st_ymd: String,
    pub ed_ymd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub anno_dt: String,
    #[serde(default)]
    pub ca_title: String,
    #[serde(default)]
    pub div_lock_dt: String,
    #[serde(default)]
    pub pay_dt: String,
    #[serde(default)]
    pub record_dt: String,
    #[serde(default)]
    pub validity_dt: String,
    #[serde(default)]
    pub local_end_dt: String,
    #[serde(default)]
    pub lock_dt: String,
    #[serde(default)]
    pub delist_dt: String,
    #[serde(default)]
    pub redempt_dt: String,
    #[serde(default)]
    pub early_redempt_dt: String,
    #[serde(default)]
    pub effective_dt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("해외주식 권리종합은 모의투자 미지원");
    }
    let params = [
        ("NCOD", req.ncod.as_str()),
        ("SYMB", req.symb.as_str()),
        ("ST_YMD", req.st_ymd.as_str()),
        ("ED_YMD", req.ed_ymd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output1
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(rows)
}
