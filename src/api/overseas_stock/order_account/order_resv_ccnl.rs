//! 해외주식 예약주문접수취소 — POST /uapi/overseas-stock/v1/trading/order-resv-ccnl
//!
//! 스펙: .agent/specs/overseas_stock__order_account__order_resv_ccnl.md
//! 미국 예약주문 취소만 지원 (아시아 미제공).

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/order-resv-ccnl";
pub const TR_ID_REAL: &str = "TTTT3017U";
pub const TR_ID_MOCK: &str = "VTTT3017U";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "RSYN_ORD_RCIT_DT")]
    pub rsyn_ord_rcit_dt: String,
    #[serde(rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let tr = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let resp: ApiResponse = client.post_json(ENDPOINT, tr, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
