//! 선물옵션 증거금률 — GET /uapi/domestic-futureoption/v1/quotations/margin-rate
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__margin_rate.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/margin-rate";
pub const TR_ID: &str = "TTTO6032R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub bass_dt: String,
    pub bast_id: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bast_id: String,
    #[serde(default)]
    pub bast_name: String,
    #[serde(default)]
    pub brkg_mgna_rt: String,
    #[serde(default)]
    pub tr_mgna_rt: String,
    #[serde(default)]
    pub bast_pric: String,
    #[serde(default)]
    pub tr_mtpl_idx: String,
    #[serde(default)]
    pub ctrt_per_futr_mgna: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("선물옵션 증거금률은 모의투자 미지원");
    }
    let params = [
        ("BASS_DT", req.bass_dt.as_str()),
        ("BAST_ID", req.bast_id.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
