//! 종합 시황/공시(제목) — GET /uapi/domestic-stock/v1/quotations/news-title
//!
//! 스펙: .agent/specs/domestic_stock__sector__news_title.md
//!
//! 대부분 파라미터는 공백 입력이 필수. 연속조회용 일련번호로 페이징.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/news-title";
pub const TR_ID: &str = "FHKST01011800";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 공백 입력
    pub fid_news_ofer_entp_code: String,
    /// 공백 입력
    pub fid_cond_mrkt_cls_code: String,
    /// 공백: 전체, 종목코드: 해당 종목 뉴스
    pub fid_input_iscd: String,
    /// 공백 입력
    pub fid_titl_cntt: String,
    /// 공백: 현재, 또는 00YYYYMMDD
    pub fid_input_date_1: String,
    /// 공백: 현재, 또는 0000HHMMSS
    pub fid_input_hour_1: String,
    /// 공백 입력
    pub fid_rank_sort_cls_code: String,
    /// 공백 입력
    pub fid_input_srno: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewsItem {
    #[serde(default)]
    pub cntt_usiq_srno: String,
    #[serde(default)]
    pub news_ofer_entp_code: String,
    #[serde(default)]
    pub data_dt: String,
    #[serde(default)]
    pub data_tm: String,
    #[serde(default)]
    pub hts_pbnt_titl_cntt: String,
    #[serde(default)]
    pub news_lrdv_code: String,
    #[serde(default)]
    pub dorg: String,
    #[serde(default)]
    pub iscd1: String,
    #[serde(default)]
    pub iscd2: String,
    #[serde(default)]
    pub iscd3: String,
    #[serde(default)]
    pub iscd4: String,
    #[serde(default)]
    pub iscd5: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<NewsItem>> {
    let params = [
        ("FID_NEWS_OFER_ENTP_CODE", req.fid_news_ofer_entp_code.as_str()),
        ("FID_COND_MRKT_CLS_CODE", req.fid_cond_mrkt_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_TITL_CNTT", req.fid_titl_cntt.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_INPUT_SRNO", req.fid_input_srno.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    // output이 Array or single Object
    let items: Vec<NewsItem> = match output {
        serde_json::Value::Array(arr) => arr
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect(),
        v => serde_json::from_value::<NewsItem>(v)
            .map(|item| vec![item])
            .unwrap_or_default(),
    };
    Ok(items)
}
