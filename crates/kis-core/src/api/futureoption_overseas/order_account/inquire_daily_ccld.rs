//! 해외선물옵션 일별 체결내역 — GET /uapi/overseas-futureoption/v1/trading/inquire-daily-ccld
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_daily_ccld.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-daily-ccld";
pub const TR_ID: &str = "OTFM3122R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub strt_dt: String,
    pub end_dt: String,
    pub fuop_dvsn_cd: String,
    pub fm_pdgr_cd: String,
    pub crcy_cd: String,
    pub fm_item_ftng_yn: String,
    pub sll_buy_dvsn_cd: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub dt: String,
    #[serde(default)]
    pub ccno: String,
    #[serde(default)]
    pub ovrs_futr_fx_pdno: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub fm_ccld_qty: String,
    #[serde(default)]
    pub fm_ccld_amt: String,
    #[serde(default)]
    pub fm_futr_ccld_amt: String,
    #[serde(default)]
    pub fm_opt_ccld_amt: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub fm_fee: String,
    #[serde(default)]
    pub fm_futr_pure_agrm_amt: String,
    #[serde(default)]
    pub fm_opt_pure_agrm_amt: String,
    #[serde(default)]
    pub ccld_dtl_dtime: String,
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub ord_mdia_dvsn_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub fm_tot_ccld_qty: String,
    #[serde(default)]
    pub fm_tot_futr_agrm_amt: String,
    #[serde(default)]
    pub fm_tot_opt_agrm_amt: String,
    #[serde(default)]
    pub fm_fee_smtl: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 일별 체결내역은 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("STRT_DT", req.strt_dt.as_str()),
        ("END_DT", req.end_dt.as_str()),
        ("FUOP_DVSN_CD", req.fuop_dvsn_cd.as_str()),
        ("FM_PDGR_CD", req.fm_pdgr_cd.as_str()),
        ("CRCY_CD", req.crcy_cd.as_str()),
        ("FM_ITEM_FTNG_YN", req.fm_item_ftng_yn.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output1
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { rows, summary })
}
