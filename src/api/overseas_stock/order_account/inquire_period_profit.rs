//! 해외주식 기간손익 — GET /uapi/overseas-stock/v1/trading/inquire-period-profit
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_period_profit.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-period-profit";
pub const TR_ID: &str = "TTTS3039R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ovrs_excg_cd: String,
    pub natn_cd: String,
    pub crcy_cd: String,
    pub pdno: String,
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub wcrc_frcr_dvsn_cd: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Profit {
    #[serde(default)]
    pub trad_day: String,
    #[serde(default)]
    pub ovrs_pdno: String,
    #[serde(default)]
    pub ovrs_item_name: String,
    #[serde(default)]
    pub slcl_qty: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
    #[serde(default)]
    pub frcr_pchs_amt1: String,
    #[serde(default)]
    pub avg_sll_unpr: String,
    #[serde(default)]
    pub frcr_sll_amt_smtl1: String,
    #[serde(default)]
    pub stck_sll_tlex: String,
    #[serde(default)]
    pub ovrs_rlzt_pfls_amt: String,
    #[serde(default)]
    pub pftrt: String,
    #[serde(default)]
    pub exrt: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
    #[serde(default)]
    pub frst_bltn_exrt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub stck_sll_amt_smtl: String,
    #[serde(default)]
    pub stck_buy_amt_smtl: String,
    #[serde(default)]
    pub smtl_fee1: String,
    #[serde(default)]
    pub excc_dfrm_amt: String,
    #[serde(default)]
    pub ovrs_rlzt_pfls_tot_amt: String,
    #[serde(default)]
    pub tot_pftrt: String,
    #[serde(default)]
    pub bass_dt: String,
    #[serde(default)]
    pub exrt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Profit>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 기간손익은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("OVRS_EXCG_CD", req.ovrs_excg_cd.as_str()),
        ("NATN_CD", req.natn_cd.as_str()),
        ("CRCY_CD", req.crcy_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("WCRC_FRCR_DVSN_CD", req.wcrc_frcr_dvsn_cd.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output1
        .map(serde_json::from_value::<Vec<Profit>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { rows, summary })
}
