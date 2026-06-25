//! 장내채권 주문체결내역 — GET /uapi/domestic-bond/v1/trading/inquire-daily-ccld
//!
//! 스펙: .agent/specs/bond__order_account__inquire_daily_ccld.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/trading/inquire-daily-ccld";
pub const TR_ID: &str = "CTSC8013R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub sll_buy_dvsn_cd: String,
    pub sort_sqn_dvsn: String,
    pub pdno: String,
    pub nccs_yn: String,
    pub ctx_area_nk200: String,
    pub ctx_area_fk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub tot_ord_qty: String,
    #[serde(default)]
    pub tot_ccld_qty_smtl: String,
    #[serde(default)]
    pub tot_bond_ccld_avg_unpr: String,
    #[serde(default)]
    pub tot_ccld_amt_smtl: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub ord_dvsn_name: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd_name: String,
    #[serde(default)]
    pub shtn_pdno: String,
    #[serde(default)]
    pub prdt_abrv_name: String,
    #[serde(default)]
    pub ord_qty: String,
    #[serde(default)]
    pub bond_ord_unpr: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub bond_avg_unpr: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub buy_dt: String,
    #[serde(default)]
    pub samt_mket_ptci_yn_name: String,
    #[serde(default)]
    pub sprx_psbl_yn_ifom: String,
    #[serde(default)]
    pub ord_mdia_dvsn_name: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub nccs_qty: String,
    #[serde(default)]
    pub ord_gno_brno: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub summaries: Vec<Summary>,
    pub orders: Vec<Order>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 주문체결내역은 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("SORT_SQN_DVSN", req.sort_sqn_dvsn.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("NCCS_YN", req.nccs_yn.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let summaries = resp
        .output1
        .map(serde_json::from_value::<Vec<Summary>>)
        .transpose()?
        .unwrap_or_default();
    let orders = resp
        .output2
        .map(serde_json::from_value::<Vec<Order>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { summaries, orders })
}
