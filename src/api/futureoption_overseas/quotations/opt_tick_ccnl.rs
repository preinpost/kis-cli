//! 해외옵션 체결추이(틱) — GET /uapi/overseas-futureoption/v1/quotations/opt-tick-ccnl
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__opt_tick_ccnl.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/opt-tick-ccnl";
pub const TR_ID: &str = "HHDFO55020200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub srs_cd: String,
    pub exch_cd: String,
    pub start_date_time: String,
    pub close_date_time: String,
    pub qry_tp: String,
    pub qry_cnt: String,
    pub qry_gap: String,
    pub index_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub ret_cnt: String,
    #[serde(default)]
    pub last_n_cnt: String,
    #[serde(default)]
    pub index_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tick {
    #[serde(default)]
    pub data_date: String,
    #[serde(default)]
    pub data_time: String,
    #[serde(default)]
    pub open_price: String,
    #[serde(default)]
    pub high_price: String,
    #[serde(default)]
    pub low_price: String,
    #[serde(default)]
    pub last_price: String,
    #[serde(default)]
    pub last_qntt: String,
    #[serde(default)]
    pub vol: String,
    #[serde(default)]
    pub prev_diff_flag: String,
    #[serde(default)]
    pub prev_diff_price: String,
    #[serde(default)]
    pub prev_diff_rate: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub ticks: Vec<Tick>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외옵션 체결추이(틱)은 모의투자 미지원 API입니다");
    }
    let params = [
        ("SRS_CD", req.srs_cd.as_str()),
        ("EXCH_CD", req.exch_cd.as_str()),
        ("START_DATE_TIME", req.start_date_time.as_str()),
        ("CLOSE_DATE_TIME", req.close_date_time.as_str()),
        ("QRY_TP", req.qry_tp.as_str()),
        ("QRY_CNT", req.qry_cnt.as_str()),
        ("QRY_GAP", req.qry_gap.as_str()),
        ("INDEX_KEY", req.index_key.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let ticks = resp
        .output2
        .map(serde_json::from_value::<Vec<Tick>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, ticks })
}
