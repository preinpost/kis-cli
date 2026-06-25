//! 국내주식 수익성비율 — GET /uapi/domestic-stock/v1/finance/profit-ratio

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/finance/profit-ratio";
pub const TR_ID: &str = "FHKST66430400";

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
    pub cptl_ntin_rate: String,
    #[serde(default)]
    pub self_cptl_ntin_inrt: String,
    #[serde(default)]
    pub sale_ntin_rate: String,
    #[serde(default)]
    pub sale_totl_rate: String,
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
