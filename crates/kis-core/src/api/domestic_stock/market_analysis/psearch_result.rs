//! 종목조건검색조회 — GET /uapi/domestic-stock/v1/quotations/psearch-result

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/psearch-result";
pub const TR_ID: &str = "HHKST03900400";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub user_id: String,
    /// psearch_title에서 받은 seq
    pub seq: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub daebi: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub chgrate: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub trade_amt: String,
    #[serde(default)]
    pub change: String,
    #[serde(default)]
    pub cttr: String,
    #[serde(default)]
    pub open: String,
    #[serde(default)]
    pub high: String,
    #[serde(default)]
    pub low: String,
    #[serde(default)]
    pub high52: String,
    #[serde(default)]
    pub low52: String,
    #[serde(default)]
    pub expprice: String,
    #[serde(default)]
    pub expchange: String,
    #[serde(default)]
    pub expchggrate: String,
    #[serde(default)]
    pub expcvol: String,
    #[serde(default)]
    pub chgrate2: String,
    #[serde(default)]
    pub expdaebi: String,
    #[serde(default)]
    pub recprice: String,
    #[serde(default)]
    pub uplmtprice: String,
    #[serde(default)]
    pub dnlmtprice: String,
    #[serde(default)]
    pub stotprice: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("user_id", req.user_id.as_str()),
        ("seq", req.seq.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output2.ok_or_else(|| anyhow!("응답에 output2 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
