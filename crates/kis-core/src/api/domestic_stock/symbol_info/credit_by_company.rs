//! 국내주식 당사 신용가능종목 — GET /uapi/domestic-stock/v1/quotations/credit-by-company

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/credit-by-company";
pub const TR_ID: &str = "FHPST04770000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 파라미터 구조는 스펙에 명확히 안 나와있음 — 대부분 공백 또는 조건값
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_div_cls_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub crdt_rate: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
