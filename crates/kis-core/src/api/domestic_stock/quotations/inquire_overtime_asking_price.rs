//! 국내주식 시간외호가 — GET /uapi/domestic-stock/v1/quotations/inquire-overtime-asking-price
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_overtime_asking_price.md
//!
//! 모의투자 미지원. 1~10레벨 호가 + 잔량 + 증감.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-overtime-asking-price";
pub const TR_ID: &str = "FHPST02300400";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_input_iscd: String,
    pub fid_cond_mrkt_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub ovtm_untp_last_hour: String,
    #[serde(default)]
    pub ovtm_untp_askp1: String,
    #[serde(default)]
    pub ovtm_untp_askp2: String,
    #[serde(default)]
    pub ovtm_untp_askp3: String,
    #[serde(default)]
    pub ovtm_untp_askp4: String,
    #[serde(default)]
    pub ovtm_untp_askp5: String,
    #[serde(default)]
    pub ovtm_untp_askp6: String,
    #[serde(default)]
    pub ovtm_untp_askp7: String,
    #[serde(default)]
    pub ovtm_untp_askp8: String,
    #[serde(default)]
    pub ovtm_untp_askp9: String,
    #[serde(default)]
    pub ovtm_untp_askp10: String,
    #[serde(default)]
    pub ovtm_untp_bidp1: String,
    #[serde(default)]
    pub ovtm_untp_bidp2: String,
    #[serde(default)]
    pub ovtm_untp_bidp3: String,
    #[serde(default)]
    pub ovtm_untp_bidp4: String,
    #[serde(default)]
    pub ovtm_untp_bidp5: String,
    #[serde(default)]
    pub ovtm_untp_bidp6: String,
    #[serde(default)]
    pub ovtm_untp_bidp7: String,
    #[serde(default)]
    pub ovtm_untp_bidp8: String,
    #[serde(default)]
    pub ovtm_untp_bidp9: String,
    #[serde(default)]
    pub ovtm_untp_bidp10: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc1: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc2: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc3: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc4: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc5: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc6: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc7: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc8: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc9: String,
    #[serde(default)]
    pub ovtm_untp_askp_icdc10: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc1: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc2: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc3: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc4: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc5: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc6: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc7: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc8: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc9: String,
    #[serde(default)]
    pub ovtm_untp_bidp_icdc10: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn1: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn2: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn3: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn4: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn5: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn6: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn7: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn8: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn9: String,
    #[serde(default)]
    pub ovtm_untp_askp_rsqn10: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn1: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn2: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn3: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn4: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn5: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn6: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn7: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn8: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn9: String,
    #[serde(default)]
    pub ovtm_untp_bidp_rsqn10: String,
    #[serde(default)]
    pub ovtm_untp_total_askp_rsqn: String,
    #[serde(default)]
    pub ovtm_untp_total_bidp_rsqn: String,
    #[serde(default)]
    pub ovtm_untp_total_askp_rsqn_icdc: String,
    #[serde(default)]
    pub ovtm_untp_total_bidp_rsqn_icdc: String,
    #[serde(default)]
    pub ovtm_untp_ntby_bidp_rsqn: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub total_askp_rsqn_icdc: String,
    #[serde(default)]
    pub total_bidp_rsqn_icdc: String,
    #[serde(default)]
    pub ovtm_total_askp_rsqn: String,
    #[serde(default)]
    pub ovtm_total_bidp_rsqn: String,
    #[serde(default)]
    pub ovtm_total_askp_icdc: String,
    #[serde(default)]
    pub ovtm_total_bidp_icdc: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내주식 시간외호가는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
