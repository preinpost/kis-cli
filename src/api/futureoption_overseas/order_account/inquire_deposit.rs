//! 해외선물옵션 예수금현황 — GET /uapi/overseas-futureoption/v1/trading/inquire-deposit
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_deposit.md
//! 모의투자 미지원.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-deposit";
pub const TR_ID: &str = "OTFM1411R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub crcy_cd: String,
    pub inqr_dt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub fm_nxdy_dncl_amt: String,
    #[serde(default)]
    pub fm_tot_asst_evlu_amt: String,
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub resp_dt: String,
    #[serde(default)]
    pub fm_dnca_rmnd: String,
    #[serde(default)]
    pub fm_lqd_pfls_amt: String,
    #[serde(default)]
    pub fm_fee: String,
    #[serde(default)]
    pub fm_fuop_evlu_pfls_amt: String,
    #[serde(default)]
    pub fm_rcvb_amt: String,
    #[serde(default)]
    pub fm_brkg_mgn_amt: String,
    #[serde(default)]
    pub fm_mntn_mgn_amt: String,
    #[serde(default)]
    pub fm_add_mgn_amt: String,
    #[serde(default)]
    pub fm_risk_rt: String,
    #[serde(default)]
    pub fm_ord_psbl_amt: String,
    #[serde(default)]
    pub fm_drwg_psbl_amt: String,
    #[serde(default)]
    pub fm_echm_rqrm_amt: String,
    #[serde(default)]
    pub fm_drwg_prar_amt: String,
    #[serde(default)]
    pub fm_opt_tr_chgs: String,
    #[serde(default)]
    pub fm_opt_icld_asst_evlu_amt: String,
    #[serde(default)]
    pub fm_opt_evlu_amt: String,
    #[serde(default)]
    pub fm_crcy_sbst_amt: String,
    #[serde(default)]
    pub fm_crcy_sbst_use_amt: String,
    #[serde(default)]
    pub fm_crcy_sbst_stup_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 예수금현황은 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("CRCY_CD", req.crcy_cd.as_str()),
        ("INQR_DT", req.inqr_dt.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.context("응답에 output 없음")?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
