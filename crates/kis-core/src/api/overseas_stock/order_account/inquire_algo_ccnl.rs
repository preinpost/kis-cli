//! 해외주식 지정가체결내역조회 — GET /uapi/overseas-stock/v1/trading/inquire-algo-ccnl
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_algo_ccnl.md
//! 모의투자 미지원. TWAP/VWAP 체결내역 (output1만; spec의 output3는 ApiResponse 미지원).

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-algo-ccnl";
pub const TR_ID: &str = "TTTS6059R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ord_dt: String,
    pub odno: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default, rename = "CCLD_SEQ")]
    pub ccld_seq: String,
    #[serde(default, rename = "CCLD_BTWN")]
    pub ccld_btwn: String,
    #[serde(default, rename = "PDNO")]
    pub pdno: String,
    #[serde(default, rename = "ITEM_NAME")]
    pub item_name: String,
    #[serde(default, rename = "FT_CCLD_QTY")]
    pub ft_ccld_qty: String,
    #[serde(default, rename = "FT_CCLD_UNPR3")]
    pub ft_ccld_unpr3: String,
    #[serde(default, rename = "FT_CCLD_AMT3")]
    pub ft_ccld_amt3: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("해외주식 지정가체결내역조회는 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("ORD_DT", req.ord_dt.as_str()),
        ("ODNO", req.odno.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
