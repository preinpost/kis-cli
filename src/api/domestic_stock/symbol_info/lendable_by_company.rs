//! 국내주식 대주가능종목 — GET /uapi/domestic-stock/v1/quotations/lendable-by-company

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/lendable-by-company";
pub const TR_ID: &str = "CTSC2702R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub excg_dvsn_cd: String,
    pub pdno: String,
    pub thco_stln_psbl_yn: String,
    pub inqr_dvsn_1: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub papr: String,
    #[serde(default)]
    pub bfdy_clpr: String,
    #[serde(default)]
    pub sbst_prvs: String,
    #[serde(default)]
    pub tr_stop_dvsn_name: String,
    #[serde(default)]
    pub psbl_yn_name: String,
    #[serde(default)]
    pub lmt_qty1: String,
    #[serde(default)]
    pub use_qty1: String,
    #[serde(default)]
    pub trad_psbl_qty2: String,
    #[serde(default)]
    pub rght_type_cd: String,
    #[serde(default)]
    pub bass_dt: String,
    #[serde(default)]
    pub psbl_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub tot_stup_lmt_qty: String,
    #[serde(default)]
    pub brch_lmt_qty: String,
    #[serde(default)]
    pub rqst_psbl_qty: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("EXCG_DVSN_CD", req.excg_dvsn_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("THCO_STLN_PSBL_YN", req.thco_stln_psbl_yn.as_str()),
        ("INQR_DVSN_1", req.inqr_dvsn_1.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows: Vec<Row> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { rows, summary })
}
