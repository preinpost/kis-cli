//! HTS조회상위20종목 — GET /uapi/domestic-stock/v1/ranking/hts-top-view
//!
//! 스펙: .agent/specs/domestic_stock__ranking__hts_top_view.md
//! 모의투자 미지원. 쿼리 파라미터 없이 호출.

use anyhow::{bail, Result};
use serde::Deserialize;

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/hts-top-view";
pub const TR_ID: &str = "HHMCM000100C0";

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub mrkt_div_cls_code: String,
    #[serde(default)]
    pub mksc_shrn_iscd: String,
}

pub async fn call(client: &KisClient) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("HTS조회상위20종목은 모의투자 미지원 API입니다");
    }
    let resp = client.get(ENDPOINT, TR_ID, &[]).await?;
    let rows: Vec<Row> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(rows)
}
