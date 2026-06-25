//! 국내주식 안정성비율 — GET /uapi/domestic-stock/v1/finance/stability-ratio

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/finance/stability-ratio";
pub const TR_ID: &str = "FHKST66430600";

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
    /// 부채비율
    #[serde(default)]
    pub lblt_rate: String,
    /// 차입금의존도
    #[serde(default)]
    pub bram_depn: String,
    /// 유동비율
    #[serde(default)]
    pub crnt_rate: String,
    /// 당좌비율
    #[serde(default)]
    pub quck_rate: String,
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
