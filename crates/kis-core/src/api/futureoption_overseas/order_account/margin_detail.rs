//! 해외선물옵션 증거금상세 — GET /uapi/overseas-futureoption/v1/trading/margin-detail
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__margin_detail.md
//! 모의투자 미지원.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/margin-detail";
pub const TR_ID: &str = "OTFM3115R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub crcy_cd: String,
    pub inqr_dt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub resp_dt: String,
    #[serde(default)]
    pub acnt_net_risk_mgna_aply_yn: String,
    #[serde(default)]
    pub fm_ord_psbl_amt: String,
    #[serde(default)]
    pub fm_add_mgn_amt: String,
    #[serde(default)]
    pub fm_brkg_mgn_amt: String,
    #[serde(default)]
    pub fm_excc_brkg_mgn_amt: String,
    #[serde(default)]
    pub fm_ustl_mgn_amt: String,
    #[serde(default)]
    pub fm_mntn_mgn_amt: String,
    #[serde(default)]
    pub fm_ord_mgn_amt: String,
    #[serde(default)]
    pub fm_futr_ord_mgn_amt: String,
    #[serde(default)]
    pub fm_opt_buy_ord_amt: String,
    #[serde(default)]
    pub fm_opt_sll_ord_mgn_amt: String,
    #[serde(default)]
    pub fm_opt_buy_ord_mgn_amt: String,
    #[serde(default)]
    pub fm_ecis_rsvn_mgn_amt: String,
    #[serde(default)]
    pub fm_span_brkg_mgn_amt: String,
    #[serde(default)]
    pub fm_span_pric_altr_mgn_amt: String,
    #[serde(default)]
    pub fm_span_term_sprd_mgn_amt: String,
    #[serde(default)]
    pub fm_span_buy_opt_min_mgn_amt: String,
    #[serde(default)]
    pub fm_span_opt_min_mgn_amt: String,
    #[serde(default)]
    pub fm_span_tot_risk_mgn_amt: String,
    #[serde(default)]
    pub fm_span_mntn_mgn_amt: String,
    #[serde(default)]
    pub fm_span_mntn_pric_altr_mgn_amt: String,
    #[serde(default)]
    pub fm_span_mntn_term_sprd_mgn_amt: String,
    #[serde(default)]
    pub fm_span_mntn_opt_pric_mgn_amt: String,
    #[serde(default)]
    pub fm_span_mntn_opt_min_mgn_amt: String,
    #[serde(default)]
    pub fm_span_mntn_tot_risk_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_brkg_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_pric_altr_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_term_sprd_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_opt_pric_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_buy_opt_min_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_tot_risk_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_mntn_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_mntn_pric_altr_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_mntn_term_sprd_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_mntn_opt_pric_mgn_amt: String,
    #[serde(default)]
    pub fm_eurx_mntn_tot_risk_mgn_amt: String,
    #[serde(default)]
    pub fm_gnrl_brkg_mgn_amt: String,
    #[serde(default)]
    pub fm_futr_ustl_mgn_amt: String,
    #[serde(default)]
    pub fm_sll_opt_ustl_mgn_amt: String,
    #[serde(default)]
    pub fm_buy_opt_ustl_mgn_amt: String,
    #[serde(default)]
    pub fm_sprd_ustl_mgn_amt: String,
    #[serde(default)]
    pub fm_avg_dsct_mgn_amt: String,
    #[serde(default)]
    pub fm_gnrl_mntn_mgn_amt: String,
    #[serde(default)]
    pub fm_futr_mntn_mgn_amt: String,
    #[serde(default)]
    pub fm_opt_mntn_mgn_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 증거금상세는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("CRCY_CD", req.crcy_cd.as_str()),
        ("INQR_DT", req.inqr_dt.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.context("응답에 output 없음")?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
