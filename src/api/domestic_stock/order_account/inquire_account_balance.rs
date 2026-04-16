//! 투자계좌자산현황조회 — GET /uapi/domestic-stock/v1/trading/inquire-account-balance
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_account_balance.md
//!
//! 모의투자 미지원. 스펙상 `Output1`, `Output2`로 대문자 시작인데 실제 응답은 보통 소문자 —
//! `ApiResponse`의 output1/output2로 매칭 시도, 대소문자 불일치면 빈 값이 됨.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-account-balance";
pub const TR_ID: &str = "CTRP6548R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    /// 공백입력
    pub inqr_dvsn_1: String,
    /// 공백입력
    pub bspr_bf_dt_aply_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetRow {
    #[serde(default)]
    pub pchs_amt: String,
    #[serde(default)]
    pub evlu_amt: String,
    #[serde(default)]
    pub evlu_pfls_amt: String,
    #[serde(default)]
    pub crdt_lnd_amt: String,
    #[serde(default)]
    pub real_nass_amt: String,
    #[serde(default)]
    pub whol_weit_rt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub pchs_amt_smtl: String,
    #[serde(default)]
    pub nass_tot_amt: String,
    #[serde(default)]
    pub loan_amt_smtl: String,
    #[serde(default)]
    pub evlu_pfls_amt_smtl: String,
    #[serde(default)]
    pub evlu_amt_smtl: String,
    #[serde(default)]
    pub tot_asst_amt: String,
    #[serde(default)]
    pub tot_lnda_tot_ulst_lnda: String,
    #[serde(default)]
    pub cma_auto_loan_amt: String,
    #[serde(default)]
    pub tot_mgln_amt: String,
    #[serde(default)]
    pub stln_evlu_amt: String,
    #[serde(default)]
    pub crdt_fncg_amt: String,
    #[serde(default)]
    pub ocl_apl_loan_amt: String,
    #[serde(default)]
    pub pldg_stup_amt: String,
    #[serde(default)]
    pub frcr_evlu_tota: String,
    #[serde(default)]
    pub tot_dncl_amt: String,
    #[serde(default)]
    pub cma_evlu_amt: String,
    #[serde(default)]
    pub dncl_amt: String,
    #[serde(default)]
    pub tot_sbst_amt: String,
    #[serde(default)]
    pub thdt_rcvb_amt: String,
    #[serde(default)]
    pub ovrs_stck_evlu_amt1: String,
    #[serde(default)]
    pub ovrs_bond_evlu_amt: String,
    #[serde(default)]
    pub mmf_cma_mgge_loan_amt: String,
    #[serde(default)]
    pub sbsc_dncl_amt: String,
    #[serde(default)]
    pub pbst_sbsc_fnds_loan_use_amt: String,
    #[serde(default)]
    pub etpr_crdt_grnt_loan_amt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub assets: Vec<AssetRow>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("투자계좌자산현황조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_DVSN_1", req.inqr_dvsn_1.as_str()),
        ("BSPR_BF_DT_APLY_YN", req.bspr_bf_dt_aply_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let assets: Vec<AssetRow> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { assets, summary })
}
