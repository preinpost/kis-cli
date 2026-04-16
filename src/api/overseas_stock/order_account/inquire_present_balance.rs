//! 해외주식 체결기준현재잔고 — GET /uapi/overseas-stock/v1/trading/inquire-present-balance
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_present_balance.md
//! 모의투자는 output3만 정상 출력. ApiResponse 래퍼는 output1/output2만 노출 → output3 무시.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-present-balance";
pub const TR_ID_REAL: &str = "CTRP6504R";
pub const TR_ID_MOCK: &str = "VTRP6504R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub wcrc_frcr_dvsn_cd: String,
    pub natn_cd: String,
    pub tr_mket_cd: String,
    pub inqr_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub cblc_qty13: String,
    #[serde(default)]
    pub thdt_buy_ccld_qty1: String,
    #[serde(default)]
    pub thdt_sll_ccld_qty1: String,
    #[serde(default)]
    pub ccld_qty_smtl1: String,
    #[serde(default)]
    pub ord_psbl_qty1: String,
    #[serde(default)]
    pub frcr_pchs_amt: String,
    #[serde(default)]
    pub frcr_evlu_amt2: String,
    #[serde(default)]
    pub evlu_pfls_amt2: String,
    #[serde(default)]
    pub evlu_pfls_rt1: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub bass_exrt: String,
    #[serde(default)]
    pub buy_crcy_cd: String,
    #[serde(default)]
    pub ovrs_now_pric1: String,
    #[serde(default)]
    pub avg_unpr3: String,
    #[serde(default)]
    pub tr_mket_name: String,
    #[serde(default)]
    pub natn_kor_name: String,
    #[serde(default)]
    pub pchs_rmnd_wcrc_amt: String,
    #[serde(default)]
    pub thdt_sll_ccld_frcr_amt: String,
    #[serde(default)]
    pub unit_amt: String,
    #[serde(default)]
    pub std_pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub scts_dvsn_name: String,
    #[serde(default)]
    pub loan_rmnd: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub loan_expd_dt: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
    #[serde(default)]
    pub item_lnkg_excg_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Currency {
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub crcy_cd_name: String,
    #[serde(default)]
    pub frcr_buy_amt_smtl: String,
    #[serde(default)]
    pub frcr_sll_amt_smtl: String,
    #[serde(default)]
    pub frcr_dncl_amt_2: String,
    #[serde(default)]
    pub frst_bltn_exrt: String,
    #[serde(default)]
    pub frcr_buy_mgn_amt: String,
    #[serde(default)]
    pub frcr_etc_mgna: String,
    #[serde(default)]
    pub frcr_drwg_psbl_amt_1: String,
    #[serde(default)]
    pub frcr_evlu_amt2: String,
    #[serde(default)]
    pub acpl_cstd_crcy_yn: String,
    #[serde(default)]
    pub nxdy_frcr_drwg_psbl_amt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub currencies: Vec<Currency>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let tr = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("WCRC_FRCR_DVSN_CD", req.wcrc_frcr_dvsn_cd.as_str()),
        ("NATN_CD", req.natn_cd.as_str()),
        ("TR_MKET_CD", req.tr_mket_cd.as_str()),
        ("INQR_DVSN_CD", req.inqr_dvsn_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, tr, &params).await?;
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
