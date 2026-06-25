//! 국내주식 공매도 상위종목 — GET /uapi/domestic-stock/v1/ranking/short-sale
//!
//! 스펙: .agent/specs/domestic_stock__ranking__short_sale.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/short-sale";
pub const TR_ID: &str = "FHPST04820000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_aply_rang_vol: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_period_div_code: String,
    pub fid_input_cnt_1: String,
    pub fid_trgt_exls_cls_code: String,
    pub fid_trgt_cls_code: String,
    pub fid_aply_rang_prc_1: String,
    pub fid_aply_rang_prc_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
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
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub ssts_cntg_qty: String,
    #[serde(default)]
    pub ssts_vol_rlim: String,
    #[serde(default)]
    pub ssts_tr_pbmn: String,
    #[serde(default)]
    pub ssts_tr_pbmn_rlim: String,
    #[serde(default)]
    pub stnd_date1: String,
    #[serde(default)]
    pub stnd_date2: String,
    #[serde(default)]
    pub avrg_prc: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내주식 공매도 상위종목은 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_APLY_RANG_VOL", req.fid_aply_rang_vol.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_PERIOD_DIV_CODE", req.fid_period_div_code.as_str()),
        ("FID_INPUT_CNT_1", req.fid_input_cnt_1.as_str()),
        ("FID_TRGT_EXLS_CLS_CODE", req.fid_trgt_exls_cls_code.as_str()),
        ("FID_TRGT_CLS_CODE", req.fid_trgt_cls_code.as_str()),
        ("FID_APLY_RANG_PRC_1", req.fid_aply_rang_prc_1.as_str()),
        ("FID_APLY_RANG_PRC_2", req.fid_aply_rang_prc_2.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
