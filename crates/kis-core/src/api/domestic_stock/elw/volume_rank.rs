//! ELW 거래량순위 — GET /uapi/elw/v1/ranking/volume-rank
//!
//! 스펙: .agent/specs/domestic_stock__elw__volume_rank.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/ranking/volume-rank";
pub const TR_ID: &str = "FHPEW02780000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// 20278
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
    /// 0 거래량, 1 평균거래증가율, 2 평균거래회전율, 3 거래금액, 4 순매수잔량, 5 순매도잔량
    pub fid_rank_sort_cls_code: String,
    pub fid_blng_cls_code: String,
    pub fid_input_iscd_2: String,
    pub fid_input_date_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub elw_kor_isnm: String,
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
    pub lstn_stcn: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub n_prdy_vol: String,
    #[serde(default)]
    pub n_prdy_vol_vrss: String,
    #[serde(default)]
    pub vol_inrt: String,
    #[serde(default)]
    pub vol_tnrt: String,
    #[serde(default)]
    pub nday_vol_tnrt: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub n_prdy_tr_pbmn: String,
    #[serde(default)]
    pub n_prdy_tr_pbmn_vrss: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub ntsl_rsqn: String,
    #[serde(default)]
    pub ntby_rsqn: String,
    #[serde(default)]
    pub seln_rsqn_rate: String,
    #[serde(default)]
    pub shnu_rsqn_rate: String,
    #[serde(default)]
    pub stck_cnvr_rate: String,
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
        ("FID_INPUT_ISCD_2", req.fid_input_iscd_2.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
