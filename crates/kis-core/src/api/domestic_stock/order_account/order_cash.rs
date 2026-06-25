//! 주식주문(현금) — POST /uapi/domestic-stock/v1/trading/order-cash
//!
//! 스펙: .agent/specs/domestic_stock__order_account__order_cash.md
//!
//! TR_ID가 매수/매도 × 실전/모의 = 4가지이므로 `Side` enum으로 분기.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/order-cash";
pub const TR_ID_REAL_BUY: &str = "TTTC0012U";
pub const TR_ID_REAL_SELL: &str = "TTTC0011U";
pub const TR_ID_MOCK_BUY: &str = "VTTC0012U";
pub const TR_ID_MOCK_SELL: &str = "VTTC0011U";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도주문 시 (01@일반매도, 02@임의매매, 05@대차매도)
    #[serde(rename = "SLL_TYPE", skip_serializing_if = "Option::is_none")]
    pub sll_type: Option<String>,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 시장가는 "0"
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// 스탑지정가(ORD_DVSN=22)일 때만 필수
    #[serde(rename = "CNDT_PRIC", skip_serializing_if = "Option::is_none")]
    pub cndt_pric: Option<String>,
    /// KRX/NXT/SOR — 미입력 시 KRX
    #[serde(rename = "EXCG_ID_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub excg_id_dvsn_cd: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    #[serde(default, rename = "ODNO")]
    pub odno: String,
    #[serde(default, rename = "ORD_TMD")]
    pub ord_tmd: String,
}

pub async fn call(client: &KisClient, side: Side, req: &Request) -> Result<Response> {
    let tr_id = match (client.is_mock(), side) {
        (false, Side::Buy) => TR_ID_REAL_BUY,
        (false, Side::Sell) => TR_ID_REAL_SELL,
        (true, Side::Buy) => TR_ID_MOCK_BUY,
        (true, Side::Sell) => TR_ID_MOCK_SELL,
    };
    let resp = client.post_json(ENDPOINT, tr_id, req, &[]).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
