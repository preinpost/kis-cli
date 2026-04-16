//! 주식주문(정정취소) — POST /uapi/domestic-stock/v1/trading/order-rvsecncl
//!
//! 스펙: .agent/specs/domestic_stock__order_account__order_rvsecncl.md
//!
//! 정정(01) / 취소(02)는 `RVSE_CNCL_DVSN_CD` 필드로 구분. 매수/매도 구분 없음.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/order-rvsecncl";
pub const TR_ID_REAL: &str = "TTTC0013U";
pub const TR_ID_MOCK: &str = "VTTC0013U";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "KRX_FWDG_ORD_ORGNO")]
    pub krx_fwdg_ord_orgno: String,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    /// 01=정정, 02=취소
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    /// Y=전량, N=일부
    #[serde(rename = "QTY_ALL_ORD_YN")]
    pub qty_all_ord_yn: String,
    /// 스탑지정가(ORD_DVSN=22) 사용 시 필수
    #[serde(rename = "CNDT_PRIC", skip_serializing_if = "Option::is_none")]
    pub cndt_pric: Option<String>,
    /// KRX/NXT/SOR — 미입력 시 KRX
    #[serde(rename = "EXCG_ID_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub excg_id_dvsn_cd: Option<String>,
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
    let tr_id = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let resp = client.post_json(ENDPOINT, tr_id, req, &[]).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
