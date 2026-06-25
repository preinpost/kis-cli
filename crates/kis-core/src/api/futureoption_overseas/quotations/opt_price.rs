//! 해외옵션종목현재가 — GET /uapi/overseas-futureoption/v1/quotations/opt-price
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__opt_price.md
//! 모의투자 미지원. 소수점 해석은 focode.mst/fostkcode.mst의 sCalcDesz 값 참고.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/opt-price";
pub const TR_ID: &str = "HHDFO55010000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub srs_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub proc_date: String,
    #[serde(default)]
    pub proc_time: String,
    #[serde(default)]
    pub open_price: String,
    #[serde(default)]
    pub high_price: String,
    #[serde(default)]
    pub low_price: String,
    #[serde(default)]
    pub last_price: String,
    #[serde(default)]
    pub vol: String,
    #[serde(default)]
    pub prev_diff_flag: String,
    #[serde(default)]
    pub prev_diff_price: String,
    #[serde(default)]
    pub prev_diff_rate: String,
    #[serde(default)]
    pub bid_qntt: String,
    #[serde(default)]
    pub bid_price: String,
    #[serde(default)]
    pub ask_qntt: String,
    #[serde(default)]
    pub ask_price: String,
    #[serde(default)]
    pub trst_mgn: String,
    #[serde(default)]
    pub exch_cd: String,
    #[serde(default)]
    pub crc_cd: String,
    #[serde(default)]
    pub trd_fr_date: String,
    #[serde(default)]
    pub expr_date: String,
    #[serde(default)]
    pub trd_to_date: String,
    #[serde(default)]
    pub remn_cnt: String,
    #[serde(default)]
    pub last_qntt: String,
    #[serde(default)]
    pub tot_ask_qntt: String,
    #[serde(default)]
    pub tot_bid_qntt: String,
    #[serde(default)]
    pub tick_size: String,
    #[serde(default)]
    pub open_date: String,
    #[serde(default)]
    pub open_time: String,
    #[serde(default)]
    pub close_date: String,
    #[serde(default)]
    pub close_time: String,
    #[serde(default)]
    pub sbsnsdate: String,
    #[serde(default)]
    pub sttl_price: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외옵션종목현재가는 모의투자 미지원 API입니다");
    }
    let params = [("SRS_CD", req.srs_cd.as_str())];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow::anyhow!("응답에 output1 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
