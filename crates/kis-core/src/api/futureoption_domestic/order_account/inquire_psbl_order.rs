//! 선물옵션 주문가능 — GET /uapi/domestic-futureoption/v1/trading/inquire-psbl-order
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_psbl_order.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-psbl-order";
pub const TR_ID_REAL: &str = "TTTO5105R";
pub const TR_ID_MOCK: &str = "VTTO5105R";

// 주의: 모든 Query 파라미터가 Required=N. 실제 호출 시 빈값 가능.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub pdno: String,
    pub sll_buy_dvsn_cd: String,
    pub unit_price: String,
    pub ord_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub tot_psbl_qty: String,
    #[serde(default)]
    pub lqd_psbl_qty1: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub bass_idx: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let tr_id = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("UNIT_PRICE", req.unit_price.as_str()),
        ("ORD_DVSN_CD", req.ord_dvsn_cd.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, tr_id, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
