//! 해외주식 잔고 — GET /uapi/overseas-stock/v1/trading/inquire-balance
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_balance.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-balance";
pub const TR_ID_REAL: &str = "TTTS3012R";
pub const TR_ID_MOCK: &str = "VTTS3012R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ovrs_excg_cd: String,
    pub tr_crcy_cd: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub ovrs_pdno: String,
    #[serde(default)]
    pub ovrs_item_name: String,
    #[serde(default)]
    pub frcr_evlu_pfls_amt: String,
    #[serde(default)]
    pub evlu_pfls_rt: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
    #[serde(default)]
    pub ovrs_cblc_qty: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub frcr_pchs_amt1: String,
    #[serde(default)]
    pub ovrs_stck_evlu_amt: String,
    #[serde(default)]
    pub now_pric2: String,
    #[serde(default)]
    pub tr_crcy_cd: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
    #[serde(default)]
    pub loan_type_cd: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub expd_dt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub frcr_pchs_amt1: String,
    #[serde(default)]
    pub ovrs_rlzt_pfls_amt: String,
    #[serde(default)]
    pub ovrs_tot_pfls: String,
    #[serde(default)]
    pub rlzt_erng_rt: String,
    #[serde(default)]
    pub tot_evlu_pfls_amt: String,
    #[serde(default)]
    pub tot_pftrt: String,
    #[serde(default)]
    pub frcr_buy_amt_smtl1: String,
    #[serde(default)]
    pub ovrs_rlzt_pfls_amt2: String,
    #[serde(default)]
    pub frcr_buy_amt_smtl2: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let tr = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("OVRS_EXCG_CD", req.ovrs_excg_cd.as_str()),
        ("TR_CRCY_CD", req.tr_crcy_cd.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, tr, &params).await?;
    let holdings = resp
        .output1
        .map(serde_json::from_value::<Vec<Holding>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { holdings, summary })
}
