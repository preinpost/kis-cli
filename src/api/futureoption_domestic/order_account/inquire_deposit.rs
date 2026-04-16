//! 선물옵션 총자산현황 — GET /uapi/domestic-futureoption/v1/trading/inquire-deposit
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_deposit.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-deposit";
pub const TR_ID: &str = "CTRP6550R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub dnca_tota: String,
    #[serde(default)]
    pub bfdy_chck_amt: String,
    #[serde(default)]
    pub thdt_chck_amt: String,
    #[serde(default)]
    pub rlth_uwdl_dpos_amt: String,
    #[serde(default)]
    pub brkg_mgna_cash: String,
    #[serde(default)]
    pub wdrw_psbl_tot_amt: String,
    #[serde(default)]
    pub ord_psbl_cash: String,
    #[serde(default)]
    pub ord_psbl_tota: String,
    #[serde(default)]
    pub dnca_sbst: String,
    #[serde(default)]
    pub scts_sbst_amt: String,
    #[serde(default)]
    pub frcr_evlu_amt: String,
    #[serde(default)]
    pub brkg_mgna_sbst: String,
    #[serde(default)]
    pub sbst_rlse_psbl_amt: String,
    #[serde(default)]
    pub mtnc_rt: String,
    #[serde(default)]
    pub add_mgna_tota: String,
    #[serde(default)]
    pub add_mgna_cash: String,
    #[serde(default)]
    pub rcva: String,
    #[serde(default)]
    pub futr_trad_pfls: String,
    #[serde(default)]
    pub opt_trad_pfls_amt: String,
    #[serde(default)]
    pub trad_pfls_smtl: String,
    #[serde(default)]
    pub futr_evlu_pfls_amt: String,
    #[serde(default)]
    pub opt_evlu_pfls_amt: String,
    #[serde(default)]
    pub evlu_pfls_smtl: String,
    #[serde(default)]
    pub excc_dfpa: String,
    #[serde(default)]
    pub opt_dfpa: String,
    #[serde(default)]
    pub brkg_fee: String,
    #[serde(default)]
    pub nxdy_dnca: String,
    #[serde(default)]
    pub prsm_dpast_amt: String,
    #[serde(default)]
    pub cash_mntn_amt: String,
    #[serde(default)]
    pub hack_acdt_acnt_move_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("선물옵션 총자산현황은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
