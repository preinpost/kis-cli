//! 국내업종 시간별지수(분) — GET /uapi/domestic-stock/v1/quotations/inquire-index-timeprice
//!
//! 스펙: .agent/specs/domestic_stock__sector__inquire_index_timeprice.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-index-timeprice";
pub const TR_ID: &str = "FHPUP02110200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 초단위 60(1분), 300(5분), 600(10분)
    pub fid_input_hour_1: String,
    pub fid_input_iscd: String,
    pub fid_cond_mrkt_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bsop_hour: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub cntg_vol: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
