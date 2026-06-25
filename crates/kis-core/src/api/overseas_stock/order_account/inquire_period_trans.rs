//! 해외주식 일별거래내역 — GET /uapi/overseas-stock/v1/trading/inquire-period-trans
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_period_trans.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-period-trans";
pub const TR_ID: &str = "CTOS4001R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub erlm_strt_dt: String,
    pub erlm_end_dt: String,
    pub ovrs_excg_cd: String,
    pub pdno: String,
    pub sll_buy_dvsn_cd: String,
    pub loan_dvsn_cd: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Trans {
    #[serde(default)]
    pub trad_dt: String,
    #[serde(default)]
    pub sttl_dt: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub ovrs_item_name: String,
    #[serde(default)]
    pub ccld_qty: String,
    #[serde(default)]
    pub amt_unit_ccld_qty: String,
    #[serde(default)]
    pub ft_ccld_unpr2: String,
    #[serde(default)]
    pub ovrs_stck_ccld_unpr: String,
    #[serde(default)]
    pub tr_frcr_amt2: String,
    #[serde(default)]
    pub tr_amt: String,
    #[serde(default)]
    pub frcr_excc_amt_1: String,
    #[serde(default)]
    pub wcrc_excc_amt: String,
    #[serde(default)]
    pub dmst_frcr_fee1: String,
    #[serde(default)]
    pub frcr_fee1: String,
    #[serde(default)]
    pub dmst_wcrc_fee: String,
    #[serde(default)]
    pub ovrs_wcrc_fee: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub std_pdno: String,
    #[serde(default)]
    pub erlm_exrt: String,
    #[serde(default)]
    pub loan_dvsn_cd: String,
    #[serde(default)]
    pub loan_dvsn_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub frcr_buy_amt_smtl: String,
    #[serde(default)]
    pub frcr_sll_amt_smtl: String,
    #[serde(default)]
    pub dmst_fee_smtl: String,
    #[serde(default)]
    pub ovrs_fee_smtl: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Trans>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 일별거래내역은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("ERLM_STRT_DT", req.erlm_strt_dt.as_str()),
        ("ERLM_END_DT", req.erlm_end_dt.as_str()),
        ("OVRS_EXCG_CD", req.ovrs_excg_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("LOAN_DVSN_CD", req.loan_dvsn_cd.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output1
        .map(serde_json::from_value::<Vec<Trans>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { rows, summary })
}
