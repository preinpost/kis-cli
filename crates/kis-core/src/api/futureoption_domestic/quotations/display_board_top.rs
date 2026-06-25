//! 국내선물 기초자산 시세 — GET /uapi/domestic-futureoption/v1/quotations/display-board-top
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__display_board_top.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/display-board-top";
pub const TR_ID: &str = "FHPIF05030000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_cond_mrkt_div_code1: String,
    pub fid_cond_scr_div_code: String,
    pub fid_mtrt_cnt: String,
    pub fid_cond_mrkt_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Header {
    #[serde(default)]
    pub unas_prpr: String,
    #[serde(default)]
    pub unas_prdy_vrss: String,
    #[serde(default)]
    pub unas_prdy_vrss_sign: String,
    #[serde(default)]
    pub unas_prdy_ctrt: String,
    #[serde(default)]
    pub unas_acml_vol: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub futs_prpr: String,
    #[serde(default)]
    pub futs_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub futs_prdy_ctrt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Maturity {
    #[serde(default)]
    pub hts_rmnn_dynu: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub header: Option<Header>,
    pub maturities: Vec<Maturity>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내선물 기초자산 시세는 모의투자 미지원");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_MRKT_DIV_CODE1", req.fid_cond_mrkt_div_code1.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_MTRT_CNT", req.fid_mtrt_cnt.as_str()),
        ("FID_COND_MRKT_CLS_CODE", req.fid_cond_mrkt_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let header = resp
        .output1
        .and_then(|v| serde_json::from_value::<Header>(v).ok());
    let maturities = resp
        .output2
        .map(serde_json::from_value::<Vec<Maturity>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { header, maturities })
}
