//! 퇴직연금 체결기준잔고 — GET /uapi/domestic-stock/v1/trading/pension/inquire-present-balance
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_present_balance.md
//!
//! 모의투자 미지원. 55번 계좌(DC가입자) 사용 불가.
//! output1(Vec<Holding>) + output2(Vec<Summary>, 통상 단일).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/pension/inquire-present-balance";
pub const TR_ID: &str = "TTTC2202R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    /// 29
    pub acnt_prdt_cd: String,
    /// 00
    pub user_dvsn_cd: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
    /// 00 전체, 01 0주 숨김
    pub prcs_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub cblc_dvsn: String,
    #[serde(default)]
    pub cblc_dvsn_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub hldg_qty: String,
    #[serde(default)]
    pub slpsb_qty: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
    #[serde(default)]
    pub evlu_pfls_amt: String,
    #[serde(default)]
    pub evlu_pfls_rt: String,
    #[serde(default)]
    pub prpr: String,
    #[serde(default)]
    pub evlu_amt: String,
    #[serde(default)]
    pub pchs_amt: String,
    #[serde(default)]
    pub cblc_weit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub pchs_amt_smtl_amt: String,
    #[serde(default)]
    pub evlu_amt_smtl_amt: String,
    #[serde(default)]
    pub evlu_pfls_smtl_amt: String,
    #[serde(default)]
    pub trad_pfls_smtl: String,
    #[serde(default)]
    pub thdt_tot_pfls_amt: String,
    #[serde(default)]
    pub pftrt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("퇴직연금 체결기준잔고는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("USER_DVSN_CD", req.user_dvsn_cd.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
        ("PRCS_DVSN_CD", req.prcs_dvsn_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let holdings: Vec<Holding> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    let summary = resp.output2.and_then(|v| {
        serde_json::from_value::<Vec<Summary>>(v)
            .ok()
            .and_then(|mut arr| arr.pop())
    });
    Ok(Response { holdings, summary })
}
