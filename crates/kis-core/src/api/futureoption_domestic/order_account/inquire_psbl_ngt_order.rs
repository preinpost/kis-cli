//! (야간)선물옵션 주문가능 조회 — GET /uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_psbl_ngt_order.md
//! 모의투자 미지원. 신 TR_ID `STTN5105R` 사용.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-psbl-ngt-order";
pub const TR_ID: &str = "STTN5105R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub pdno: String,
    pub prdt_type_cd: String,
    pub sll_buy_dvsn_cd: String,
    pub unit_price: String,
    pub ord_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub max_ord_psbl_qty: String,
    #[serde(default)]
    pub tot_psbl_qty: String,
    #[serde(default)]
    pub lqd_psbl_qty: String,
    #[serde(default)]
    pub lqd_psbl_qty_1: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub bass_idx: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("(야간)선물옵션 주문가능 조회는 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("UNIT_PRICE", req.unit_price.as_str()),
        ("ORD_DVSN_CD", req.ord_dvsn_cd.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
