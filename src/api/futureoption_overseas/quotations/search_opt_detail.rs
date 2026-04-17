//! 해외옵션 상품기본정보 — GET /uapi/overseas-futureoption/v1/quotations/search-opt-detail
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__search_opt_detail.md
//! 모의투자 미지원. SRS_CD_01~SRS_CD_30 최대 30건 조회.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/search-opt-detail";
pub const TR_ID: &str = "HHDFO55200000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub srs_cds: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub exch_cd: String,
    #[serde(default)]
    pub clas_cd: String,
    #[serde(default)]
    pub crc_cd: String,
    #[serde(default)]
    pub sttl_price: String,
    #[serde(default)]
    pub sttl_date: String,
    #[serde(default)]
    pub trst_mgn: String,
    #[serde(default)]
    pub disp_digit: String,
    #[serde(default)]
    pub tick_sz: String,
    #[serde(default)]
    pub tick_val: String,
    #[serde(default)]
    pub mrkt_open_date: String,
    #[serde(default)]
    pub mrkt_open_time: String,
    #[serde(default)]
    pub mrkt_close_date: String,
    #[serde(default)]
    pub mrkt_close_time: String,
    #[serde(default)]
    pub trd_fr_date: String,
    #[serde(default)]
    pub expr_date: String,
    #[serde(default)]
    pub trd_to_date: String,
    #[serde(default)]
    pub remn_cnt: String,
    #[serde(default)]
    pub stat_tp: String,
    #[serde(default)]
    pub ctrt_size: String,
    #[serde(default)]
    pub stl_tp: String,
    #[serde(default)]
    pub frst_noti_date: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외옵션 상품기본정보는 모의투자 미지원 API입니다");
    }
    if req.srs_cds.is_empty() || req.srs_cds.len() > 30 {
        bail!("srs_cds 개수는 1~30건이어야 합니다 (현재 {})", req.srs_cds.len());
    }
    let qry_cnt = req.srs_cds.len().to_string();
    let mut owned: Vec<(String, String)> = Vec::with_capacity(1 + req.srs_cds.len());
    owned.push(("QRY_CNT".to_string(), qry_cnt));
    for (i, code) in req.srs_cds.iter().enumerate() {
        owned.push((format!("SRS_CD_{:02}", i + 1), code.clone()));
    }
    let params: Vec<(&str, &str)> = owned.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output2
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
