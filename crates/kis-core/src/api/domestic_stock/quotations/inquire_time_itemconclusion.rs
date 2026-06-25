//! 주식현재가 당일시간대별체결 — GET /uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_time_itemconclusion.md
//!
//! output1(Meta) + output2(체결 Vec). FID_INPUT_HOUR_1로 과거 시간대 조회 가능.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion";
pub const TR_ID: &str = "FHPST01060000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_hour_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tick {
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub stck_pbpr: String,
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
    pub tday_rltv: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub cnqn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    /// output2가 스펙상 single이지만 array로 올 수 있어 Vec로 수용
    pub ticks: Vec<Tick>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let ticks: Vec<Tick> = match resp.output2 {
        Some(serde_json::Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect(),
        Some(v) => serde_json::from_value::<Tick>(v).map(|t| vec![t]).unwrap_or_default(),
        None => vec![],
    };
    Ok(Response { meta, ticks })
}
