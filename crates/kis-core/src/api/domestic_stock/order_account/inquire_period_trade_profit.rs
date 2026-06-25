//! 기간별매매손익현황조회 — GET /uapi/domestic-stock/v1/trading/inquire-period-trade-profit
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_period_trade_profit.md
//!
//! 모의투자 미지원. output1(종목별 체결 Vec) + output2(합계).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-period-trade-profit";
pub const TR_ID: &str = "TTTC8715R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    /// 00 최근, 01 과거, 02 최근
    pub sort_dvsn: String,
    pub acnt_prdt_cd: String,
    pub pdno: String,
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub ctx_area_nk100: String,
    /// 00 전체
    pub cblc_dvsn: String,
    pub ctx_area_fk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub trad_dt: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub trad_dvsn_name: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub hldg_qty: String,
    #[serde(default)]
    pub pchs_unpr: String,
    #[serde(default)]
    pub buy_qty: String,
    #[serde(default)]
    pub buy_amt: String,
    #[serde(default)]
    pub sll_pric: String,
    #[serde(default)]
    pub sll_qty: String,
    #[serde(default)]
    pub sll_amt: String,
    #[serde(default)]
    pub rlzt_pfls: String,
    #[serde(default)]
    pub pfls_rt: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub tl_tax: String,
    #[serde(default)]
    pub loan_int: String,
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
    pub buyqty_smtl: String,
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
    #[serde(default)]
    pub tot_pftrt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("기간별매매손익현황조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("SORT_DVSN", req.sort_dvsn.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
        ("CBLC_DVSN", req.cblc_dvsn.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
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
