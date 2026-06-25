//! 퇴직연금 예수금조회 — GET /uapi/domestic-stock/v1/trading/pension/inquire-deposit
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_deposit.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/pension/inquire-deposit";
pub const TR_ID: &str = "TTTC0506R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    /// 29
    pub acnt_prdt_cd: String,
    /// 00
    pub acca_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub dnca_tota: String,
    #[serde(default)]
    pub nxdy_excc_amt: String,
    #[serde(default)]
    pub nxdy_sttl_amt: String,
    #[serde(default)]
    pub nx2_day_sttl_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("퇴직연금 예수금조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("ACCA_DVSN_CD", req.acca_dvsn_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
