//! 당사 해외주식담보대출 가능 종목 — GET /uapi/overseas-price/v1/quotations/colable-by-company
//!
//! 스펙: .agent/specs/overseas_stock__market_analysis__colable_by_company.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/colable-by-company";
pub const TR_ID: &str = "CTLN4050R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub pdno: String,
    pub prdt_type_cd: String,
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub inqr_dvsn: String,
    pub natn_cd: String,
    pub inqr_sqn_dvsn: String,
    pub rt_dvsn_cd: String,
    pub rt: String,
    pub loan_psbl_yn: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub ovrs_item_name: String,
    #[serde(default)]
    pub loan_rt: String,
    #[serde(default)]
    pub mgge_mntn_rt: String,
    #[serde(default)]
    pub mgge_ensu_rt: String,
    #[serde(default)]
    pub loan_exec_psbl_yn: String,
    #[serde(default)]
    pub stff_name: String,
    #[serde(default)]
    pub erlm_dt: String,
    #[serde(default)]
    pub tr_mket_name: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub natn_kor_name: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub loan_psbl_item_num: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub items: Vec<Item>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("당사 해외주식담보대출 가능 종목은 모의투자 미지원");
    }
    let params = [
        ("PDNO", req.pdno.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("INQR_DVSN", req.inqr_dvsn.as_str()),
        ("NATN_CD", req.natn_cd.as_str()),
        ("INQR_SQN_DVSN", req.inqr_sqn_dvsn.as_str()),
        ("RT_DVSN_CD", req.rt_dvsn_cd.as_str()),
        ("RT", req.rt.as_str()),
        ("LOAN_PSBL_YN", req.loan_psbl_yn.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let items = resp
        .output1
        .map(serde_json::from_value::<Vec<Item>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { items, summary })
}
