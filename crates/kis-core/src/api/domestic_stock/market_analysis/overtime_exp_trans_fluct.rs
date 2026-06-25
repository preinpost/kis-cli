//! 국내주식 시간외예상체결등락률 — GET /uapi/domestic-stock/v1/ranking/overtime-exp-trans-fluct

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/overtime-exp-trans-fluct";
pub const TR_ID: &str = "FHKST11860000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_rank_sort_cls_code: String,
    pub fid_div_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_input_vol_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub data_rank: String,
    #[serde(default)]
    pub iscd_stat_cls_code: String,
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub ovtm_untp_antc_cnpr: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_vrss: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_vrsssign: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_ctrt: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn1: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn1: String,
    #[serde(default)]
    pub ovtm_untp_antc_cnqn: String,
    #[serde(default)]
    pub itmt_vol: String,
    #[serde(default)]
    pub stck_prpr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_INPUT_PRICE_1", req.fid_input_price_1.as_str()),
        ("FID_INPUT_PRICE_2", req.fid_input_price_2.as_str()),
        ("FID_INPUT_VOL_1", req.fid_input_vol_1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
