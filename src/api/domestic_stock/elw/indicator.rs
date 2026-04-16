//! ELW 지표순위 — GET /uapi/elw/v1/ranking/indicator
//!
//! 스펙: .agent/specs/domestic_stock__elw__indicator.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/ranking/indicator";
pub const TR_ID: &str = "FHPEW02790000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// 20279
    pub fid_cond_scr_div_code: String,
    pub fid_unas_input_iscd: String,
    pub fid_input_iscd: String,
    /// 0 전체, 1 콜, 2 풋
    pub fid_div_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_input_vol_1: String,
    pub fid_input_vol_2: String,
    /// 0 전환비율, 1 레버리지, 2 행사가, 3 내재가치, 4 시간가치
    pub fid_rank_sort_cls_code: String,
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
    pub stck_cnvr_rate: String,
    #[serde(default)]
    pub lvrg_val: String,
    #[serde(default)]
    pub acpr: String,
    #[serde(default)]
    pub tmvl_val: String,
    #[serde(default)]
    pub invl_val: String,
    #[serde(default)]
    pub elw_ko_barrier: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
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
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output1.ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
