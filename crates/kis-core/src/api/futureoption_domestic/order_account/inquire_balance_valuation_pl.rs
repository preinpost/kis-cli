//! 선물옵션 잔고평가손익내역 — GET /uapi/domestic-futureoption/v1/trading/inquire-balance-valuation-pl
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_balance_valuation_pl.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-balance-valuation-pl";
pub const TR_ID: &str = "CTFO6159R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub mgna_dvsn: String,
    pub excc_stat_cd: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub shtn_pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub sll_buy_dvsn_name: String,
    #[serde(default)]
    pub cblc_qty1: String,
    #[serde(default)]
    pub excc_unpr: String,
    #[serde(default)]
    pub ccld_avg_unpr1: String,
    #[serde(default)]
    pub idx_clpr: String,
    #[serde(default)]
    pub pchs_amt: String,
    #[serde(default)]
    pub evlu_amt: String,
    #[serde(default)]
    pub evlu_pfls_amt: String,
    #[serde(default)]
    pub trad_pfls_amt: String,
    #[serde(default)]
    pub lqd_psbl_qty: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub dnca_cash: String,
    #[serde(default)]
    pub frcr_dncl_amt: String,
    #[serde(default)]
    pub dnca_sbst: String,
    #[serde(default)]
    pub tot_dncl_amt: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub cash_mgna: String,
    #[serde(default)]
    pub sbst_mgna: String,
    #[serde(default)]
    pub mgna_tota: String,
    #[serde(default)]
    pub opt_dfpa: String,
    #[serde(default)]
    pub thdt_dfpa: String,
    #[serde(default)]
    pub rnwl_dfpa: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub nxdy_dnca: String,
    #[serde(default)]
    pub nxdy_dncl_amt: String,
    #[serde(default)]
    pub prsm_dpast: String,
    #[serde(default)]
    pub prsm_dpast_amt: String,
    #[serde(default)]
    pub pprt_ord_psbl_cash: String,
    #[serde(default)]
    pub add_mgna_cash: String,
    #[serde(default)]
    pub add_mgna_tota: String,
    #[serde(default)]
    pub futr_trad_pfls_amt: String,
    #[serde(default)]
    pub opt_trad_pfls_amt: String,
    #[serde(default)]
    pub futr_evlu_pfls_amt: String,
    #[serde(default)]
    pub opt_evlu_pfls_amt: String,
    #[serde(default)]
    pub trad_pfls_amt_smtl: String,
    #[serde(default)]
    pub evlu_pfls_amt_smtl: String,
    #[serde(default)]
    pub wdrw_psbl_tot_amt: String,
    #[serde(default)]
    pub ord_psbl_cash: String,
    #[serde(default)]
    pub ord_psbl_sbst: String,
    #[serde(default)]
    pub ord_psbl_tota: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("선물옵션 잔고평가손익내역은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("MGNA_DVSN", req.mgna_dvsn.as_str()),
        ("EXCC_STAT_CD", req.excc_stat_cd.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let holdings = resp
        .output1
        .map(serde_json::from_value::<Vec<Holding>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { holdings, summary })
}
