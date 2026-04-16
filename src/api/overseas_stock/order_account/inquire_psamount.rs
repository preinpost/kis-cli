//! 해외주식 매수가능금액조회 — GET /uapi/overseas-stock/v1/trading/inquire-psamount
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_psamount.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-psamount";
pub const TR_ID_REAL: &str = "TTTS3007R";
pub const TR_ID_MOCK: &str = "VTTS3007R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ovrs_excg_cd: String,
    pub ovrs_ord_unpr: String,
    pub item_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub tr_crcy_cd: String,
    #[serde(default)]
    pub ord_psbl_frcr_amt: String,
    #[serde(default)]
    pub sll_ruse_psbl_amt: String,
    #[serde(default)]
    pub ovrs_ord_psbl_amt: String,
    #[serde(default)]
    pub max_ord_psbl_qty: String,
    #[serde(default)]
    pub echm_af_ord_psbl_amt: String,
    #[serde(default)]
    pub echm_af_ord_psbl_qty: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub exrt: String,
    #[serde(default)]
    pub frcr_ord_psbl_amt1: String,
    #[serde(default)]
    pub ovrs_max_ord_psbl_qty: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Option<Response>> {
    let tr = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("OVRS_EXCG_CD", req.ovrs_excg_cd.as_str()),
        ("OVRS_ORD_UNPR", req.ovrs_ord_unpr.as_str()),
        ("ITEM_CD", req.item_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, tr, &params).await?;
    Ok(resp
        .output
        .and_then(|v| serde_json::from_value::<Response>(v).ok()))
}
