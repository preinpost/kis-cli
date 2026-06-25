//! 해외주식 미국주간주문 — POST /uapi/overseas-stock/v1/trading/daytime-order
//!
//! 스펙: .agent/specs/overseas_stock__order_account__daytime_order.md
//! 모의투자 미지원. 주간매수 TTTS6036U / 주간매도 TTTS6037U.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/daytime-order";
pub const TR_ID_BUY: &str = "TTTS6036U";
pub const TR_ID_SELL: &str = "TTTS6037U";

#[derive(Debug, Clone, Copy)]
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
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
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
    if client.is_mock() {
        bail!("해외주식 미국주간주문은 모의투자 미지원");
    }
    let tr = match side {
        Side::Buy => TR_ID_BUY,
        Side::Sell => TR_ID_SELL,
    };
    let resp: ApiResponse = client.post_json(ENDPOINT, tr, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
