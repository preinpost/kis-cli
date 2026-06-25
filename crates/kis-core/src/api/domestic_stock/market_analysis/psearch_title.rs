//! 종목조건검색 목록조회 — GET /uapi/domestic-stock/v1/quotations/psearch-title

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/psearch-title";
pub const TR_ID: &str = "HHKST03900300";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub seq: String,
    #[serde(default)]
    pub grp_nm: String,
    #[serde(default)]
    pub condition_nm: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [("user_id", req.user_id.as_str())];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output2.ok_or_else(|| anyhow!("응답에 output2 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
