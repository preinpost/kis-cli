//! 기간별손익일별합산조회 — GET /uapi/domestic-stock/v1/trading/inquire-period-profit
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_period_profit.md
//!
//! 모의투자 미지원. output1(일별 행) + output2(합계).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-period-profit";
pub const TR_ID: &str = "TTTC8708R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub acnt_prdt_cd: String,
    pub cano: String,
    pub inqr_strt_dt: String,
    /// 공란 시 전체
    pub pdno: String,
    pub ctx_area_nk100: String,
    pub inqr_end_dt: String,
    /// 00 최근순, 01 과거순, 02 최근순
    pub sort_dvsn: String,
    /// 00 입력
    pub inqr_dvsn: String,
    /// 00 전체
    pub cblc_dvsn: String,
    pub ctx_area_fk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DailyRow {
    #[serde(default)]
    pub trad_dt: String,
    #[serde(default)]
    pub buy_amt: String,
    #[serde(default)]
    pub sll_amt: String,
    #[serde(default)]
    pub rlzt_pfls: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub loan_int: String,
    #[serde(default)]
    pub tl_tax: String,
    #[serde(default)]
    pub pfls_rt: String,
    #[serde(default)]
    pub sll_qty1: String,
    #[serde(default)]
    pub buy_qty1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub sll_qty_smtl: String,
    #[serde(default)]
    pub sll_tr_amt_smtl: String,
    #[serde(default)]
    pub sll_fee_smtl: String,
    #[serde(default)]
    pub sll_tltx_smtl: String,
    #[serde(default)]
    pub sll_excc_amt_smtl: String,
    #[serde(default)]
    pub buy_qty_smtl: String,
    #[serde(default)]
    pub buy_tr_amt_smtl: String,
    #[serde(default)]
    pub buy_fee_smtl: String,
    #[serde(default)]
    pub buy_tax_smtl: String,
    #[serde(default)]
    pub buy_excc_amt_smtl: String,
    #[serde(default)]
    pub tot_qty: String,
    #[serde(default)]
    pub tot_tr_amt: String,
    #[serde(default)]
    pub tot_fee: String,
    #[serde(default)]
    pub tot_tltx: String,
    #[serde(default)]
    pub tot_excc_amt: String,
    #[serde(default)]
    pub tot_rlzt_pfls: String,
    #[serde(default)]
    pub loan_int: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub daily: Vec<DailyRow>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("기간별손익일별합산조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("CANO", req.cano.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("SORT_DVSN", req.sort_dvsn.as_str()),
        ("INQR_DVSN", req.inqr_dvsn.as_str()),
        ("CBLC_DVSN", req.cblc_dvsn.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let daily: Vec<DailyRow> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { daily, summary })
}
