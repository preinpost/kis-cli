//! 주식현재가 시간외시간별체결 — GET /uapi/domestic-stock/v1/quotations/inquire-time-overtimeconclusion
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_time_overtimeconclusion.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-time-overtimeconclusion";
pub const TR_ID: &str = "FHPST02310000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J 주식/ETF/ETN
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// 1 시간외 (Default)
    pub fid_hour_cls_code: String,
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
    #[serde(default)]
    pub uplm_sign: String,
    #[serde(default)]
    pub lslm_sign: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tick {
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub askp: String,
    #[serde(default)]
    pub bidp: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub cntg_vol: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub ticks: Vec<Tick>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_HOUR_CLS_CODE", req.fid_hour_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let ticks: Vec<Tick> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, ticks })
}
