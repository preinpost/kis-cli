//! 예탁원정보(실권주일정) — GET /uapi/domestic-stock/v1/ksdinfo/forfeit

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ksdinfo/forfeit";
pub const TR_ID: &str = "HHKDB669109C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub sht_cd: String,
    pub t_dt: String,
    pub f_dt: String,
    pub cts: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub record_date: String,
    #[serde(default)]
    pub sht_cd: String,
    #[serde(default)]
    pub isin_name: String,
    #[serde(default)]
    pub subscr_dt: String,
    #[serde(default)]
    pub subscr_price: String,
    #[serde(default)]
    pub subscr_stk_qty: String,
    #[serde(default)]
    pub refund_dt: String,
    #[serde(default)]
    pub list_dt: String,
    #[serde(default)]
    pub lead_mgr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("SHT_CD", req.sht_cd.as_str()),
        ("T_DT", req.t_dt.as_str()),
        ("F_DT", req.f_dt.as_str()),
        ("CTS", req.cts.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output1.ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
