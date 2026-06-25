//! 국내주식 장마감 예상체결가 — GET /uapi/domestic-stock/v1/quotations/exp-closing-price
//!
//! 스펙: .agent/specs/domestic_stock__quotations__exp_closing_price.md
//!
//! 모의투자 미지원. output1이 Array.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/exp-closing-price";
pub const TR_ID: &str = "FHKST117300C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 0 전체, 1 상한가마감, 2 하한가마감, 3 상승률상위, 4 하락률상위
    pub fid_rank_sort_cls_code: String,
    /// J 주식
    pub fid_cond_mrkt_div_code: String,
    /// Unique key 11173
    pub fid_cond_scr_div_code: String,
    /// 0000 전체, 0001 거래소, 1001 코스닥, 2001 코스피200, 4001 KRX100
    pub fid_input_iscd: String,
    /// 0 전체, 1 종가범위연장
    pub fid_blng_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub sdpr_vrss_prpr: String,
    #[serde(default)]
    pub sdpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub cntg_vol: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내주식 장마감 예상체결가는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
