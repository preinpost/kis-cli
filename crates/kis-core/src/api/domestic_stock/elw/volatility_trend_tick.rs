//! ELW 변동성 추이(틱) — GET /uapi/elw/v1/quotations/volatility-trend-tick
//!
//! 스펙: .agent/specs/domestic_stock__elw__volatility_trend_tick.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/volatility-trend-tick";
pub const TR_ID: &str = "FHPEW02840400";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// W
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bsop_date: String,
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub hts_ints_vltl: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
