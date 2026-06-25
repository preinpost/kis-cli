//! 해외옵션종목상세 — GET /uapi/overseas-futureoption/v1/quotations/opt-detail
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__opt_detail.md
//! 모의투자 미지원. 주의: sttl_price 자리에 "전일종가"가 수신됨 (정산가 아님).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/opt-detail";
pub const TR_ID: &str = "HHDFO55010100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub srs_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
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

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외옵션종목상세는 모의투자 미지원 API입니다");
    }
    let params = [("SRS_CD", req.srs_cd.as_str())];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow::anyhow!("응답에 output1 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
