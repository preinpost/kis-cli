//! 해외선물옵션 기간계좌손익 일별 — GET /uapi/overseas-futureoption/v1/trading/inquire-period-ccld
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_period_ccld.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-period-ccld";
pub const TR_ID: &str = "OTFM3118R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub inqr_term_from_dt: String,
    pub inqr_term_to_dt: String,
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub crcy_cd: String,
    pub whol_trsl_yn: String,
    pub fuop_dvsn: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub fm_buy_qty: String,
    #[serde(default)]
    pub fm_sll_qty: String,
    #[serde(default)]
    pub fm_lqd_pfls_amt: String,
    #[serde(default)]
    pub fm_fee: String,
    #[serde(default)]
    pub fm_net_pfls_amt: String,
    #[serde(default)]
    pub fm_ustl_buy_qty: String,
    #[serde(default)]
    pub fm_ustl_sll_qty: String,
    #[serde(default)]
    pub fm_ustl_evlu_pfls_amt: String,
    #[serde(default)]
    pub fm_ustl_evlu_pfls_amt2: String,
    #[serde(default)]
    pub fm_ustl_evlu_pfls_icdc_amt: String,
    #[serde(default)]
    pub fm_ustl_agrm_amt: String,
    #[serde(default)]
    pub fm_opt_lqd_amt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Detail {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub ovrs_futr_fx_pdno: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub fm_buy_qty: String,
    #[serde(default)]
    pub fm_sll_qty: String,
    #[serde(default)]
    pub fm_lqd_pfls_amt: String,
    #[serde(default)]
    pub fm_fee: String,
    #[serde(default)]
    pub fm_net_pfls_amt: String,
    #[serde(default)]
    pub fm_ustl_buy_qty: String,
    #[serde(default)]
    pub fm_ustl_sll_qty: String,
    #[serde(default)]
    pub fm_ustl_evlu_pfls_amt: String,
    #[serde(default)]
    pub fm_ustl_evlu_pfls_amt2: String,
    #[serde(default)]
    pub fm_ustl_evlu_pfls_icdc_amt: String,
    #[serde(default)]
    pub fm_ccld_avg_pric: String,
    #[serde(default)]
    pub fm_ustl_agrm_amt: String,
    #[serde(default)]
    pub fm_opt_lqd_amt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub summaries: Vec<Summary>,
    pub details: Vec<Detail>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 기간계좌손익은 모의투자 미지원 API입니다");
    }
    let params = [
        ("INQR_TERM_FROM_DT", req.inqr_term_from_dt.as_str()),
        ("INQR_TERM_TO_DT", req.inqr_term_to_dt.as_str()),
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("CRCY_CD", req.crcy_cd.as_str()),
        ("WHOL_TRSL_YN", req.whol_trsl_yn.as_str()),
        ("FUOP_DVSN", req.fuop_dvsn.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let summaries = resp
        .output1
        .map(serde_json::from_value::<Vec<Summary>>)
        .transpose()?
        .unwrap_or_default();
    let details = resp
        .output2
        .map(serde_json::from_value::<Vec<Detail>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { summaries, details })
}
