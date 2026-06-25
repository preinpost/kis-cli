//! 해외주식 예약주문접수 — POST /uapi/overseas-stock/v1/trading/order-resv
//!
//! 스펙: .agent/specs/overseas_stock__order_account__order_resv.md
//! 미국 매수/매도와 아시아 통합 + 실전/모의 6개 TR_ID 분기.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/order-resv";

#[derive(Debug, Clone, Copy)]
pub enum Variant {
    UsBuy,
    UsSell,
    Asia,
}

fn tr_id(is_mock: bool, v: Variant) -> &'static str {
    use Variant::*;
    match (is_mock, v) {
        (false, UsBuy) => "TTTT3014U",
        (false, UsSell) => "TTTT3016U",
        (false, Asia) => "TTTS3013U",
        (true, UsBuy) => "VTTT3014U",
        (true, UsSell) => "VTTT3016U",
        (true, Asia) => "VTTS3013U",
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    #[serde(rename = "PRDT_TYPE_CD")]
    pub prdt_type_cd: String,
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    #[serde(rename = "FT_ORD_QTY")]
    pub ft_ord_qty: String,
    #[serde(rename = "FT_ORD_UNPR3")]
    pub ft_ord_unpr3: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "ODNO")]
    pub odno: String,
    #[serde(default, rename = "RSVN_ORD_RCIT_DT")]
    pub rsvn_ord_rcit_dt: String,
    #[serde(default, rename = "OVRS_RSVN_ODNO")]
    pub ovrs_rsvn_odno: String,
}

pub async fn call(client: &KisClient, variant: Variant, req: &Request) -> Result<Response> {
    let tr = tr_id(client.is_mock(), variant);
    let resp: ApiResponse = client.post_json(ENDPOINT, tr, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
