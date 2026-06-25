//! 해외주식 지정가주문번호조회 — GET /uapi/overseas-stock/v1/trading/algo-ordno
//!
//! 스펙: .agent/specs/overseas_stock__order_account__algo_ordno.md
//! TWAP/VWAP 주문번호 조회. 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/algo-ordno";
pub const TR_ID: &str = "TTTS6058R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub trad_dt: String,
    pub cano: String,
    pub acnt_prdt_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub trad_dvsn_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub ft_ord_qty: String,
    #[serde(default)]
    pub ft_ord_unpr3: String,
    #[serde(default)]
    pub splt_buy_attr_name: String,
    #[serde(default)]
    pub ft_ccld_qty: String,
    #[serde(default)]
    pub ord_gno_brno: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("해외주식 지정가주문번호조회는 모의투자 미지원");
    }
    let params = [
        ("TRAD_DT", req.trad_dt.as_str()),
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
