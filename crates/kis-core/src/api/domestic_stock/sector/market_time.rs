//! 국내선물 영업일조회 — GET /uapi/domestic-stock/v1/quotations/market-time
//!
//! 스펙: .agent/specs/domestic_stock__sector__market_time.md
//!
//! 파라미터 없음. 5영업일 + 오늘/장시작/장마감 시간 반환.

use anyhow::anyhow;
use serde::Deserialize;

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/market-time";
pub const TR_ID: &str = "HHMCM000002C0";

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub date1: String,
    #[serde(default)]
    pub date2: String,
    #[serde(default)]
    pub date3: String,
    #[serde(default)]
    pub date4: String,
    #[serde(default)]
    pub date5: String,
    #[serde(default)]
    pub today: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub s_time: String,
    #[serde(default)]
    pub e_time: String,
}

pub async fn call(client: &KisClient) -> anyhow::Result<Response> {
    let resp = client.get(ENDPOINT, TR_ID, &[]).await?;
    let output = resp.output1.ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let parsed: Response = match output {
        serde_json::Value::Array(mut arr) => {
            let first = arr.pop().ok_or_else(|| anyhow!("output1 배열 비어있음"))?;
            serde_json::from_value(first)?
        }
        v => serde_json::from_value(v)?,
    };
    Ok(parsed)
}
