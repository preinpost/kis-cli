//! 신용매수가능조회 — GET /uapi/domestic-stock/v1/trading/inquire-credit-psamount
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_credit_psamount.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-credit-psamount";
pub const TR_ID: &str = "TTTC8909R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub pdno: String,
    /// 시장가는 "0" 권고
    pub ord_unpr: String,
    pub ord_dvsn: String,
    /// 신용유형 21/22/23/24/25/26/27/28
    pub crdt_type: String,
    pub cma_evlu_amt_icld_yn: String,
    pub ovrs_icld_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub ord_psbl_cash: String,
    #[serde(default)]
    pub ord_psbl_sbst: String,
    #[serde(default)]
    pub ruse_psbl_amt: String,
    #[serde(default)]
    pub fund_rpch_chgs: String,
    #[serde(default)]
    pub psbl_qty_calc_unpr: String,
    #[serde(default)]
    pub nrcvb_buy_amt: String,
    #[serde(default)]
    pub nrcvb_buy_qty: String,
    #[serde(default)]
    pub max_buy_amt: String,
    #[serde(default)]
    pub max_buy_qty: String,
    #[serde(default)]
    pub cma_evlu_amt: String,
    #[serde(default)]
    pub ovrs_re_use_amt_wcrc: String,
    #[serde(default)]
    pub ord_psbl_frcr_amt_wcrc: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("신용매수가능조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("ORD_UNPR", req.ord_unpr.as_str()),
        ("ORD_DVSN", req.ord_dvsn.as_str()),
        ("CRDT_TYPE", req.crdt_type.as_str()),
        ("CMA_EVLU_AMT_ICLD_YN", req.cma_evlu_amt_icld_yn.as_str()),
        ("OVRS_ICLD_YN", req.ovrs_icld_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
