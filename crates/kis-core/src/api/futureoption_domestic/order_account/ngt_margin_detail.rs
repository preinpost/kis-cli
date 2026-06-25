//! (야간)선물옵션 증거금 상세 — GET /uapi/domestic-futureoption/v1/trading/ngt-margin-detail
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__ngt_margin_detail.md
//! 모의투자 미지원. 신 TR_ID `CTFN7107R` 기준.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/ngt-margin-detail";
pub const TR_ID: &str = "CTFN7107R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub mgna_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MarginRow {
    #[serde(default)]
    pub futr_new_mgn_amt: String,
    #[serde(default)]
    pub futr_sprd_ord_mgna: String,
    #[serde(default)]
    pub opt_sll_new_mgn_amt: String,
    #[serde(default)]
    pub opt_buy_new_mgn_amt: String,
    #[serde(default)]
    pub new_mgn_amt: String,
    #[serde(default)]
    pub opt_pric_mgna: String,
    #[serde(default)]
    pub fuop_pric_altr_mgna: String,
    #[serde(default)]
    pub futr_sprd_mgna: String,
    #[serde(default)]
    pub uwdl_mgna: String,
    #[serde(default)]
    pub ctrt_per_min_mgna: String,
    #[serde(default)]
    pub tot_risk_mgna: String,
    #[serde(default)]
    pub netrisk_brkg_mgna: String,
    #[serde(default)]
    pub opt_sll_chgs: String,
    #[serde(default)]
    pub opt_buy_chgs: String,
    #[serde(default)]
    pub futr_loss_amt: String,
    #[serde(default)]
    pub futr_prft_amt: String,
    #[serde(default)]
    pub thdt_ccld_net_loss_amt: String,
    #[serde(default)]
    pub brkg_mgna: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DepositSummary {
    #[serde(default)]
    pub dnca_cash: String,
    #[serde(default)]
    pub dnca_sbst: String,
    #[serde(default)]
    pub dnca_tota: String,
    #[serde(default)]
    pub wdrw_psbl_cash_amt: String,
    #[serde(default)]
    pub wdrw_psbl_sbsa: String,
    #[serde(default)]
    pub wdrw_psbl_tot_amt: String,
    #[serde(default)]
    pub ord_psbl_cash_amt: String,
    #[serde(default)]
    pub ord_psbl_sbsa: String,
    #[serde(default)]
    pub ord_psbl_tot_amt: String,
    #[serde(default)]
    pub brkg_mgna_cash_amt: String,
    #[serde(default)]
    pub brkg_mgna_sbst: String,
    #[serde(default)]
    pub brkg_mgna_tot_amt: String,
    #[serde(default)]
    pub add_mgna_cash_amt: String,
    #[serde(default)]
    pub add_mgna_sbsa: String,
    #[serde(default)]
    pub add_mgna_tot_amt: String,
    #[serde(default)]
    pub bfdy_sbst_sll_sbst_amt: String,
    #[serde(default)]
    pub thdt_sbst_sll_sbst_amt: String,
    #[serde(default)]
    pub bfdy_sbst_sll_ccld_amt: String,
    #[serde(default)]
    pub thdt_sbst_sll_ccld_amt: String,
    #[serde(default)]
    pub opt_dfpa: String,
    #[serde(default)]
    pub excc_dfpa: String,
    #[serde(default)]
    pub fee_amt: String,
    #[serde(default)]
    pub nxdy_dncl_amt: String,
    #[serde(default)]
    pub prsm_dpast_amt: String,
    #[serde(default)]
    pub opt_buy_exus_acnt_yn: String,
    #[serde(default)]
    pub base_dpsa_gdat_grad_cd: String,
    #[serde(default)]
    pub opt_base_dpsa_gdat_grad_cd: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub margin_detail: Vec<MarginRow>,
    pub margin_summary: Vec<MarginRow>,
    pub deposit: Option<DepositSummary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("(야간)선물옵션 증거금 상세는 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("MGNA_DVSN_CD", req.mgna_dvsn_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let margin_detail = resp
        .output1
        .map(serde_json::from_value::<Vec<MarginRow>>)
        .transpose()?
        .unwrap_or_default();
    let margin_summary = resp
        .output2
        .map(serde_json::from_value::<Vec<MarginRow>>)
        .transpose()?
        .unwrap_or_default();
    // output3 is single object — not exposed via ApiResponse; access via raw if 필요.
    let deposit = None;
    Ok(Response {
        margin_detail,
        margin_summary,
        deposit,
    })
}
