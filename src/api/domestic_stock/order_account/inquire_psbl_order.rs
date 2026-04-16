//! 매수가능조회 — GET /uapi/domestic-stock/v1/trading/inquire-psbl-order
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_psbl_order.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-psbl-order";
pub const TR_ID_REAL: &str = "TTTC8908R";
pub const TR_ID_MOCK: &str = "VTTC8908R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    /// PDNO + ORD_UNPR 공란이면 매수금액만 조회
    pub pdno: String,
    /// 시장가는 공란
    pub ord_unpr: String,
    /// 전량매수 가능수량 확인 시 01(시장가) 권장
    pub ord_dvsn: String,
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
    /// 미수 사용 X — 이 값 확인
    #[serde(default)]
    pub nrcvb_buy_amt: String,
    #[serde(default)]
    pub nrcvb_buy_qty: String,
    /// 미수 사용 O — 이 값 확인
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
    let tr_id = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("ORD_UNPR", req.ord_unpr.as_str()),
        ("ORD_DVSN", req.ord_dvsn.as_str()),
        ("CMA_EVLU_AMT_ICLD_YN", req.cma_evlu_amt_icld_yn.as_str()),
        ("OVRS_ICLD_YN", req.ovrs_icld_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, tr_id, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
