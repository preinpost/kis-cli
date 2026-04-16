//! 국내휴장일조회 — GET /uapi/domestic-stock/v1/quotations/chk-holiday
//!
//! 스펙: .agent/specs/domestic_stock__sector__chk_holiday.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/chk-holiday";
pub const TR_ID: &str = "CTCA0903R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// YYYYMMDD
    pub bass_dt: String,
    /// 공백
    pub ctx_area_nk: String,
    /// 공백
    pub ctx_area_fk: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub bass_dt: String,
    /// 01~07 (일~토)
    #[serde(default)]
    pub wday_dvsn_cd: String,
    /// Y/N 영업일(금융기관 업무일)
    #[serde(default)]
    pub bzdy_yn: String,
    /// Y/N 거래일
    #[serde(default)]
    pub tr_day_yn: String,
    /// Y/N 개장일 — 주문 가능일 판단에 사용
    #[serde(default)]
    pub opnd_yn: String,
    /// Y/N 결제일
    #[serde(default)]
    pub sttl_day_yn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("BASS_DT", req.bass_dt.as_str()),
        ("CTX_AREA_NK", req.ctx_area_nk.as_str()),
        ("CTX_AREA_FK", req.ctx_area_fk.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
