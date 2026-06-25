//! 해외선물옵션 정정취소주문 — POST /uapi/overseas-futureoption/v1/trading/order-rvsecncl
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__order_rvsecncl.md
//! 실전: 정정 OTFM3002U / 취소 OTFM3003U. 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/order-rvsecncl";
pub const TR_ID_REVISE: &str = "OTFM3002U";
pub const TR_ID_CANCEL: &str = "OTFM3003U";

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Revise,
    Cancel,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "ORGN_ORD_DT")]
    pub orgn_ord_dt: String,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    #[serde(rename = "FM_HDGE_ORD_SCRN_YN")]
    pub fm_hdge_ord_scrn_yn: String,
    // Required=N (정정 전용): FM_LIMIT_ORD_PRIC, FM_STOP_ORD_PRIC,
    //   FM_LQD_LMT_ORD_PRIC, FM_LQD_STOP_ORD_PRIC.
    // Required=N (취소 전용): FM_MKPR_CVSN_YN.
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_LIMIT_ORD_PRIC")]
    pub fm_limit_ord_pric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_STOP_ORD_PRIC")]
    pub fm_stop_ord_pric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_LQD_LMT_ORD_PRIC")]
    pub fm_lqd_lmt_ord_pric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_LQD_STOP_ORD_PRIC")]
    pub fm_lqd_stop_ord_pric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "FM_MKPR_CVSN_YN")]
    pub fm_mkpr_cvsn_yn: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "ORD_DT")]
    pub ord_dt: String,
    #[serde(default, rename = "ODNO")]
    pub odno: String,
}

pub async fn call(client: &KisClient, action: Action, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 정정취소주문은 모의투자 미지원 API입니다");
    }
    let tr_id = match action {
        Action::Revise => TR_ID_REVISE,
        Action::Cancel => TR_ID_CANCEL,
    };
    let resp: ApiResponse = client.post_json(ENDPOINT, tr_id, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
