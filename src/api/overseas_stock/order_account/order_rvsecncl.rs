//! 해외주식 정정취소주문 — POST /uapi/overseas-stock/v1/trading/order-rvsecncl
//!
//! 스펙: .agent/specs/overseas_stock__order_account__order_rvsecncl.md
//! 시장×실전/모의 조합으로 TR_ID 분기.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/order-rvsecncl";

#[derive(Debug, Clone, Copy)]
pub enum Market {
    Usa,
    Hongkong,
    Japan,
    Shanghai,
    Shenzhen,
    Vietnam,
}

fn tr_id(is_mock: bool, market: Market) -> &'static str {
    use Market::*;
    match (is_mock, market) {
        (false, Usa) => "TTTT1004U",
        (false, Hongkong) => "TTTS1003U",
        (false, Japan) => "TTTS0309U",
        (false, Shanghai) => "TTTS0302U",
        (false, Shenzhen) => "TTTS0306U",
        (false, Vietnam) => "TTTS0312U",
        (true, Usa) => "VTTT1004U",
        (true, Hongkong) => "VTTS1003U",
        (true, Japan) => "VTTS0309U",
        (true, Shanghai) => "VTTS0302U",
        (true, Shenzhen) => "VTTS0306U",
        (true, Vietnam) => "VTTS0312U",
    }
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
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
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

pub async fn call(client: &KisClient, market: Market, req: &Request) -> Result<Response> {
    let tr = tr_id(client.is_mock(), market);
    let resp: ApiResponse = client.post_json(ENDPOINT, tr, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
