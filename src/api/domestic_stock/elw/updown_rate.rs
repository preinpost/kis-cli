//! ELW 상승률순위 — GET /uapi/elw/v1/ranking/updown-rate
//!
//! 스펙: .agent/specs/domestic_stock__elw__updown_rate.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/ranking/updown-rate";
pub const TR_ID: &str = "FHPEW02770000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// 20277
    pub fid_cond_scr_div_code: String,
    pub fid_unas_input_iscd: String,
    pub fid_input_iscd: String,
    pub fid_input_rmnn_dynu_1: String,
    /// 0 전체, 1 콜, 2 풋
    pub fid_div_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_input_vol_1: String,
    pub fid_input_vol_2: String,
    pub fid_input_date_1: String,
    /// 0 상승율, 1 하락율, 2 시가대비상승율, 3 시가대비하락율, 4 변동율
    pub fid_rank_sort_cls_code: String,
    pub fid_blng_cls_code: String,
    pub fid_input_date_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub stck_sdpr: String,
    #[serde(default)]
    pub sdpr_vrss_prpr_sign: String,
    #[serde(default)]
    pub sdpr_vrss_prpr: String,
    #[serde(default)]
    pub sdpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub oprc_vrss_prpr_sign: String,
    #[serde(default)]
    pub oprc_vrss_prpr: String,
    #[serde(default)]
    pub oprc_vrss_prpr_rate: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub prd_rsfl_sign: String,
    #[serde(default)]
    pub prd_rsfl: String,
    #[serde(default)]
    pub prd_rsfl_rate: String,
    #[serde(default)]
    pub stck_cnvr_rate: String,
    #[serde(default)]
    pub hts_rmnn_dynu: String,
    #[serde(default)]
    pub acpr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_UNAS_INPUT_ISCD", req.fid_unas_input_iscd.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_RMNN_DYNU_1", req.fid_input_rmnn_dynu_1.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_INPUT_PRICE_1", req.fid_input_price_1.as_str()),
        ("FID_INPUT_PRICE_2", req.fid_input_price_2.as_str()),
        ("FID_INPUT_VOL_1", req.fid_input_vol_1.as_str()),
        ("FID_INPUT_VOL_2", req.fid_input_vol_2.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
