//! 국내주식 종목추정실적 — GET /uapi/domestic-stock/v1/quotations/estimate-perform
//!
//! output1 (기본정보) + output2 (추정손익계산서 6개 row × 5년 데이터)
//! + output3 (투자지표 8개 row × 5년 데이터) + output4 (결산년월).

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/estimate-perform";
pub const TR_ID: &str = "HHKST668300C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub sht_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub sht_cd: String,
    #[serde(default)]
    pub item_kor_nm: String,
    #[serde(default)]
    pub name1: String,
    #[serde(default)]
    pub name2: String,
    #[serde(default)]
    pub estdate: String,
    #[serde(default)]
    pub rcmd_name: String,
    #[serde(default)]
    pub capital: String,
    #[serde(default)]
    pub forn_item_lmtrt: String,
}

/// output2(추정손익), output3(투자지표) 공통 구조 — data1~5 (최근 5년치)
#[derive(Debug, Clone, Deserialize)]
pub struct DataRow {
    #[serde(default)]
    pub data1: String,
    #[serde(default)]
    pub data2: String,
    #[serde(default)]
    pub data3: String,
    #[serde(default)]
    pub data4: String,
    #[serde(default)]
    pub data5: String,
}

/// output4 결산년월
#[derive(Debug, Clone, Deserialize)]
pub struct YearMonth {
    #[serde(default)]
    pub dt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    /// 추정손익계산서 6개 항목 (매출액/영업이익/순이익 등)
    pub estimate: Vec<DataRow>,
    /// 투자지표 8개 항목 (EBITDA/EPS/PER 등)
    pub indicators: Vec<DataRow>,
    /// 결산년월 (data1~5 대응)
    pub year_months: Vec<YearMonth>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [("SHT_CD", req.sht_cd.as_str())];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let estimate: Vec<DataRow> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    // output3/4는 ApiResponse에 없으므로 현재 비움. KIS 실제 응답은
    // output3/output4 최상위 필드가 있을 수 있으나 구조체상 접근 불가.
    let indicators = Vec::new();
    let year_months = Vec::new();
    Ok(Response { meta, estimate, indicators, year_months })
}
