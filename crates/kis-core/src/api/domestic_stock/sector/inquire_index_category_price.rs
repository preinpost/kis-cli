//! 국내업종 구분별전체시세 — GET /uapi/domestic-stock/v1/quotations/inquire-index-category-price
//!
//! 스펙: .agent/specs/domestic_stock__sector__inquire_index_category_price.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-index-category-price";
pub const TR_ID: &str = "FHPUP02140000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// U
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// 20214
    pub fid_cond_scr_div_code: String,
    /// K/Q/K2
    pub fid_mrkt_cls_code: String,
    /// 0/1/2/3 (시장별 의미 다름)
    pub fid_blng_cls_code: String,
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
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub bstp_nmix_oprc: String,
    #[serde(default)]
    pub bstp_nmix_hgpr: String,
    #[serde(default)]
    pub bstp_nmix_lwpr: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub ascn_issu_cnt: String,
    #[serde(default)]
    pub down_issu_cnt: String,
    #[serde(default)]
    pub stnr_issu_cnt: String,
    #[serde(default)]
    pub uplm_issu_cnt: String,
    #[serde(default)]
    pub lslm_issu_cnt: String,
    #[serde(default)]
    pub prdy_tr_pbmn: String,
    #[serde(default)]
    pub dryy_bstp_nmix_hgpr_date: String,
    #[serde(default)]
    pub dryy_bstp_nmix_hgpr: String,
    #[serde(default)]
    pub dryy_bstp_nmix_lwpr: String,
    #[serde(default)]
    pub dryy_bstp_nmix_lwpr_date: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bstp_cls_code: String,
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
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
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
