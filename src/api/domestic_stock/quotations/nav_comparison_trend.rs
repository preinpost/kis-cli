//! NAV 비교추이(종목) — GET /uapi/etfetn/v1/quotations/nav-comparison-trend
//!
//! 스펙: .agent/specs/domestic_stock__quotations__nav_comparison_trend.md
//!
//! 모의투자 미지원. output1(주가) + output2(NAV).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/etfetn/v1/quotations/nav-comparison-trend";
pub const TR_ID: &str = "FHPST02440000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Price {
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
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub stck_prdy_clpr: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub stck_mxpr: String,
    #[serde(default)]
    pub stck_llam: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Nav {
    #[serde(default)]
    pub nav: String,
    #[serde(default)]
    pub nav_prdy_vrss_sign: String,
    #[serde(default)]
    pub nav_prdy_vrss: String,
    #[serde(default)]
    pub nav_prdy_ctrt: String,
    #[serde(default)]
    pub prdy_clpr_nav: String,
    #[serde(default)]
    pub oprc_nav: String,
    #[serde(default)]
    pub hprc_nav: String,
    #[serde(default)]
    pub lprc_nav: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub price: Option<Price>,
    pub nav: Option<Nav>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("NAV 비교추이(종목)는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let price = resp
        .output1
        .and_then(|v| serde_json::from_value::<Price>(v).ok());
    let nav = resp
        .output2
        .and_then(|v| serde_json::from_value::<Nav>(v).ok());
    Ok(Response { price, nav })
}
