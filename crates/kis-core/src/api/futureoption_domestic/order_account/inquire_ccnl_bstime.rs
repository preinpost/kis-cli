//! 선물옵션 기준일체결내역 — GET /uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_ccnl_bstime.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-ccnl-bstime";
pub const TR_ID: &str = "CTFO5139R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ord_dt: String,
    pub fuop_tr_strt_tmd: String,
    pub fuop_tr_end_tmd: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub tr_type_name: String,
    #[serde(default)]
    pub last_sttldt: String,
    #[serde(default)]
    pub ccld_idx: String,
    #[serde(default)]
    pub ccld_qty: String,
    #[serde(default)]
    pub trad_amt: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub ccld_btwn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub tot_ccld_qty_smtl: String,
    #[serde(default)]
    pub tot_ccld_amt_smtl: String,
    #[serde(default)]
    pub fee_adjt: String,
    #[serde(default)]
    pub fee_smtl: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("선물옵션 기준일체결내역은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("ORD_DT", req.ord_dt.as_str()),
        ("FUOP_TR_STRT_TMD", req.fuop_tr_strt_tmd.as_str()),
        ("FUOP_TR_END_TMD", req.fuop_tr_end_tmd.as_str()),
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
