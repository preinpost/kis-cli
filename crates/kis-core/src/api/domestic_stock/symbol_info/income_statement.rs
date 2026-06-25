//! 국내주식 손익계산서 — GET /uapi/domestic-stock/v1/finance/income-statement
//!
//! 스펙: .agent/specs/domestic_stock__symbol_info__income_statement.md
//!
//! 분기 데이터는 연단위 누적합산.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/finance/income-statement";
pub const TR_ID: &str = "FHKST66430200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_div_cls_code: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stac_yymm: String,
    #[serde(default)]
    pub sale_account: String,
    #[serde(default)]
    pub sale_cost: String,
    #[serde(default)]
    pub sale_totl_prfi: String,
    #[serde(default)]
    pub depr_cost: String,
    #[serde(default)]
    pub sell_mang: String,
    #[serde(default)]
    pub bsop_prti: String,
    #[serde(default)]
    pub bsop_non_ernn: String,
    #[serde(default)]
    pub bsop_non_expn: String,
    #[serde(default)]
    pub op_prfi: String,
    #[serde(default)]
    pub spec_prfi: String,
    #[serde(default)]
    pub spec_loss: String,
    #[serde(default)]
    pub thtr_ntin: String,
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
