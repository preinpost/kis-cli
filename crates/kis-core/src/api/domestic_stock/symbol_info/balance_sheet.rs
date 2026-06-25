//! 국내주식 대차대조표 — GET /uapi/domestic-stock/v1/finance/balance-sheet
//!
//! 스펙: .agent/specs/domestic_stock__symbol_info__balance_sheet.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/finance/balance-sheet";
pub const TR_ID: &str = "FHKST66430100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 0 년, 1 분기
    pub fid_div_cls_code: String,
    /// J
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stac_yymm: String,
    #[serde(default)]
    pub cras: String,
    #[serde(default)]
    pub fxas: String,
    #[serde(default)]
    pub total_aset: String,
    #[serde(default)]
    pub flow_lblt: String,
    #[serde(default)]
    pub fix_lblt: String,
    #[serde(default)]
    pub total_lblt: String,
    #[serde(default)]
    pub cpfn: String,
    #[serde(default)]
    pub cfp_surp: String,
    #[serde(default)]
    pub prfi_surp: String,
    #[serde(default)]
    pub total_cptl: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
