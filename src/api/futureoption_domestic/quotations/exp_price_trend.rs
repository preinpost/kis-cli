//! 선물옵션 일중예상체결추이 — GET /uapi/domestic-futureoption/v1/quotations/exp-price-trend
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__exp_price_trend.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/exp-price-trend";
pub const TR_ID: &str = "FHPIF05110100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_input_iscd: String,
    pub fid_cond_mrkt_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub futs_antc_cnpr: String,
    #[serde(default)]
    pub antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub futs_antc_cntg_vrss: String,
    #[serde(default)]
    pub antc_cntg_prdy_ctrt: String,
    #[serde(default)]
    pub futs_sdpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tick {
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub futs_antc_cnpr: String,
    #[serde(default)]
    pub antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub futs_antc_cntg_vrss: String,
    #[serde(default)]
    pub antc_cntg_prdy_ctrt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub ticks: Vec<Tick>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("선물옵션 일중예상체결추이는 모의투자 미지원");
    }
    let params = [
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
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
