//! 장내채권 기간별시세(일) — GET /uapi/domestic-bond/v1/quotations/inquire-daily-itemchartprice
//!
//! 스펙: .agent/specs/bond__quotations__inquire_daily_itemchartprice.md
//! 모의투자 미지원. 최근 30건까지 조회.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/quotations/inquire-daily-itemchartprice";
pub const TR_ID: &str = "FHKBJ773701C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub market: String,
    pub symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub bond_oprc: String,
    #[serde(default)]
    pub bond_hgpr: String,
    #[serde(default)]
    pub bond_lwpr: String,
    #[serde(default)]
    pub bond_prpr: String,
    #[serde(default)]
    pub acml_vol: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 기간별시세(일)는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.market.as_str()),
        ("FID_INPUT_ISCD", req.symbol.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
