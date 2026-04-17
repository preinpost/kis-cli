//! 해외속보(제목) — GET /uapi/overseas-price/v1/quotations/brknews-title
//!
//! 스펙: .agent/specs/overseas_stock__market_analysis__brknews_title.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/brknews-title";
pub const TR_ID: &str = "FHKST01011801";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_news_ofer_entp_code: String,
    pub fid_cond_mrkt_cls_code: String,
    pub fid_input_iscd: String,
    pub fid_titl_cntt: String,
    pub fid_input_date_1: String,
    pub fid_input_hour_1: String,
    pub fid_rank_sort_cls_code: String,
    pub fid_input_srno: String,
    pub fid_cond_scr_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct News {
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
    #[serde(default)]
    pub iscd6: String,
    #[serde(default)]
    pub iscd7: String,
    #[serde(default)]
    pub iscd8: String,
    #[serde(default)]
    pub iscd9: String,
    #[serde(default)]
    pub iscd10: String,
    #[serde(default)]
    pub kor_isnm1: String,
    #[serde(default)]
    pub kor_isnm2: String,
    #[serde(default)]
    pub kor_isnm3: String,
    #[serde(default)]
    pub kor_isnm4: String,
    #[serde(default)]
    pub kor_isnm5: String,
    #[serde(default)]
    pub kor_isnm6: String,
    #[serde(default)]
    pub kor_isnm7: String,
    #[serde(default)]
    pub kor_isnm8: String,
    #[serde(default)]
    pub kor_isnm9: String,
    #[serde(default)]
    pub kor_isnm10: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<News>> {
    if client.is_mock() {
        bail!("해외속보(제목)은 모의투자 미지원");
    }
    let params = [
        ("FID_NEWS_OFER_ENTP_CODE", req.fid_news_ofer_entp_code.as_str()),
        ("FID_COND_MRKT_CLS_CODE", req.fid_cond_mrkt_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_TITL_CNTT", req.fid_titl_cntt.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_INPUT_SRNO", req.fid_input_srno.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let news: Vec<News> = serde_json::from_value(output)?;
    Ok(news)
}
