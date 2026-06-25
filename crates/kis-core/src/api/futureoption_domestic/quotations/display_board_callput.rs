//! 국내옵션전광판_콜풋 — GET /uapi/domestic-futureoption/v1/quotations/display-board-callput
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__display_board_callput.md
//! 모의투자 미지원. output1: 콜옵션, output2: 풋옵션.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/display-board-callput";
pub const TR_ID: &str = "FHPIF05030100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_mrkt_cls_code: String,
    pub fid_mtrt_cnt: String,
    pub fid_cond_mrkt_cls_code: String,
    pub fid_mrkt_cls_code1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OptionRow {
    #[serde(default)]
    pub acpr: String,
    #[serde(default)]
    pub unch_prpr: String,
    #[serde(default)]
    pub optn_shrn_iscd: String,
    #[serde(default)]
    pub optn_prpr: String,
    #[serde(default)]
    pub optn_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub optn_prdy_ctrt: String,
    #[serde(default)]
    pub optn_bidp: String,
    #[serde(default)]
    pub optn_askp: String,
    #[serde(default)]
    pub tmvl_val: String,
    #[serde(default)]
    pub nmix_sdpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub seln_rsqn: String,
    #[serde(default)]
    pub shnu_rsqn: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub hts_otst_stpl_qty: String,
    #[serde(default)]
    pub otst_stpl_qty_icdc: String,
    #[serde(default)]
    pub delta_val: String,
    #[serde(default)]
    pub gama: String,
    #[serde(default)]
    pub vega: String,
    #[serde(default)]
    pub theta: String,
    #[serde(default)]
    pub rho: String,
    #[serde(default)]
    pub hts_ints_vltl: String,
    #[serde(default)]
    pub invl_val: String,
    #[serde(default)]
    pub esdg: String,
    #[serde(default)]
    pub dprt: String,
    #[serde(default)]
    pub hist_vltl: String,
    #[serde(default)]
    pub hts_thpr: String,
    #[serde(default)]
    pub optn_oprc: String,
    #[serde(default)]
    pub optn_hgpr: String,
    #[serde(default)]
    pub optn_lwpr: String,
    #[serde(default)]
    pub optn_mxpr: String,
    #[serde(default)]
    pub optn_llam: String,
    #[serde(default)]
    pub atm_cls_name: String,
    #[serde(default)]
    pub rgbf_vrss_icdc: String,
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

#[derive(Debug, Clone)]
pub struct Response {
    pub calls: Vec<OptionRow>,
    pub puts: Vec<OptionRow>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내옵션전광판_콜풋은 모의투자 미지원");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_MTRT_CNT", req.fid_mtrt_cnt.as_str()),
        ("FID_COND_MRKT_CLS_CODE", req.fid_cond_mrkt_cls_code.as_str()),
        ("FID_MRKT_CLS_CODE1", req.fid_mrkt_cls_code1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let calls = resp
        .output1
        .map(serde_json::from_value::<Vec<OptionRow>>)
        .transpose()?
        .unwrap_or_default();
    let puts = resp
        .output2
        .map(serde_json::from_value::<Vec<OptionRow>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { calls, puts })
}
