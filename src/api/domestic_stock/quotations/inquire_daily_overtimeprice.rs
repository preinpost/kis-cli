//! 주식현재가 시간외일자별주가 — GET /uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_daily_overtimeprice.md
//!
//! 최근 30일. output1(시간외 단일가 현재) + output2(일자별 Vec).

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice";
pub const TR_ID: &str = "FHPST02320000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J 주식/ETF/ETN
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub ovtm_untp_prpr: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss_sign: String,
    #[serde(default)]
    pub ovtm_untp_prdy_ctrt: String,
    #[serde(default)]
    pub ovtm_untp_vol: String,
    #[serde(default)]
    pub ovtm_untp_tr_pbmn: String,
    #[serde(default)]
    pub ovtm_untp_mxpr: String,
    #[serde(default)]
    pub ovtm_untp_llam: String,
    #[serde(default)]
    pub ovtm_untp_oprc: String,
    #[serde(default)]
    pub ovtm_untp_hgpr: String,
    #[serde(default)]
    pub ovtm_untp_lwpr: String,
    #[serde(default)]
    pub ovtm_untp_antc_cnpr: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_vrss: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_ctrt: String,
    #[serde(default)]
    pub ovtm_untp_antc_vol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DailyRow {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub ovtm_untp_prpr: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss_sign: String,
    #[serde(default)]
    pub ovtm_untp_prdy_ctrt: String,
    #[serde(default)]
    pub ovtm_untp_vol: String,
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
    pub ovtm_untp_tr_pbmn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub daily: Vec<DailyRow>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let daily: Vec<DailyRow> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, daily })
}
