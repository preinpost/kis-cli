//! 국내주식 수익자산지표 순위 — GET /uapi/domestic-stock/v1/ranking/profit-asset-index
//!
//! 스펙: .agent/specs/domestic_stock__ranking__profit_asset_index.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/profit-asset-index";
pub const TR_ID: &str = "FHPST01730000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_trgt_cls_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_div_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_vol_cnt: String,
    pub fid_input_option_1: String,
    pub fid_input_option_2: String,
    pub fid_rank_sort_cls_code: String,
    pub fid_blng_cls_code: String,
    pub fid_trgt_exls_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub data_rank: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub sale_totl_prfi: String,
    #[serde(default)]
    pub bsop_prti: String,
    #[serde(default)]
    pub op_prfi: String,
    #[serde(default)]
    pub thtr_ntin: String,
    #[serde(default)]
    pub total_aset: String,
    #[serde(default)]
    pub total_lblt: String,
    #[serde(default)]
    pub total_cptl: String,
    #[serde(default)]
    pub stac_month: String,
    #[serde(default)]
    pub stac_month_cls_code: String,
    #[serde(default)]
    pub iqry_csnu: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내주식 수익자산지표 순위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_trgt_cls_code", req.fid_trgt_cls_code.as_str()),
        ("fid_cond_scr_div_code", req.fid_cond_scr_div_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_div_cls_code", req.fid_div_cls_code.as_str()),
        ("fid_input_price_1", req.fid_input_price_1.as_str()),
        ("fid_input_price_2", req.fid_input_price_2.as_str()),
        ("fid_vol_cnt", req.fid_vol_cnt.as_str()),
        ("fid_input_option_1", req.fid_input_option_1.as_str()),
        ("fid_input_option_2", req.fid_input_option_2.as_str()),
        ("fid_rank_sort_cls_code", req.fid_rank_sort_cls_code.as_str()),
        ("fid_blng_cls_code", req.fid_blng_cls_code.as_str()),
        ("fid_trgt_exls_cls_code", req.fid_trgt_exls_cls_code.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
