//! 해외주식조건검색 — GET /uapi/overseas-price/v1/quotations/inquire-search
//!
//! 스펙: .agent/specs/overseas_stock__quotations__inquire_search.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/inquire-search";
pub const TR_ID: &str = "HHDFS76410000";

#[derive(Debug, Clone, Default, Serialize)]
pub struct Request {
    pub auth: String,
    pub excd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub stat: String,
    #[serde(default)]
    pub crec: String,
    #[serde(default)]
    pub trec: String,
    #[serde(default)]
    pub nrec: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub excd: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub symb: String,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub shar: String,
    #[serde(default)]
    pub valx: String,
    #[serde(default)]
    pub plow: String,
    #[serde(default)]
    pub phigh: String,
    #[serde(default)]
    pub popen: String,
    #[serde(default)]
    pub tvol: String,
    #[serde(default)]
    pub rate: String,
    #[serde(default)]
    pub diff: String,
    #[serde(default)]
    pub sign: String,
    #[serde(default)]
    pub avol: String,
    #[serde(default)]
    pub eps: String,
    #[serde(default)]
    pub per: String,
    #[serde(default)]
    pub rank: String,
    #[serde(default)]
    pub ename: String,
    #[serde(default)]
    pub e_ordyn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let rows = resp
        .output2
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, rows })
}
