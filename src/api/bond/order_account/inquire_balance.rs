//! 장내채권 잔고조회 — GET /uapi/domestic-bond/v1/trading/inquire-balance
//!
//! 스펙: .agent/specs/bond__order_account__inquire_balance.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/trading/inquire-balance";
pub const TR_ID: &str = "CTSC8407R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub inqr_cndt: String,
    pub pdno: String,
    pub buy_dt: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub buy_dt: String,
    #[serde(default)]
    pub buy_sqno: String,
    #[serde(default)]
    pub cblc_qty: String,
    #[serde(default)]
    pub agrx_qty: String,
    #[serde(default)]
    pub sprx_qty: String,
    #[serde(default)]
    pub exdt: String,
    #[serde(default)]
    pub buy_erng_rt: String,
    #[serde(default)]
    pub buy_unpr: String,
    #[serde(default)]
    pub buy_amt: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 잔고조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_CNDT", req.inqr_cndt.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("BUY_DT", req.buy_dt.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let holdings = resp
        .output
        .map(serde_json::from_value::<Vec<Holding>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { holdings })
}
