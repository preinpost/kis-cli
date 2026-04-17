//! 해외주식 미국주간정정취소 — POST /uapi/overseas-stock/v1/trading/daytime-order-rvsecncl
//!
//! 스펙: .agent/specs/overseas_stock__order_account__daytime_order_rvsecncl.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/daytime-order-rvsecncl";
pub const TR_ID: &str = "TTTS6038U";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "OVRS_EXCG_CD")]
    pub ovrs_excg_cd: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    #[serde(rename = "OVRS_ORD_UNPR")]
    pub ovrs_ord_unpr: String,
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    #[serde(default, rename = "ODNO")]
    pub odno: String,
    #[serde(default, rename = "ORD_TMD")]
    pub ord_tmd: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 미국주간정정취소는 모의투자 미지원");
    }
    let resp: ApiResponse = client.post_json(ENDPOINT, TR_ID, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
