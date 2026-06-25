//! 국내주식 시가총액 상위 — GET /uapi/domestic-stock/v1/ranking/market-cap
//!
//! 스펙: .agent/specs/domestic_stock__ranking__market_cap.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/market-cap";
pub const TR_ID: &str = "FHPST01740000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_input_price_2: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_div_cls_code: String,
    pub fid_input_iscd: String,
    pub fid_trgt_cls_code: String,
    pub fid_trgt_exls_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_vol_cnt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    #[serde(default)]
    pub data_rank: String,
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
    pub acml_vol: String,
    #[serde(default)]
    pub lstn_stcn: String,
    #[serde(default)]
    pub stck_avls: String,
    #[serde(default)]
    pub mrkt_whol_avls_rlim: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내주식 시가총액 상위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("fid_input_price_2", req.fid_input_price_2.as_str()),
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_cond_scr_div_code", req.fid_cond_scr_div_code.as_str()),
        ("fid_div_cls_code", req.fid_div_cls_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_trgt_cls_code", req.fid_trgt_cls_code.as_str()),
        ("fid_trgt_exls_cls_code", req.fid_trgt_exls_cls_code.as_str()),
        ("fid_input_price_1", req.fid_input_price_1.as_str()),
        ("fid_vol_cnt", req.fid_vol_cnt.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
