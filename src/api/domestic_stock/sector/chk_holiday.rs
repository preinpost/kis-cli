//! 국내휴장일조회 — GET /uapi/domestic-stock/v1/quotations/chk-holiday
//!
//! 스펙: .agent/specs/domestic_stock__sector__chk_holiday.md
//!
//! KIS 가이드: 동일 일자에 대해 1일 1회만 호출 권장.
//!
//! 응답 형태가 스펙과 실제 사이에서 자주 어긋나는 API라 방어적으로 파싱:
//! - `output` / `output1` 모두 시도
//! - Array(여러 일자) 면 BASS_DT 일치하는 행 선택, 없으면 첫 행
//! - 파싱 실패 시 원본 JSON을 에러 메시지에 포함

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

    let value = resp
        .output
        .or(resp.output1)
        .ok_or_else(|| anyhow!("응답에 output/output1 없음"))?;

    let rows: Vec<serde_json::Value> = match value {
        serde_json::Value::Array(arr) => arr,
        v => vec![v],
    };
    if rows.is_empty() {
        return Err(anyhow!("응답 배열 비어있음"));
    }

    // BASS_DT 일치하는 행 우선, 없으면 첫 행
    let chosen = rows
        .iter()
        .find(|r| r.get("bass_dt").and_then(|v| v.as_str()) == Some(req.bass_dt.as_str()))
        .cloned()
        .unwrap_or_else(|| rows[0].clone());

    serde_json::from_value::<Response>(chosen.clone())
        .map_err(|e| anyhow!("파싱 실패: {} (raw: {})", e, chosen))
}
