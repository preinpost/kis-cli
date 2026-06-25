//! 해외선물옵션 일별 주문내역 — GET /uapi/overseas-futureoption/v1/trading/inquire-daily-order
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_daily_order.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-daily-order";
pub const TR_ID: &str = "OTFM3120R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub strt_dt: String,
    pub end_dt: String,
    pub fm_pdgr_cd: String,
    pub ccld_nccs_dvsn: String,
    pub sll_buy_dvsn_cd: String,
    pub fuop_dvsn: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub dt: String,
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
    pub rvse_cncl_dvsn_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub cplx_ord_dvsn_cd: String,
    #[serde(default)]
    pub pric_dvsn_cd: String,
    #[serde(default)]
    pub rcit_dvsn_cd: String,
    #[serde(default)]
    pub fm_ord_qty: String,
    #[serde(default)]
    pub fm_ord_pric: String,
    #[serde(default)]
    pub fm_stop_ord_pric: String,
    #[serde(default)]
    pub ecis_rsvn_ord_yn: String,
    #[serde(default)]
    pub fm_ccld_qty: String,
    #[serde(default)]
    pub fm_ccld_pric: String,
    #[serde(default)]
    pub fm_ord_rmn_qty: String,
    #[serde(default)]
    pub ord_grp_name: String,
    #[serde(default)]
    pub rcit_dtl_dtime: String,
    #[serde(default)]
    pub ccld_dtl_dtime: String,
    #[serde(default)]
    pub ordr_emp_no: String,
    #[serde(default)]
    pub rjct_rson_name: String,
    #[serde(default)]
    pub ccld_cndt_cd: String,
    #[serde(default)]
    pub trad_end_dt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 일별 주문내역은 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("STRT_DT", req.strt_dt.as_str()),
        ("END_DT", req.end_dt.as_str()),
        ("FM_PDGR_CD", req.fm_pdgr_cd.as_str()),
        ("CCLD_NCCS_DVSN", req.ccld_nccs_dvsn.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("FUOP_DVSN", req.fuop_dvsn.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
