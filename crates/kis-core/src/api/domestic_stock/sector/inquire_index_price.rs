//! 국내업종 현재지수 — GET /uapi/domestic-stock/v1/quotations/inquire-index-price
//!
//! 스펙: .agent/specs/domestic_stock__sector__inquire_index_price.md
//!
//! 모의투자 미지원. 업종 시장구분(U), 업종코드(ex 0001 코스피, 1001 코스닥, 2001 코스피200).

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-index-price";
pub const TR_ID: &str = "FHPUP02100000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// U 고정
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub prdy_tr_pbmn: String,
    #[serde(default)]
    pub bstp_nmix_oprc: String,
    #[serde(default)]
    pub prdy_nmix_vrss_nmix_oprc: String,
    #[serde(default)]
    pub oprc_vrss_prpr_sign: String,
    #[serde(default)]
    pub bstp_nmix_oprc_prdy_ctrt: String,
    #[serde(default)]
    pub bstp_nmix_hgpr: String,
    #[serde(default)]
    pub prdy_nmix_vrss_nmix_hgpr: String,
    #[serde(default)]
    pub hgpr_vrss_prpr_sign: String,
    #[serde(default)]
    pub bstp_nmix_hgpr_prdy_ctrt: String,
    #[serde(default)]
    pub bstp_nmix_lwpr: String,
    #[serde(default)]
    pub prdy_clpr_vrss_lwpr: String,
    #[serde(default)]
    pub lwpr_vrss_prpr_sign: String,
    #[serde(default)]
    pub prdy_clpr_vrss_lwpr_rate: String,
    #[serde(default)]
    pub ascn_issu_cnt: String,
    #[serde(default)]
    pub uplm_issu_cnt: String,
    #[serde(default)]
    pub stnr_issu_cnt: String,
    #[serde(default)]
    pub down_issu_cnt: String,
    #[serde(default)]
    pub lslm_issu_cnt: String,
    #[serde(default)]
    pub dryy_bstp_nmix_hgpr: String,
    #[serde(default)]
    pub dryy_hgpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub dryy_bstp_nmix_hgpr_date: String,
    #[serde(default)]
    pub dryy_bstp_nmix_lwpr: String,
    #[serde(default)]
    pub dryy_lwpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub dryy_bstp_nmix_lwpr_date: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub seln_rsqn_rate: String,
    #[serde(default)]
    pub shnu_rsqn_rate: String,
    #[serde(default)]
    pub ntby_rsqn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내업종 현재지수는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
