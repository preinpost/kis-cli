//! 해외증거금 통화별조회 — GET /uapi/overseas-stock/v1/trading/foreign-margin
//!
//! 스펙: .agent/specs/overseas_stock__order_account__foreign_margin.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/foreign-margin";
pub const TR_ID: &str = "TTTC2101R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub natn_name: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub frcr_dncl_amt1: String,
    #[serde(default)]
    pub ustl_buy_amt: String,
    #[serde(default)]
    pub ustl_sll_amt: String,
    #[serde(default)]
    pub frcr_rcvb_amt: String,
    #[serde(default)]
    pub frcr_mgn_amt: String,
    #[serde(default)]
    pub frcr_gnrl_ord_psbl_amt: String,
    #[serde(default)]
    pub frcr_ord_psbl_amt1: String,
    #[serde(default)]
    pub itgr_ord_psbl_amt: String,
    #[serde(default)]
    pub bass_exrt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("해외증거금 통화별조회는 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
