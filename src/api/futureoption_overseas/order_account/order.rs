//! 해외선물옵션 주문 — POST /uapi/overseas-futureoption/v1/trading/order
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__order.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/order";
pub const TR_ID: &str = "OTFM3001U";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "OVRS_FUTR_FX_PDNO")]
    pub ovrs_futr_fx_pdno: String,
    #[serde(rename = "SLL_BUY_DVSN_CD")]
    pub sll_buy_dvsn_cd: String,
    #[serde(rename = "PRIC_DVSN_CD")]
    pub pric_dvsn_cd: String,
    #[serde(rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: String,
    #[serde(rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: String,
    #[serde(rename = "FM_ORD_QTY")]
    pub fm_ord_qty: String,
    #[serde(rename = "CCLD_CNDT_CD")]
    pub ccld_cndt_cd: String,
    #[serde(rename = "CPLX_ORD_DVSN_CD")]
    pub cplx_ord_dvsn_cd: String,
    #[serde(rename = "ECIS_RSVN_ORD_YN")]
    pub ecis_rsvn_ord_yn: String,
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
    // Required=N (hedge청산용): FM_LQD_USTL_CCLD_DT, FM_LQD_USTL_CCNO,
    //   FM_LQD_LMT_ORD_PRIC, FM_LQD_STOP_ORD_PRIC — 빈 문자열 전송.
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_LQD_USTL_CCLD_DT")]
    pub fm_lqd_ustl_ccld_dt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_LQD_USTL_CCNO")]
    pub fm_lqd_ustl_ccno: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "ORD_DT")]
    pub ord_dt: String,
    #[serde(default, rename = "ODNO")]
    pub odno: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 주문은 모의투자 미지원 API입니다");
    }
    let resp: ApiResponse = client.post_json(ENDPOINT, TR_ID, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
