//! 국내주식 배당률 상위 — GET /uapi/domestic-stock/v1/ranking/dividend-rate
//!
//! 스펙: .agent/specs/domestic_stock__ranking__dividend_rate.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/dividend-rate";
pub const TR_ID: &str = "HHKDB13470100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cts_area: String,
    pub gb1: String,
    pub upjong: String,
    pub gb2: String,
    pub gb3: String,
    pub f_dt: String,
    pub t_dt: String,
    pub gb4: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub rank: String,
    #[serde(default)]
    pub sht_cd: String,
    #[serde(default)]
    pub isin_name: String,
    #[serde(default)]
    pub record_date: String,
    #[serde(default)]
    pub per_sto_divi_amt: String,
    #[serde(default)]
    pub divi_rate: String,
    #[serde(default)]
    pub divi_kind: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내주식 배당률 상위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CTS_AREA", req.cts_area.as_str()),
        ("GB1", req.gb1.as_str()),
        ("UPJONG", req.upjong.as_str()),
        ("GB2", req.gb2.as_str()),
        ("GB3", req.gb3.as_str()),
        ("F_DT", req.f_dt.as_str()),
        ("T_DT", req.t_dt.as_str()),
        ("GB4", req.gb4.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
