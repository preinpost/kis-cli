//! 국내업종 일자별지수 — GET /uapi/domestic-stock/v1/quotations/inquire-index-daily-price
//!
//! 스펙: .agent/specs/domestic_stock__sector__inquire_index_daily_price.md
//!
//! 모의투자 미지원. output1(메타) + output2(일자별 Vec).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-index-daily-price";
pub const TR_ID: &str = "FHPUP02120000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// D 일, W 주, M 월
    pub fid_period_div_code: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
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
    pub stck_bsop_date: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub bstp_nmix_oprc: String,
    #[serde(default)]
    pub bstp_nmix_hgpr: String,
    #[serde(default)]
    pub bstp_nmix_lwpr: String,
    #[serde(default)]
    pub acml_vol_rlim: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub invt_new_psdg: String,
    #[serde(default)]
    pub d20_dsrt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내업종 일자별지수는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_PERIOD_DIV_CODE", req.fid_period_div_code.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
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
