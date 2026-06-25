//! 선물옵션 시세 — GET /uapi/domestic-futureoption/v1/quotations/inquire-price
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__inquire_price.md
//! 응답에 output3가 있으나 ApiResponse 래퍼는 output1/output2만 노출 → output3는 무시.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/inquire-price";
pub const TR_ID: &str = "FHMIF10000000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quote {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub futs_prpr: String,
    #[serde(default)]
    pub futs_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub futs_prdy_clpr: String,
    #[serde(default)]
    pub futs_prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub hts_otst_stpl_qty: String,
    #[serde(default)]
    pub otst_stpl_qty_icdc: String,
    #[serde(default)]
    pub futs_oprc: String,
    #[serde(default)]
    pub futs_hgpr: String,
    #[serde(default)]
    pub futs_lwpr: String,
    #[serde(default)]
    pub futs_mxpr: String,
    #[serde(default)]
    pub futs_llam: String,
    #[serde(default)]
    pub basis: String,
    #[serde(default)]
    pub futs_sdpr: String,
    #[serde(default)]
    pub hts_thpr: String,
    #[serde(default)]
    pub dprt: String,
    #[serde(default)]
    pub crbr_aply_mxpr: String,
    #[serde(default)]
    pub crbr_aply_llam: String,
    #[serde(default)]
    pub futs_last_tr_date: String,
    #[serde(default)]
    pub hts_rmnn_dynu: String,
    #[serde(default)]
    pub futs_lstn_medm_hgpr: String,
    #[serde(default)]
    pub futs_lstn_medm_lwpr: String,
    #[serde(default)]
    pub delta_val: String,
    #[serde(default)]
    pub gama: String,
    #[serde(default)]
    pub theta: String,
    #[serde(default)]
    pub vega: String,
    #[serde(default)]
    pub rho: String,
    #[serde(default)]
    pub hist_vltl: String,
    #[serde(default)]
    pub hts_ints_vltl: String,
    #[serde(default)]
    pub mrkt_basis: String,
    #[serde(default)]
    pub acpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndexInfo {
    #[serde(default)]
    pub bstp_cls_code: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub quote: Option<Quote>,
    pub index: Option<IndexInfo>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let quote = resp
        .output1
        .and_then(|v| serde_json::from_value::<Quote>(v).ok());
    let index = resp
        .output2
        .and_then(|v| serde_json::from_value::<IndexInfo>(v).ok());
    Ok(Response { quote, index })
}
