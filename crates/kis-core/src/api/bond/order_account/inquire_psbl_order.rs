//! 장내채권 매수가능조회 — GET /uapi/domestic-bond/v1/trading/inquire-psbl-order
//!
//! 스펙: .agent/specs/bond__order_account__inquire_psbl_order.md
//! 모의투자 미지원. 매수가능수량 = 매수가능금액 / 채권주문단가2 * 10.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/trading/inquire-psbl-order";
pub const TR_ID: &str = "TTTC8910R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub pdno: String,
    pub bond_ord_unpr: String,
    pub samt_mket_ptci_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub ord_psbl_cash: String,
    #[serde(default)]
    pub ord_psbl_sbst: String,
    #[serde(default)]
    pub ruse_psbl_amt: String,
    #[serde(default)]
    pub bond_ord_unpr2: String,
    #[serde(default)]
    pub buy_psbl_amt: String,
    #[serde(default)]
    pub buy_psbl_qty: String,
    #[serde(default)]
    pub cma_evlu_amt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 매수가능조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("BOND_ORD_UNPR", req.bond_ord_unpr.as_str()),
        ("SAMT_MKET_PTCI_YN", req.samt_mket_ptci_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
