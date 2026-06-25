//! 해외지수분봉조회 — GET /uapi/overseas-price/v1/quotations/inquire-time-indexchartprice
//!
//! 스펙: .agent/specs/overseas_stock__quotations__inquire_time_indexchartprice.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/inquire-time-indexchartprice";
pub const TR_ID: &str = "FHKST03030200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_hour_cls_code: String,
    pub fid_pw_data_incu_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub ovrs_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub ovrs_nmix_prdy_clpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub ovrs_nmix_prpr: String,
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub ovrs_prod_oprc: String,
    #[serde(default)]
    pub ovrs_prod_hgpr: String,
    #[serde(default)]
    pub ovrs_prod_lwpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bar {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub optn_prpr: String,
    #[serde(default)]
    pub optn_oprc: String,
    #[serde(default)]
    pub optn_hgpr: String,
    #[serde(default)]
    pub optn_lwpr: String,
    #[serde(default)]
    pub cntg_vol: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub bars: Vec<Bar>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외지수분봉조회는 모의투자 미지원");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_HOUR_CLS_CODE", req.fid_hour_cls_code.as_str()),
        ("FID_PW_DATA_INCU_YN", req.fid_pw_data_incu_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let bars = resp
        .output2
        .map(serde_json::from_value::<Vec<Bar>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, bars })
}
