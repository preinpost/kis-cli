//! 금리 종합(국내채권/금리) — GET /uapi/domestic-stock/v1/quotations/comp-interest
//!
//! 스펙: .agent/specs/domestic_stock__sector__comp_interest.md
//!
//! output1(단일 object) + output2(Vec of rows).

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/comp-interest";
pub const TR_ID: &str = "FHPST07020000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// I
    pub fid_cond_mrkt_div_code: String,
    /// 20702
    pub fid_cond_scr_div_code: String,
    /// 1 해외금리지표
    pub fid_div_cls_code: String,
    /// 공백 전체
    pub fid_div_cls_code1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bcdt_code: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub bond_mnrt_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bond_mnrt_prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub stck_bsop_date: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row2 {
    #[serde(default)]
    pub bcdt_code: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub bond_mnrt_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bond_mnrt_prdy_vrss: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub stck_bsop_date: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    /// 스펙상 array로 명시되어 있음 — 국내 금리 지표 (단일 요소일 가능성)
    pub domestic: Vec<Row>,
    /// 해외 금리 지표
    pub foreign: Vec<Row2>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_DIV_CLS_CODE1", req.fid_div_cls_code1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let domestic: Vec<Row> = match resp.output1 {
        Some(serde_json::Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect(),
        Some(v) => serde_json::from_value::<Row>(v).map(|r| vec![r]).unwrap_or_default(),
        None => vec![],
    };
    let foreign: Vec<Row2> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { domestic, foreign })
}
