//! NAV 비교추이(일) — GET /uapi/etfetn/v1/quotations/nav-comparison-daily-trend
//!
//! 스펙: .agent/specs/domestic_stock__quotations__nav_comparison_daily_trend.md
//!
//! 모의투자 미지원. output이 array (일별 NAV 비교).

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/etfetn/v1/quotations/nav-comparison-daily-trend";
pub const TR_ID: &str = "FHPST02440200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// YYYYMMDD
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_clpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub cntg_vol: String,
    #[serde(default)]
    pub dprt: String,
    #[serde(default)]
    pub nav_vrss_prpr: String,
    #[serde(default)]
    pub nav: String,
    #[serde(default)]
    pub nav_prdy_vrss_sign: String,
    #[serde(default)]
    pub nav_prdy_vrss: String,
    #[serde(default)]
    pub nav_prdy_ctrt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("NAV 비교추이(일)는 모의투자 미지원 API입니다");
    }
    let params = [
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_input_date_1", req.fid_input_date_1.as_str()),
        ("fid_input_date_2", req.fid_input_date_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
