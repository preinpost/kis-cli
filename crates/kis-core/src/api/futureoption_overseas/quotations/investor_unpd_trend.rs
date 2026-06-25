//! 해외선물 미결제추이 — GET /uapi/overseas-futureoption/v1/quotations/investor-unpd-trend
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__investor_unpd_trend.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/investor-unpd-trend";
pub const TR_ID: &str = "HHDDB95030000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub prod_iscd: String,
    pub bsop_date: String,
    pub upmu_gubun: String,
    pub cts_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub row_cnt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub prod_iscd: String,
    #[serde(default)]
    pub cftc_iscd: String,
    #[serde(default)]
    pub bsop_date: String,
    #[serde(default)]
    pub bidp_spec: String,
    #[serde(default)]
    pub askp_spec: String,
    #[serde(default)]
    pub spread_spec: String,
    #[serde(default)]
    pub bidp_hedge: String,
    #[serde(default)]
    pub askp_hedge: String,
    #[serde(default)]
    pub hts_otst_smtn: String,
    #[serde(default)]
    pub bidp_missing: String,
    #[serde(default)]
    pub askp_missing: String,
    #[serde(default)]
    pub bidp_spec_cust: String,
    #[serde(default)]
    pub askp_spec_cust: String,
    #[serde(default)]
    pub spread_spec_cust: String,
    #[serde(default)]
    pub bidp_hedge_cust: String,
    #[serde(default)]
    pub askp_hedge_cust: String,
    #[serde(default)]
    pub cust_smtn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물 미결제추이는 모의투자 미지원 API입니다");
    }
    let params = [
        ("PROD_ISCD", req.prod_iscd.as_str()),
        ("BSOP_DATE", req.bsop_date.as_str()),
        ("UPMU_GUBUN", req.upmu_gubun.as_str()),
        ("CTS_KEY", req.cts_key.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let rows = resp
        .output2
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, rows })
}
