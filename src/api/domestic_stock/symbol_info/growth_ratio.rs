//! 국내주식 성장성비율 — GET /uapi/domestic-stock/v1/finance/growth-ratio

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/finance/growth-ratio";
pub const TR_ID: &str = "FHKST66430800";

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
    /// 자기자본증가율
    #[serde(default)]
    pub equt_inrt: String,
    /// 총자산증가율
    #[serde(default)]
    pub totl_aset_inrt: String,
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
