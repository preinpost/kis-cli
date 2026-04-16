//! 예탁원정보(배당일정) — GET /uapi/domestic-stock/v1/ksdinfo/dividend

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ksdinfo/dividend";
pub const TR_ID: &str = "HHKDB669102C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 공백 초기조회, 연속조회 시 이전 응답 CTS
    pub cts: String,
    /// 0 배당전체, 1 결산배당, 2 중간배당
    pub gb1: String,
    pub f_dt: String,
    pub t_dt: String,
    /// 공백 전체 또는 종목코드
    pub sht_cd: String,
    /// 공백
    pub high_gb: String,
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
    pub divi_kind: String,
    #[serde(default)]
    pub face_val: String,
    #[serde(default)]
    pub per_sto_divi_amt: String,
    #[serde(default)]
    pub divi_rate: String,
    #[serde(default)]
    pub stk_divi_rate: String,
    #[serde(default)]
    pub divi_pay_dt: String,
    #[serde(default)]
    pub stk_div_pay_dt: String,
    #[serde(default)]
    pub odd_pay_dt: String,
    #[serde(default)]
    pub stk_kind: String,
    #[serde(default)]
    pub high_divi_gb: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("CTS", req.cts.as_str()),
        ("GB1", req.gb1.as_str()),
        ("F_DT", req.f_dt.as_str()),
        ("T_DT", req.t_dt.as_str()),
        ("SHT_CD", req.sht_cd.as_str()),
        ("HIGH_GB", req.high_gb.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output1.ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
