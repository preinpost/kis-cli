//! 해외선물옵션 당일주문내역조회 — GET /uapi/overseas-futureoption/v1/trading/inquire-ccld
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_ccld.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-ccld";
pub const TR_ID: &str = "OTFM3116R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ccld_nccs_dvsn: String,
    pub sll_buy_dvsn_cd: String,
    pub fuop_dvsn: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub orgn_ord_dt: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub ovrs_futr_fx_pdno: String,
    #[serde(default)]
    pub rcit_dvsn_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub trad_stgy_dvsn_cd: String,
    #[serde(default)]
    pub bass_pric_type_cd: String,
    #[serde(default)]
    pub ord_stat_cd: String,
    #[serde(default)]
    pub fm_ord_qty: String,
    #[serde(default)]
    pub fm_ord_pric: String,
    #[serde(default)]
    pub fm_stop_ord_pric: String,
    #[serde(default)]
    pub rsvn_dvsn: String,
    #[serde(default)]
    pub fm_ccld_qty: String,
    #[serde(default)]
    pub fm_ccld_pric: String,
    #[serde(default)]
    pub fm_ord_rmn_qty: String,
    #[serde(default)]
    pub ord_grp_name: String,
    #[serde(default)]
    pub erlm_dtl_dtime: String,
    #[serde(default)]
    pub ccld_dtl_dtime: String,
    #[serde(default)]
    pub ord_stfno: String,
    #[serde(default)]
    pub rmks1: String,
    #[serde(default)]
    pub new_lqd_dvsn_cd: String,
    #[serde(default)]
    pub fm_lqd_lmt_ord_pric: String,
    #[serde(default)]
    pub fm_lqd_stop_pric: String,
    #[serde(default)]
    pub ccld_cndt_cd: String,
    #[serde(default)]
    pub noti_vald_dt: String,
    #[serde(default)]
    pub acnt_type_cd: String,
    #[serde(default)]
    pub fuop_dvsn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub orders: Vec<Order>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 당일주문내역조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("CCLD_NCCS_DVSN", req.ccld_nccs_dvsn.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("FUOP_DVSN", req.fuop_dvsn.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let orders = resp
        .output
        .map(serde_json::from_value::<Vec<Order>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { orders })
}
