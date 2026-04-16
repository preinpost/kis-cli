//! 해외주식 결제기준잔고 — GET /uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_paymt_stdr_balance.md
//! 모의투자 미지원. ApiResponse 래퍼는 output1/output2만 노출 → output3 무시.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-paymt-stdr-balance";
pub const TR_ID: &str = "CTRP6010R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub bass_dt: String,
    pub wcrc_frcr_dvsn_cd: String,
    pub inqr_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub cblc_qty13: String,
    #[serde(default)]
    pub ord_psbl_qty1: String,
    #[serde(default)]
    pub avg_unpr3: String,
    #[serde(default)]
    pub ovrs_now_pric1: String,
    #[serde(default)]
    pub frcr_pchs_amt: String,
    #[serde(default)]
    pub frcr_evlu_amt2: String,
    #[serde(default)]
    pub evlu_pfls_amt2: String,
    #[serde(default)]
    pub bass_exrt: String,
    #[serde(default)]
    pub oprt_dtl_dtime: String,
    #[serde(default)]
    pub buy_crcy_cd: String,
    #[serde(default)]
    pub thdt_sll_ccld_qty1: String,
    #[serde(default)]
    pub thdt_buy_ccld_qty1: String,
    #[serde(default)]
    pub evlu_pfls_rt1: String,
    #[serde(default)]
    pub tr_mket_name: String,
    #[serde(default)]
    pub natn_kor_name: String,
    #[serde(default)]
    pub std_pdno: String,
    #[serde(default)]
    pub mgge_qty: String,
    #[serde(default)]
    pub loan_rmnd: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
    #[serde(default)]
    pub scts_dvsn_name: String,
    #[serde(default)]
    pub ldng_cblc_qty: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub crcy_cd_name: String,
    #[serde(default)]
    pub frcr_dncl_amt_2: String,
    #[serde(default)]
    pub frst_bltn_exrt: String,
    #[serde(default)]
    pub frcr_evlu_amt2: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub currencies: Vec<Currency>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 결제기준잔고는 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("BASS_DT", req.bass_dt.as_str()),
        ("WCRC_FRCR_DVSN_CD", req.wcrc_frcr_dvsn_cd.as_str()),
        ("INQR_DVSN_CD", req.inqr_dvsn_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let holdings = resp
        .output1
        .map(serde_json::from_value::<Vec<Holding>>)
        .transpose()?
        .unwrap_or_default();
    let currencies = resp
        .output2
        .map(serde_json::from_value::<Vec<Currency>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { holdings, currencies })
}
