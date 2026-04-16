//! 퇴직연금 잔고조회 — GET /uapi/domestic-stock/v1/trading/pension/inquire-balance
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_balance_2.md
//!
//! 모의투자 미지원. 주식/ETF/ETN만 조회, 펀드는 제외.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/pension/inquire-balance";
pub const TR_ID: &str = "TTTC2208R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    /// 29
    pub acnt_prdt_cd: String,
    /// 00
    pub acca_dvsn_cd: String,
    /// 00 전체
    pub inqr_dvsn: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub cblc_dvsn_name: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub item_dvsn_name: String,
    #[serde(default)]
    pub thdt_buyqty: String,
    #[serde(default)]
    pub thdt_sll_qty: String,
    #[serde(default)]
    pub hldg_qty: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
    #[serde(default)]
    pub pchs_amt: String,
    #[serde(default)]
    pub prpr: String,
    #[serde(default)]
    pub evlu_amt: String,
    #[serde(default)]
    pub evlu_pfls_amt: String,
    #[serde(default)]
    pub evlu_erng_rt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub dnca_tot_amt: String,
    #[serde(default)]
    pub nxdy_excc_amt: String,
    #[serde(default)]
    pub prvs_rcdl_excc_amt: String,
    #[serde(default)]
    pub thdt_buy_amt: String,
    #[serde(default)]
    pub thdt_sll_amt: String,
    #[serde(default)]
    pub thdt_tlex_amt: String,
    #[serde(default)]
    pub scts_evlu_amt: String,
    #[serde(default)]
    pub tot_evlu_amt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("퇴직연금 잔고조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("ACCA_DVSN_CD", req.acca_dvsn_cd.as_str()),
        ("INQR_DVSN", req.inqr_dvsn.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let holdings: Vec<Holding> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    // output2는 스펙에 Object(single)
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { holdings, summary })
}
