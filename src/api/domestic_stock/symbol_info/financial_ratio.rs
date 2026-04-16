//! 국내주식 재무비율 — GET /uapi/domestic-stock/v1/finance/financial-ratio
//!
//! 스펙: .agent/specs/domestic_stock__symbol_info__financial_ratio.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/finance/financial-ratio";
pub const TR_ID: &str = "FHKST66430300";

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
    /// 매출액증가율
    #[serde(default)]
    pub grs: String,
    /// 영업이익증가율
    #[serde(default)]
    pub bsop_prfi_inrt: String,
    /// 당기순이익증가율
    #[serde(default)]
    pub ntin_inrt: String,
    /// ROE
    #[serde(default)]
    pub roe_val: String,
    #[serde(default)]
    pub eps: String,
    /// 주당매출액
    #[serde(default)]
    pub sps: String,
    #[serde(default)]
    pub bps: String,
    /// 유보율
    #[serde(default)]
    pub rsrv_rate: String,
    /// 부채비율
    #[serde(default)]
    pub lblt_rate: String,
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
