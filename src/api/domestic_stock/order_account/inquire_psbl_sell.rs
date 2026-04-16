//! 매도가능수량조회 — GET /uapi/domestic-stock/v1/trading/inquire-psbl-sell
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_psbl_sell.md
//!
//! 모의투자 미지원. output1은 Object(single).

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-psbl-sell";
pub const TR_ID: &str = "TTTC8408R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub pdno: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub buy_qty: String,
    #[serde(default)]
    pub sll_qty: String,
    #[serde(default)]
    pub cblc_qty: String,
    #[serde(default)]
    pub nsvg_qty: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
    #[serde(default)]
    pub pchs_amt: String,
    #[serde(default)]
    pub now_pric: String,
    #[serde(default)]
    pub evlu_amt: String,
    #[serde(default)]
    pub evlu_pfls_amt: String,
    #[serde(default)]
    pub evlu_pfls_rt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("매도가능수량조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output1.ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
