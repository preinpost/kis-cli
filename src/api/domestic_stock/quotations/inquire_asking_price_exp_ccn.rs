//! 주식현재가 호가/예상체결 — GET /uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_asking_price_exp_ccn.md
//!
//! output1(호가) + output2(예상체결). 각 호가는 1~10 레벨, 잔량, 증감.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn";
pub const TR_ID: &str = "FHKST01010200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AskingPrice {
    #[serde(default)]
    pub aspr_acpt_hour: String,
    #[serde(default)]
    pub askp1: String,
    #[serde(default)]
    pub askp2: String,
    #[serde(default)]
    pub askp3: String,
    #[serde(default)]
    pub askp4: String,
    #[serde(default)]
    pub askp5: String,
    #[serde(default)]
    pub askp6: String,
    #[serde(default)]
    pub askp7: String,
    #[serde(default)]
    pub askp8: String,
    #[serde(default)]
    pub askp9: String,
    #[serde(default)]
    pub askp10: String,
    #[serde(default)]
    pub bidp1: String,
    #[serde(default)]
    pub bidp2: String,
    #[serde(default)]
    pub bidp3: String,
    #[serde(default)]
    pub bidp4: String,
    #[serde(default)]
    pub bidp5: String,
    #[serde(default)]
    pub bidp6: String,
    #[serde(default)]
    pub bidp7: String,
    #[serde(default)]
    pub bidp8: String,
    #[serde(default)]
    pub bidp9: String,
    #[serde(default)]
    pub bidp10: String,
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
    pub askp_rsqn6: String,
    #[serde(default)]
    pub askp_rsqn7: String,
    #[serde(default)]
    pub askp_rsqn8: String,
    #[serde(default)]
    pub askp_rsqn9: String,
    #[serde(default)]
    pub askp_rsqn10: String,
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
    pub bidp_rsqn6: String,
    #[serde(default)]
    pub bidp_rsqn7: String,
    #[serde(default)]
    pub bidp_rsqn8: String,
    #[serde(default)]
    pub bidp_rsqn9: String,
    #[serde(default)]
    pub bidp_rsqn10: String,
    #[serde(default)]
    pub askp_rsqn_icdc1: String,
    #[serde(default)]
    pub askp_rsqn_icdc2: String,
    #[serde(default)]
    pub askp_rsqn_icdc3: String,
    #[serde(default)]
    pub askp_rsqn_icdc4: String,
    #[serde(default)]
    pub askp_rsqn_icdc5: String,
    #[serde(default)]
    pub askp_rsqn_icdc6: String,
    #[serde(default)]
    pub askp_rsqn_icdc7: String,
    #[serde(default)]
    pub askp_rsqn_icdc8: String,
    #[serde(default)]
    pub askp_rsqn_icdc9: String,
    #[serde(default)]
    pub askp_rsqn_icdc10: String,
    #[serde(default)]
    pub bidp_rsqn_icdc1: String,
    #[serde(default)]
    pub bidp_rsqn_icdc2: String,
    #[serde(default)]
    pub bidp_rsqn_icdc3: String,
    #[serde(default)]
    pub bidp_rsqn_icdc4: String,
    #[serde(default)]
    pub bidp_rsqn_icdc5: String,
    #[serde(default)]
    pub bidp_rsqn_icdc6: String,
    #[serde(default)]
    pub bidp_rsqn_icdc7: String,
    #[serde(default)]
    pub bidp_rsqn_icdc8: String,
    #[serde(default)]
    pub bidp_rsqn_icdc9: String,
    #[serde(default)]
    pub bidp_rsqn_icdc10: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub total_askp_rsqn_icdc: String,
    #[serde(default)]
    pub total_bidp_rsqn_icdc: String,
    #[serde(default)]
    pub ovtm_total_askp_icdc: String,
    #[serde(default)]
    pub ovtm_total_bidp_icdc: String,
    #[serde(default)]
    pub ovtm_total_askp_rsqn: String,
    #[serde(default)]
    pub ovtm_total_bidp_rsqn: String,
    #[serde(default)]
    pub ntby_aspr_rsqn: String,
    #[serde(default)]
    pub new_mkop_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpCcn {
    #[serde(default)]
    pub antc_mkop_cls_code: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub stck_sdpr: String,
    #[serde(default)]
    pub antc_cnpr: String,
    #[serde(default)]
    pub antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub antc_cntg_vrss: String,
    #[serde(default)]
    pub antc_cntg_prdy_ctrt: String,
    #[serde(default)]
    pub antc_vol: String,
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub vi_cls_code: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub asking: Option<AskingPrice>,
    pub exp_ccn: Option<ExpCcn>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let asking = resp
        .output1
        .and_then(|v| serde_json::from_value::<AskingPrice>(v).ok());
    let exp_ccn = resp
        .output2
        .and_then(|v| serde_json::from_value::<ExpCcn>(v).ok());
    Ok(Response { asking, exp_ccn })
}
