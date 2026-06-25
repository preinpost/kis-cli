//! ELW 민감도 추이(체결) — GET /uapi/elw/v1/quotations/sensitivity-trend-ccnl
//!
//! 스펙: .agent/specs/domestic_stock__elw__sensitivity_trend_ccnl.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/sensitivity-trend-ccnl";
pub const TR_ID: &str = "FHPEW02830100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub hts_thpr: String,
    #[serde(default)]
    pub delta_val: String,
    #[serde(default)]
    pub gama: String,
    #[serde(default)]
    pub theta: String,
    #[serde(default)]
    pub vega: String,
    #[serde(default)]
    pub rho: String,
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
