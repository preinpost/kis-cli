//! 종목별 외인기관 추정가집계 — GET /uapi/domestic-stock/v1/quotations/investor-trend-estimate

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/investor-trend-estimate";
pub const TR_ID: &str = "HHPTJ04160200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub mksc_shrn_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bsop_hour_gb: String,
    #[serde(default)]
    pub frgn_fake_ntby_qty: String,
    #[serde(default)]
    pub orgn_fake_ntby_qty: String,
    #[serde(default)]
    pub sum_fake_ntby_qty: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [("MKSC_SHRN_ISCD", req.mksc_shrn_iscd.as_str())];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
