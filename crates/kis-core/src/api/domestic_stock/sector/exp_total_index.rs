//! 국내주식 예상체결 전체지수 — GET /uapi/domestic-stock/v1/quotations/exp-total-index
//!
//! 스펙: .agent/specs/domestic_stock__sector__exp_total_index.md
//!
//! output1(지수 요약) + output2(업종별 Vec).

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/exp-total-index";
pub const TR_ID: &str = "FHKUP11750000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 0 전체, K 거래소, Q 코스닥
    pub fid_mrkt_cls_code: String,
    pub fid_cond_mrkt_div_code: String,
    /// 11175
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    /// 1 장시작전, 2 장마감
    pub fid_mkop_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub ascn_issu_cnt: String,
    #[serde(default)]
    pub down_issu_cnt: String,
    #[serde(default)]
    pub stnr_issu_cnt: String,
    #[serde(default)]
    pub bstp_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub nmix_sdpr: String,
    #[serde(default)]
    pub ascn_issu_cnt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("fid_mrkt_cls_code", req.fid_mrkt_cls_code.as_str()),
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_cond_scr_div_code", req.fid_cond_scr_div_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_mkop_cls_code", req.fid_mkop_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let rows: Vec<Row> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, rows })
}
