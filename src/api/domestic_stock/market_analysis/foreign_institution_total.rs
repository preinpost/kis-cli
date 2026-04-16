//! 국내기관_외국인 매매종목가집계 — GET /uapi/domestic-stock/v1/quotations/foreign-institution-total

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/foreign-institution-total";
pub const TR_ID: &str = "FHPTJ04400000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_div_cls_code: String,
    pub fid_rank_sort_cls_code: String,
    pub fid_etc_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    #[serde(default)]
    pub ntby_qty: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub frgn_ntby_qty: String,
    #[serde(default)]
    pub orgn_ntby_qty: String,
    #[serde(default)]
    pub ivtr_ntby_qty: String,
    #[serde(default)]
    pub bank_ntby_qty: String,
    #[serde(default)]
    pub insu_ntby_qty: String,
    #[serde(default)]
    pub mrbn_ntby_qty: String,
    #[serde(default)]
    pub fund_ntby_qty: String,
    #[serde(default)]
    pub etc_orgt_ntby_vol: String,
    #[serde(default)]
    pub etc_corp_ntby_vol: String,
    #[serde(default)]
    pub frgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub orgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_ntby_tr_pbmn: String,
    #[serde(default)]
    pub bank_ntby_tr_pbmn: String,
    #[serde(default)]
    pub insu_ntby_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub fund_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_ntby_tr_pbmn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_ETC_CLS_CODE", req.fid_etc_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
