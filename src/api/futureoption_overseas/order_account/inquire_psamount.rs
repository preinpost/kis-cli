//! 해외선물옵션 주문가능조회 — GET /uapi/overseas-futureoption/v1/trading/inquire-psamount
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_psamount.md
//! 모의투자 미지원.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-psamount";
pub const TR_ID: &str = "OTFM3304R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ovrs_futr_fx_pdno: String,
    pub sll_buy_dvsn_cd: String,
    pub fm_ord_pric: String,
    pub ecis_rsvn_ord_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub ovrs_futr_fx_pdno: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub fm_ustl_qty: String,
    #[serde(default)]
    pub fm_lqd_psbl_qty: String,
    #[serde(default)]
    pub fm_new_ord_psbl_qty: String,
    #[serde(default)]
    pub fm_tot_ord_psbl_qty: String,
    #[serde(default)]
    pub fm_mkpr_tot_ord_psbl_qty: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 주문가능조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("OVRS_FUTR_FX_PDNO", req.ovrs_futr_fx_pdno.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("FM_ORD_PRIC", req.fm_ord_pric.as_str()),
        ("ECIS_RSVN_ORD_YN", req.ecis_rsvn_ord_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.context("응답에 output 없음")?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
