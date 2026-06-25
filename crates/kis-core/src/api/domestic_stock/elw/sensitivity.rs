//! ELW 민감도 순위 — GET /uapi/elw/v1/ranking/sensitivity
//!
//! 스펙: .agent/specs/domestic_stock__elw__sensitivity.md
//!
//! 모의투자 미지원. 델타/감마/세타/베가/로 그릭스 순위.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/ranking/sensitivity";
pub const TR_ID: &str = "FHPEW02850000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// W
    pub fid_cond_mrkt_div_code: String,
    /// Unique 20285
    pub fid_cond_scr_div_code: String,
    pub fid_unas_input_iscd: String,
    pub fid_input_iscd: String,
    /// 0 전체, 1 콜, 2 풋
    pub fid_div_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_input_vol_1: String,
    pub fid_input_vol_2: String,
    /// 0 이론가, 1 델타, 2 감마, 3 세타(로), 4 베가, 5 로, 6 내재변동성, 7 90일변동성
    pub fid_rank_sort_cls_code: String,
    pub fid_input_rmnn_dynu_1: String,
    pub fid_input_date_1: String,
    /// 0 전체, 1 일반, 2 조기종료
    pub fid_blng_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub elw_kor_isnm: String,
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
    pub hts_thpr: String,
    #[serde(default)]
    pub delta_val: String,
    #[serde(default)]
    pub gama: String,
    #[serde(default)]
    pub theta: String,
    #[serde(default)]
    pub vega: String,
    #[serde(default)]
    pub rho: String,
    #[serde(default)]
    pub hts_ints_vltl: String,
    #[serde(default)]
    pub d90_hist_vltl: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 민감도 순위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_UNAS_INPUT_ISCD", req.fid_unas_input_iscd.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_INPUT_PRICE_1", req.fid_input_price_1.as_str()),
        ("FID_INPUT_PRICE_2", req.fid_input_price_2.as_str()),
        ("FID_INPUT_VOL_1", req.fid_input_vol_1.as_str()),
        ("FID_INPUT_VOL_2", req.fid_input_vol_2.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_INPUT_RMNN_DYNU_1", req.fid_input_rmnn_dynu_1.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
