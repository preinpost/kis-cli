//! 장내채권 매도주문 — POST /uapi/domestic-bond/v1/trading/sell
//!
//! 스펙: .agent/specs/bond__order_account__sell.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/trading/sell";
pub const TR_ID: &str = "TTTC0958U";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    #[serde(rename = "ORD_QTY2")]
    pub ord_qty2: String,
    #[serde(rename = "BOND_ORD_UNPR")]
    pub bond_ord_unpr: String,
    #[serde(rename = "SPRX_YN")]
    pub sprx_yn: String,
    #[serde(rename = "BUY_DT")]
    pub buy_dt: String,
    #[serde(rename = "BUY_SEQ")]
    pub buy_seq: String,
    #[serde(rename = "SAMT_MKET_PTCI_YN")]
    pub samt_mket_ptci_yn: String,
    #[serde(rename = "SLL_AGCO_OPPS_SLL_YN")]
    pub sll_agco_opps_sll_yn: String,
    #[serde(rename = "BOND_RTL_MKET_YN")]
    pub bond_rtl_mket_yn: String,
    #[serde(rename = "MGCO_APTM_ODNO")]
    pub mgco_aptm_odno: String,
    #[serde(rename = "ORD_SVR_DVSN_CD")]
    pub ord_svr_dvsn_cd: String,
    #[serde(rename = "CTAC_TLNO")]
    pub ctac_tlno: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub krx_fwdg_ord_orgno: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub ord_tmd: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 매도주문은 모의투자 미지원 API입니다");
    }
    let resp: ApiResponse = client.post_json(ENDPOINT, TR_ID, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
