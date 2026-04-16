//! 국내주식 시간외거래량순위 — GET /uapi/domestic-stock/v1/ranking/overtime-volume
//!
//! 스펙: .agent/specs/domestic_stock__ranking__overtime_volume.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/overtime-volume";
pub const TR_ID: &str = "FHPST02350000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_rank_sort_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_vol_cnt: String,
    pub fid_trgt_cls_code: String,
    pub fid_trgt_exls_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub ovtm_untp_exch_vol: String,
    #[serde(default)]
    pub ovtm_untp_exch_tr_pbmn: String,
    #[serde(default)]
    pub ovtm_untp_kosdaq_vol: String,
    #[serde(default)]
    pub ovtm_untp_kosdaq_tr_pbmn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub ovtm_untp_prpr: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss_sign: String,
    #[serde(default)]
    pub ovtm_untp_prdy_ctrt: String,
    #[serde(default)]
    pub ovtm_untp_seln_rsqn: String,
    #[serde(default)]
    pub ovtm_untp_shnu_rsqn: String,
    #[serde(default)]
    pub ovtm_untp_vol: String,
    #[serde(default)]
    pub ovtm_vrss_acml_vol_rlim: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub bidp: String,
    #[serde(default)]
    pub askp: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내주식 시간외거래량순위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_INPUT_PRICE_1", req.fid_input_price_1.as_str()),
        ("FID_INPUT_PRICE_2", req.fid_input_price_2.as_str()),
        ("FID_VOL_CNT", req.fid_vol_cnt.as_str()),
        ("FID_TRGT_CLS_CODE", req.fid_trgt_cls_code.as_str()),
        ("FID_TRGT_EXLS_CLS_CODE", req.fid_trgt_exls_cls_code.as_str()),
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
