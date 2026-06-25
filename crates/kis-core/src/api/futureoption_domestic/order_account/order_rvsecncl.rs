//! 선물옵션 정정취소주문 — POST /uapi/domestic-futureoption/v1/trading/order-rvsecncl
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__order_rvsecncl.md
//! 실전 주간 TTTO1103U / 야간 STTN1103U. 모의 주간 VTTO1103U (야간 미지원).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/order-rvsecncl";
pub const TR_ID_REAL_DAY: &str = "TTTO1103U";
pub const TR_ID_REAL_NIGHT: &str = "STTN1103U";
pub const TR_ID_MOCK_DAY: &str = "VTTO1103U";

#[derive(Debug, Clone, Copy)]
pub enum Session {
    Day,
    Night,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "ORD_PRCS_DVSN_CD")]
    pub ord_prcs_dvsn_cd: String,
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "RVSE_CNCL_DVSN_CD")]
    pub rvse_cncl_dvsn_cd: String,
    #[serde(rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    #[serde(rename = "UNIT_PRICE")]
    pub unit_price: String,
    #[serde(rename = "NMPR_TYPE_CD")]
    pub nmpr_type_cd: String,
    #[serde(rename = "KRX_NMPR_CNDT_CD")]
    pub krx_nmpr_cndt_cd: String,
    #[serde(rename = "RMN_QTY_YN")]
    pub rmn_qty_yn: String,
    #[serde(rename = "ORD_DVSN_CD")]
    pub ord_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default, rename = "ACNT_NAME")]
    pub acnt_name: String,
    #[serde(default, rename = "TRAD_DVSN_NAME")]
    pub trad_dvsn_name: String,
    #[serde(default, rename = "ITEM_NAME")]
    pub item_name: String,
    #[serde(default, rename = "ORD_TMD")]
    pub ord_tmd: String,
    #[serde(default, rename = "ORD_GNO_BRNO")]
    pub ord_gno_brno: String,
    #[serde(default, rename = "ORGN_ODNO")]
    pub orgn_odno: String,
    #[serde(default, rename = "ODNO")]
    pub odno: String,
}

pub async fn call(client: &KisClient, session: Session, req: &Request) -> Result<Response> {
    let tr_id = match (client.is_mock(), session) {
        (true, Session::Day) => TR_ID_MOCK_DAY,
        (true, Session::Night) => bail!("선물옵션 야간 정정취소는 모의투자 미지원"),
        (false, Session::Day) => TR_ID_REAL_DAY,
        (false, Session::Night) => TR_ID_REAL_NIGHT,
    };
    let resp: ApiResponse = client.post_json(ENDPOINT, tr_id, req, &[]).await?;
    let output = resp
        .output
        .ok_or_else(|| anyhow::anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
