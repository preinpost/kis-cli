//! 선물옵션기간약정수수료일별 — GET /uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_daily_amount_fee.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-daily-amount-fee";
pub const TR_ID: &str = "CTFO6119R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub inqr_strt_day: String,
    pub inqr_end_day: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Daily {
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub item_name: String,
    #[serde(default)]
    pub sll_agrm_amt: String,
    #[serde(default)]
    pub sll_fee: String,
    #[serde(default)]
    pub buy_agrm_amt: String,
    #[serde(default)]
    pub buy_fee: String,
    #[serde(default)]
    pub tot_fee_smtl: String,
    #[serde(default)]
    pub trad_pfls: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub futr_agrm: String,
    #[serde(default)]
    pub futr_agrm_amt: String,
    #[serde(default)]
    pub futr_agrm_amt_smtl: String,
    #[serde(default)]
    pub futr_sll_fee_smtl: String,
    #[serde(default)]
    pub futr_buy_fee_smtl: String,
    #[serde(default)]
    pub futr_fee_smtl: String,
    #[serde(default)]
    pub opt_agrm: String,
    #[serde(default)]
    pub opt_agrm_amt: String,
    #[serde(default)]
    pub opt_agrm_amt_smtl: String,
    #[serde(default)]
    pub opt_sll_fee_smtl: String,
    #[serde(default)]
    pub opt_buy_fee_smtl: String,
    #[serde(default)]
    pub opt_fee_smtl: String,
    #[serde(default)]
    pub prdt_futr_agrm: String,
    #[serde(default)]
    pub prdt_fuop: String,
    #[serde(default)]
    pub prdt_futr_evlu_amt: String,
    #[serde(default)]
    pub futr_fee: String,
    #[serde(default)]
    pub opt_fee: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub sll_agrm_amt: String,
    #[serde(default)]
    pub buy_agrm_amt: String,
    #[serde(default)]
    pub agrm_amt_smtl: String,
    #[serde(default)]
    pub sll_fee: String,
    #[serde(default)]
    pub buy_fee: String,
    #[serde(default)]
    pub fee_smtl: String,
    #[serde(default)]
    pub trad_pfls_smtl: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub daily: Vec<Daily>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("선물옵션기간약정수수료일별은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_STRT_DAY", req.inqr_strt_day.as_str()),
        ("INQR_END_DAY", req.inqr_end_day.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let daily = resp
        .output1
        .map(serde_json::from_value::<Vec<Daily>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { daily, summary })
}
