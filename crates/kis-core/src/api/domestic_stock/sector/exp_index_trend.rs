//! 국내주식 예상체결지수 추이 — GET /uapi/domestic-stock/v1/quotations/exp-index-trend
//!
//! 스펙: .agent/specs/domestic_stock__sector__exp_index_trend.md
//!
//! NOTE: 스펙 Response Body의 필드명-한글명 대응이 뒤섞여있음 — 필드명은 스펙 그대로 유지.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/exp-index-trend";
pub const TR_ID: &str = "FHPST01840000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 1 장시작전, 2 장마감
    pub fid_mkop_cls_code: String,
    /// 10(10초), 30(30초), 60(1분), 600(10분)
    pub fid_input_hour_1: String,
    pub fid_input_iscd: String,
    /// U
    pub fid_cond_mrkt_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_MKOP_CLS_CODE", req.fid_mkop_cls_code.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
