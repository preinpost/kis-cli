//! 채권정정취소가능주문조회 — GET /uapi/domestic-bond/v1/trading/inquire-psbl-rvsecncl
//!
//! 스펙: .agent/specs/bond__order_account__inquire_psbl_rvsecncl.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/trading/inquire-psbl-rvsecncl";
pub const TR_ID: &str = "CTSC8035R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ord_dt: String,
    pub odno: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub rvse_cncl_dvsn_name: String,
    #[serde(default)]
    pub ord_qty: String,
    #[serde(default)]
    pub bond_ord_unpr: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub ord_dvsn_cd: String,
    #[serde(default)]
    pub mgco_aptm_odno: String,
    #[serde(default)]
    pub samt_mket_ptci_yn: String,
    #[serde(default)]
    pub prdt_abrv_name: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("채권정정취소가능주문조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("ORD_DT", req.ord_dt.as_str()),
        ("ODNO", req.odno.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
