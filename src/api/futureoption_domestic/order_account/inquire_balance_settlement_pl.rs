//! 선물옵션 잔고정산손익내역 — GET /uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_balance_settlement_pl.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-balance-settlement-pl";
pub const TR_ID: &str = "CTFO6117R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub inqr_dt: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub trad_dvsn_name: String,
    #[serde(default)]
    pub bfdy_cblc_qty: String,
    #[serde(default)]
    pub new_qty: String,
    #[serde(default)]
    pub mnpl_rpch_qty: String,
    #[serde(default)]
    pub cblc_qty: String,
    #[serde(default)]
    pub cblc_amt: String,
    #[serde(default)]
    pub trad_pfls_amt: String,
    #[serde(default)]
    pub evlu_amt: String,
    #[serde(default)]
    pub evlu_pfls_amt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub nxdy_dnca: String,
    #[serde(default)]
    pub mmga_cash: String,
    #[serde(default)]
    pub brkg_mgna_cash: String,
    #[serde(default)]
    pub opt_buy_chgs: String,
    #[serde(default)]
    pub opt_lqd_evlu_amt: String,
    #[serde(default)]
    pub dnca_sbst: String,
    #[serde(default)]
    pub mmga_tota: String,
    #[serde(default)]
    pub brkg_mgna_tota: String,
    #[serde(default)]
    pub opt_sll_chgs: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub thdt_dfpa: String,
    #[serde(default)]
    pub rnwl_dfpa: String,
    #[serde(default)]
    pub dnca_cash: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("선물옵션 잔고정산손익내역은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_DT", req.inqr_dt.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let holdings = resp
        .output1
        .map(serde_json::from_value::<Vec<Holding>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { holdings, summary })
}
