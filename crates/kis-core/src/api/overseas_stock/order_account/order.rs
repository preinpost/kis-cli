//! 해외주식 주문 — POST /uapi/overseas-stock/v1/trading/order
//!
//! 스펙: .agent/specs/overseas_stock__order_account__order.md
//! 시장×매매×실전/모의 조합으로 TR_ID 분기.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/order";

#[derive(Debug, Clone, Copy)]
pub enum Market {
    Usa,
    Japan,
    Shanghai,
    Hongkong,
    Shenzhen,
    Vietnam,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

fn tr_id(is_mock: bool, market: Market, side: Side) -> &'static str {
    use Market::*;
    use Side::*;
    match (is_mock, market, side) {
        (false, Usa, Buy) => "TTTT1002U",
        (false, Usa, Sell) => "TTTT1006U",
        (false, Japan, Buy) => "TTTS0308U",
        (false, Japan, Sell) => "TTTS0307U",
        (false, Shanghai, Buy) => "TTTS0202U",
        (false, Shanghai, Sell) => "TTTS1005U",
        (false, Hongkong, Buy) => "TTTS1002U",
        (false, Hongkong, Sell) => "TTTS1001U",
        (false, Shenzhen, Buy) => "TTTS0305U",
        (false, Shenzhen, Sell) => "TTTS0304U",
        (false, Vietnam, Buy) => "TTTS0311U",
        (false, Vietnam, Sell) => "TTTS0310U",
        (true, Usa, Buy) => "VTTT1002U",
        (true, Usa, Sell) => "VTTT1001U",
        (true, Japan, Buy) => "VTTS0308U",
        (true, Japan, Sell) => "VTTS0307U",
        (true, Shanghai, Buy) => "VTTS0202U",
        (true, Shanghai, Sell) => "VTTS1005U",
        (true, Hongkong, Buy) => "VTTS1002U",
        (true, Hongkong, Sell) => "VTTS1001U",
        (true, Shenzhen, Buy) => "VTTS0305U",
        (true, Shenzhen, Sell) => "VTTS0304U",
        (true, Vietnam, Buy) => "VTTS0311U",
        (true, Vietnam, Sell) => "VTTS0310U",
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

pub async fn call(client: &KisClient, market: Market, side: Side, req: &Request) -> Result<Response> {
    let tr = tr_id(client.is_mock(), market, side);
    let resp: ApiResponse = client.post_json(ENDPOINT, tr, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
