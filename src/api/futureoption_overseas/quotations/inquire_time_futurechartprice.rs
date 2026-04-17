//! 해외선물 분봉조회 — GET /uapi/overseas-futureoption/v1/quotations/inquire-time-futurechartprice
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__inquire_time_futurechartprice.md
//! 모의투자 미지원. 연속조회: QRY_TP=P, INDEX_KEY에 이전 output2.index_key 입력.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/inquire-time-futurechartprice";
pub const TR_ID: &str = "HHDFC55020400";

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
pub struct Bar {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub ret_cnt: String,
    #[serde(default)]
    pub last_n_cnt: String,
    #[serde(default)]
    pub index_key: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub bars: Vec<Bar>,
    pub meta: Option<Meta>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물 분봉조회는 모의투자 미지원 API입니다");
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
    let bars = resp
        .output1
        .map(serde_json::from_value::<Vec<Bar>>)
        .transpose()?
        .unwrap_or_default();
    let meta = resp
        .output2
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    Ok(Response { bars, meta })
}
