//! 퇴직연금 매수가능조회 — GET /uapi/domestic-stock/v1/trading/pension/inquire-psbl-order
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_psbl_order_2.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/pension/inquire-psbl-order";
pub const TR_ID: &str = "TTTC0503R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    /// 29
    pub acnt_prdt_cd: String,
    pub pdno: String,
    /// 00
    pub acca_dvsn_cd: String,
    pub cma_evlu_amt_icld_yn: String,
    /// 00 지정가, 01 시장가
    pub ord_dvsn: String,
    pub ord_unpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub ord_psbl_cash: String,
    #[serde(default)]
    pub ruse_psbl_amt: String,
    #[serde(default)]
    pub psbl_qty_calc_unpr: String,
    #[serde(default)]
    pub max_buy_amt: String,
    #[serde(default)]
    pub max_buy_qty: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("퇴직연금 매수가능조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("ACCA_DVSN_CD", req.acca_dvsn_cd.as_str()),
        ("CMA_EVLU_AMT_ICLD_YN", req.cma_evlu_amt_icld_yn.as_str()),
        ("ORD_DVSN", req.ord_dvsn.as_str()),
        ("ORD_UNPR", req.ord_unpr.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
