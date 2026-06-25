//! 선물옵션 시세호가 — GET /uapi/domestic-futureoption/v1/quotations/inquire-asking-price
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__inquire_asking_price.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/inquire-asking-price";
pub const TR_ID: &str = "FHMIF10010000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub futs_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub futs_prdy_vrss: String,
    #[serde(default)]
    pub futs_prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub futs_prdy_clpr: String,
    #[serde(default)]
    pub futs_shrn_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quote {
    #[serde(default)]
    pub futs_askp1: String,
    #[serde(default)]
    pub futs_askp2: String,
    #[serde(default)]
    pub futs_askp3: String,
    #[serde(default)]
    pub futs_askp4: String,
    #[serde(default)]
    pub futs_askp5: String,
    #[serde(default)]
    pub futs_bidp1: String,
    #[serde(default)]
    pub futs_bidp2: String,
    #[serde(default)]
    pub futs_bidp3: String,
    #[serde(default)]
    pub futs_bidp4: String,
    #[serde(default)]
    pub futs_bidp5: String,
    #[serde(default)]
    pub askp_rsqn1: String,
    #[serde(default)]
    pub askp_rsqn2: String,
    #[serde(default)]
    pub askp_rsqn3: String,
    #[serde(default)]
    pub askp_rsqn4: String,
    #[serde(default)]
    pub askp_rsqn5: String,
    #[serde(default)]
    pub bidp_rsqn1: String,
    #[serde(default)]
    pub bidp_rsqn2: String,
    #[serde(default)]
    pub bidp_rsqn3: String,
    #[serde(default)]
    pub bidp_rsqn4: String,
    #[serde(default)]
    pub bidp_rsqn5: String,
    #[serde(default)]
    pub askp_csnu1: String,
    #[serde(default)]
    pub askp_csnu2: String,
    #[serde(default)]
    pub askp_csnu3: String,
    #[serde(default)]
    pub askp_csnu4: String,
    #[serde(default)]
    pub askp_csnu5: String,
    #[serde(default)]
    pub bidp_csnu1: String,
    #[serde(default)]
    pub bidp_csnu2: String,
    #[serde(default)]
    pub bidp_csnu3: String,
    #[serde(default)]
    pub bidp_csnu4: String,
    #[serde(default)]
    pub bidp_csnu5: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub total_askp_csnu: String,
    #[serde(default)]
    pub total_bidp_csnu: String,
    #[serde(default)]
    pub aspr_acpt_hour: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub quotes: Vec<Quote>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let quotes = resp
        .output2
        .map(serde_json::from_value::<Vec<Quote>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, quotes })
}
