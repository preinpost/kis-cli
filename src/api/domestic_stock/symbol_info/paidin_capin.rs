//! 예탁원정보(유상증자일정) — GET /uapi/domestic-stock/v1/ksdinfo/paidin-capin

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ksdinfo/paidin-capin";
pub const TR_ID: &str = "HHKDB669100C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cts: String,
    pub gb1: String,
    pub f_dt: String,
    pub t_dt: String,
    pub sht_cd: String,
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
    pub tot_issue_stk_qty: String,
    #[serde(default)]
    pub issue_stk_qty: String,
    #[serde(default)]
    pub fix_rate: String,
    #[serde(default)]
    pub disc_rate: String,
    #[serde(default)]
    pub fix_price: String,
    #[serde(default)]
    pub right_dt: String,
    #[serde(default)]
    pub sub_term_ft: String,
    #[serde(default)]
    pub sub_term: String,
    #[serde(default)]
    pub list_date: String,
    #[serde(default)]
    pub stk_kind: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("CTS", req.cts.as_str()),
        ("GB1", req.gb1.as_str()),
        ("F_DT", req.f_dt.as_str()),
        ("T_DT", req.t_dt.as_str()),
        ("SHT_CD", req.sht_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output1.ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
