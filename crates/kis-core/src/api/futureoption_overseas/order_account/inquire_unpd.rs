//! 해외선물옵션 미결제내역조회(잔고) — GET /uapi/overseas-futureoption/v1/trading/inquire-unpd
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_unpd.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-unpd";
pub const TR_ID: &str = "OTFM1412R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub fuop_dvsn: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub ovrs_futr_fx_pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub fm_ustl_qty: String,
    #[serde(default)]
    pub fm_ccld_avg_pric: String,
    #[serde(default)]
    pub fm_now_pric: String,
    #[serde(default)]
    pub fm_evlu_pfls_amt: String,
    #[serde(default)]
    pub fm_opt_evlu_amt: String,
    #[serde(default)]
    pub fm_otp_evlu_pfls_amt: String,
    #[serde(default)]
    pub fuop_dvsn: String,
    #[serde(default)]
    pub ecis_rsvn_ord_yn: String,
    #[serde(default)]
    pub fm_lqd_psbl_qty: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 미결제내역조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("FUOP_DVSN", req.fuop_dvsn.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
