//! 해외선물 호가 — GET /uapi/overseas-futureoption/v1/quotations/inquire-asking-price
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__inquire_asking_price.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/inquire-asking-price";
pub const TR_ID: &str = "HHDFC86000000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub srs_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub open_price: String,
    #[serde(default)]
    pub high_price: String,
    #[serde(default)]
    pub lowp_rice: String,
    #[serde(default)]
    pub last_price: String,
    #[serde(default)]
    pub prev_price: String,
    #[serde(default)]
    pub vol: String,
    #[serde(default)]
    pub prev_diff_price: String,
    #[serde(default)]
    pub prev_diff_rate: String,
    #[serde(default)]
    pub quot_date: String,
    #[serde(default)]
    pub quot_time: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Level {
    #[serde(default)]
    pub bid_qntt: String,
    #[serde(default)]
    pub bid_num: String,
    #[serde(default)]
    pub bid_price: String,
    #[serde(default)]
    pub ask_qntt: String,
    #[serde(default)]
    pub ask_num: String,
    #[serde(default)]
    pub ask_price: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub summary: Option<Summary>,
    pub levels: Vec<Level>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물 호가는 모의투자 미지원 API입니다");
    }
    let params = [("SRS_CD", req.srs_cd.as_str())];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let summary = resp
        .output1
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    let levels = resp
        .output2
        .map(serde_json::from_value::<Vec<Level>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { summary, levels })
}
