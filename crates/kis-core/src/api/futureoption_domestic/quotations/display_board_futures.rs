//! 국내옵션전광판_선물 — GET /uapi/domestic-futureoption/v1/quotations/display-board-futures
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__display_board_futures.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/display-board-futures";
pub const TR_ID: &str = "FHPIF05030200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_cond_mrkt_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub futs_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub futs_prpr: String,
    #[serde(default)]
    pub futs_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub futs_prdy_ctrt: String,
    #[serde(default)]
    pub hts_thpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub futs_askp: String,
    #[serde(default)]
    pub futs_bidp: String,
    #[serde(default)]
    pub hts_otst_stpl_qty: String,
    #[serde(default)]
    pub futs_hgpr: String,
    #[serde(default)]
    pub futs_lwpr: String,
    #[serde(default)]
    pub hts_rmnn_dynu: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub futs_antc_cnpr: String,
    #[serde(default)]
    pub futs_antc_cntg_vrss: String,
    #[serde(default)]
    pub antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub antc_cntg_prdy_ctrt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내옵션전광판_선물은 모의투자 미지원");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_COND_MRKT_CLS_CODE", req.fid_cond_mrkt_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output1
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(rows)
}
