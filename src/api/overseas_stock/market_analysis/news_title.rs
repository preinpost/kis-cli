//! 해외뉴스종합(제목) — GET /uapi/overseas-price/v1/quotations/news-title
//!
//! 스펙: .agent/specs/overseas_stock__market_analysis__news_title.md
//! 모의투자 미지원.
//!
//! 주의: 응답 필드명이 `outblock1` (비표준). ApiResponse 래퍼는 output/output1/output2만
//! 노출 → 현재 구현은 `output1`로 fallback 시도. 실제 데이터를 받으려면 ApiResponse 확장이 필요.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/news-title";
pub const TR_ID: &str = "HHPSTH60100C1";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub info_gb: String,
    pub class_cd: String,
    pub nation_cd: String,
    pub exchange_cd: String,
    pub symb: String,
    pub data_dt: String,
    pub data_tm: String,
    pub cts: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct News {
    #[serde(default)]
    pub info_gb: String,
    #[serde(default)]
    pub news_key: String,
    #[serde(default)]
    pub data_dt: String,
    #[serde(default)]
    pub data_tm: String,
    #[serde(default)]
    pub class_cd: String,
    #[serde(default)]
    pub class_name: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub nation_cd: String,
    #[serde(default)]
    pub exchange_cd: String,
    #[serde(default)]
    pub symb: String,
    #[serde(default)]
    pub symb_name: String,
    #[serde(default)]
    pub title: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<News>> {
    if client.is_mock() {
        bail!("해외뉴스종합(제목)은 모의투자 미지원");
    }
    let params = [
        ("INFO_GB", req.info_gb.as_str()),
        ("CLASS_CD", req.class_cd.as_str()),
        ("NATION_CD", req.nation_cd.as_str()),
        ("EXCHANGE_CD", req.exchange_cd.as_str()),
        ("SYMB", req.symb.as_str()),
        ("DATA_DT", req.data_dt.as_str()),
        ("DATA_TM", req.data_tm.as_str()),
        ("CTS", req.cts.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let news = resp
        .output1
        .map(serde_json::from_value::<Vec<News>>)
        .transpose()?
        .unwrap_or_default();
    Ok(news)
}
