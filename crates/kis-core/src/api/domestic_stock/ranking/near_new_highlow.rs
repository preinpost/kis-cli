//! 국내주식 신고/신저근접종목 상위 — GET /uapi/domestic-stock/v1/ranking/near-new-highlow
//!
//! 스펙: .agent/specs/domestic_stock__ranking__near_new_highlow.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/near-new-highlow";
pub const TR_ID: &str = "FHPST01870000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_aply_rang_vol: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_div_cls_code: String,
    pub fid_input_cnt_1: String,
    pub fid_input_cnt_2: String,
    pub fid_prc_cls_code: String,
    pub fid_input_iscd: String,
    pub fid_trgt_cls_code: String,
    pub fid_trgt_exls_cls_code: String,
    pub fid_aply_rang_prc_1: String,
    pub fid_aply_rang_prc_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub askp: String,
    #[serde(default)]
    pub askp_rsqn1: String,
    #[serde(default)]
    pub bidp: String,
    #[serde(default)]
    pub bidp_rsqn1: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub new_hgpr: String,
    #[serde(default)]
    pub hprc_near_rate: String,
    #[serde(default)]
    pub new_lwpr: String,
    #[serde(default)]
    pub lwpr_near_rate: String,
    #[serde(default)]
    pub stck_sdpr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내주식 신고/신저근접종목 상위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("fid_aply_rang_vol", req.fid_aply_rang_vol.as_str()),
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_cond_scr_div_code", req.fid_cond_scr_div_code.as_str()),
        ("fid_div_cls_code", req.fid_div_cls_code.as_str()),
        ("fid_input_cnt_1", req.fid_input_cnt_1.as_str()),
        ("fid_input_cnt_2", req.fid_input_cnt_2.as_str()),
        ("fid_prc_cls_code", req.fid_prc_cls_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_trgt_cls_code", req.fid_trgt_cls_code.as_str()),
        ("fid_trgt_exls_cls_code", req.fid_trgt_exls_cls_code.as_str()),
        ("fid_aply_rang_prc_1", req.fid_aply_rang_prc_1.as_str()),
        ("fid_aply_rang_prc_2", req.fid_aply_rang_prc_2.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
