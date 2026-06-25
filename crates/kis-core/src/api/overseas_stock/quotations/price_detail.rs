//! 해외주식 현재가상세 — GET /uapi/overseas-price/v1/quotations/price-detail
//!
//! 스펙: .agent/specs/overseas_stock__quotations__price_detail.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/price-detail";
pub const TR_ID: &str = "HHDFS76200200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub auth: String,
    pub excd: String,
    pub symb: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub rsym: String,
    #[serde(default)]
    pub pvol: String,
    #[serde(default)]
    pub open: String,
    #[serde(default)]
    pub high: String,
    #[serde(default)]
    pub low: String,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub tomv: String,
    #[serde(default)]
    pub pamt: String,
    #[serde(default)]
    pub uplp: String,
    #[serde(default)]
    pub dnlp: String,
    #[serde(default)]
    pub h52p: String,
    #[serde(default)]
    pub h52d: String,
    #[serde(default)]
    pub l52p: String,
    #[serde(default)]
    pub l52d: String,
    #[serde(default)]
    pub perx: String,
    #[serde(default)]
    pub pbrx: String,
    #[serde(default)]
    pub epsx: String,
    #[serde(default)]
    pub bpsx: String,
    #[serde(default)]
    pub shar: String,
    #[serde(default)]
    pub mcap: String,
    #[serde(default)]
    pub curr: String,
    #[serde(default)]
    pub zdiv: String,
    #[serde(default)]
    pub vnit: String,
    #[serde(default)]
    pub t_xprc: String,
    #[serde(default)]
    pub t_xdif: String,
    #[serde(default)]
    pub t_xrat: String,
    #[serde(default)]
    pub p_xprc: String,
    #[serde(default)]
    pub p_xdif: String,
    #[serde(default)]
    pub p_xrat: String,
    #[serde(default)]
    pub t_rate: String,
    #[serde(default)]
    pub p_rate: String,
    #[serde(default)]
    pub t_xsgn: String,
    #[serde(default)]
    pub p_xsng: String,
    #[serde(default)]
    pub e_ordyn: String,
    #[serde(default)]
    pub e_hogau: String,
    #[serde(default)]
    pub e_icod: String,
    #[serde(default)]
    pub e_parp: String,
    #[serde(default)]
    pub tvol: String,
    #[serde(default)]
    pub tamt: String,
    #[serde(default)]
    pub etyp_nm: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 현재가상세는 모의투자 미지원");
    }
    let params = [
        ("AUTH", req.auth.as_str()),
        ("EXCD", req.excd.as_str()),
        ("SYMB", req.symb.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
