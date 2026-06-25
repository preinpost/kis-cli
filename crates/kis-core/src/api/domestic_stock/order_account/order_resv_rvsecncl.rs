//! 주식예약주문정정취소 — POST /uapi/domestic-stock/v1/trading/order-resv-rvsecncl
//!
//! 스펙: .agent/specs/domestic_stock__order_account__order_resv_rvsecncl.md
//!
//! 모의투자 미지원. 정정/취소 TR_ID 2종 (`Action` enum).
//! 취소 시 PDNO/ORD_QTY/ORD_UNPR 등 [정정] 태그 필드는 무시돼도 전송은 필수.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/order-resv-rvsecncl";
pub const TR_ID_CANCEL: &str = "CTSC0009U";
pub const TR_ID_MODIFY: &str = "CTSC0013U";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Cancel,
    Modify,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    #[serde(rename = "LOAN_DT", skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    #[serde(rename = "RSVN_ORD_END_DT", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_end_dt: Option<String>,
    #[serde(rename = "CTAL_TLNO", skip_serializing_if = "Option::is_none")]
    pub ctal_tlno: Option<String>,
    #[serde(rename = "RSVN_ORD_SEQ")]
    pub rsvn_ord_seq: String,
    #[serde(rename = "RSVN_ORD_ORGNO", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_orgno: Option<String>,
    #[serde(rename = "RSVN_ORD_ORD_DT", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_ord_dt: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub nrml_prcs_yn: String,
}

pub async fn call(client: &KisClient, action: Action, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("주식예약주문정정취소는 모의투자 미지원 API입니다");
    }
    let tr_id = match action {
        Action::Cancel => TR_ID_CANCEL,
        Action::Modify => TR_ID_MODIFY,
    };
    let resp = client.post_json(ENDPOINT, tr_id, req, &[]).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = match output {
        serde_json::Value::Array(mut arr) => {
            let first = arr.pop().ok_or_else(|| anyhow!("output 배열이 비어있음"))?;
            serde_json::from_value(first)?
        }
        v => serde_json::from_value(v)?,
    };
    Ok(parsed)
}
