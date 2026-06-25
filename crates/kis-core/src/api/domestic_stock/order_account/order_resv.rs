//! 주식예약주문 — POST /uapi/domestic-stock/v1/trading/order-resv
//!
//! 스펙: .agent/specs/domestic_stock__order_account__order_resv.md
//!
//! 모의투자 미지원. 매수/매도는 `SLL_BUY_DVSN_CD` 필드(01 매도, 02 매수)로 구분.
//! 예약주문 가능시간: 15:40 ~ 다음 영업일 7:30.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/order-resv";
pub const TR_ID: &str = "CTSC0008U";

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
    /// 시장가는 "0"
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 01 매도, 02 매수
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    /// 00 지정가, 01 시장가, 02 조건부지정가, 05 장전 시간외
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
    /// 10 현금 / 12 주식담보대출 / 14 대여상환 / 21~28 신용
    #[serde(rename = "ORD_OBJT_CBLC_DVSN_CD")]
    pub ord_objt_cblc_dvsn_cd: String,
    #[serde(rename = "LOAN_DT", skip_serializing_if = "Option::is_none")]
    pub loan_dt: Option<String>,
    /// 미입력 시 일반예약(다음 영업일), 입력 시 기간예약(최대 30일)
    #[serde(rename = "RSVN_ORD_END_DT", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_end_dt: Option<String>,
    #[serde(rename = "LDNG_DT", skip_serializing_if = "Option::is_none")]
    pub ldng_dt: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub rsvn_ord_seq: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("주식예약주문은 모의투자 미지원 API입니다");
    }
    let resp = client.post_json(ENDPOINT, TR_ID, req, &[]).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    // output이 array일 수 있으나 단일 요소만 사용
    let parsed: Response = match output {
        serde_json::Value::Array(mut arr) => {
            let first = arr.pop().ok_or_else(|| anyhow!("output 배열이 비어있음"))?;
            serde_json::from_value(first)?
        }
        v => serde_json::from_value(v)?,
    };
    Ok(parsed)
}
