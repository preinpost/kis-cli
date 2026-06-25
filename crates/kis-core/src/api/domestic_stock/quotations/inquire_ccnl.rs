//! 주식현재가 체결 — GET /uapi/domestic-stock/v1/quotations/inquire-ccnl
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_ccnl.md
//!
//! output이 array (체결 데이터 여러 건).

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-ccnl";
pub const TR_ID: &str = "FHKST01010300";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J:KRX, NX:NXT, UN:통합
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub cntg_vol: String,
    #[serde(default)]
    pub tday_rltv: String,
    #[serde(default)]
    pub prdy_ctrt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
