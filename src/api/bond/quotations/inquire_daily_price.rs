//! 장내채권현재가(일별) — GET /uapi/domestic-bond/v1/quotations/inquire-daily-price
//!
//! 스펙: .agent/specs/bond__quotations__inquire_daily_price.md
//! 모의투자 미지원.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/quotations/inquire-daily-price";
pub const TR_ID: &str = "FHKBJ773404C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub market: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub bond_prpr: String,
    #[serde(default)]
    pub bond_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub bond_oprc: String,
    #[serde(default)]
    pub bond_hgpr: String,
    #[serde(default)]
    pub bond_lwpr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권현재가(일별)는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.market.as_str()),
        ("FID_INPUT_ISCD", req.symbol.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.context("응답에 output 없음")?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
