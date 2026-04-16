//! 해외주식 미체결내역 — GET /uapi/overseas-stock/v1/trading/inquire-nccs
//!
//! 스펙: .agent/specs/overseas_stock__order_account__inquire_nccs.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/inquire-nccs";
pub const TR_ID: &str = "TTTS3018R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub ovrs_excg_cd: String,
    pub sort_sqn: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub ord_gno_brno: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd_name: String,
    #[serde(default)]
    pub rvse_cncl_dvsn_cd: String,
    #[serde(default)]
    pub rvse_cncl_dvsn_cd_name: String,
    #[serde(default)]
    pub rjct_rson: String,
    #[serde(default)]
    pub rjct_rson_name: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub tr_mket_name: String,
    #[serde(default)]
    pub tr_crcy_cd: String,
    #[serde(default)]
    pub natn_cd: String,
    #[serde(default)]
    pub natn_kor_name: String,
    #[serde(default)]
    pub ft_ord_qty: String,
    #[serde(default)]
    pub ft_ccld_qty: String,
    #[serde(default)]
    pub nccs_qty: String,
    #[serde(default)]
    pub ft_ord_unpr3: String,
    #[serde(default)]
    pub ft_ccld_unpr3: String,
    #[serde(default)]
    pub ft_ccld_amt3: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
    #[serde(default)]
    pub prcs_stat_name: String,
    #[serde(default)]
    pub loan_type_cd: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub usa_amk_exts_rqst_yn: String,
    #[serde(default)]
    pub splt_buy_attr_name: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("해외주식 미체결내역은 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("OVRS_EXCG_CD", req.ovrs_excg_cd.as_str()),
        ("SORT_SQN", req.sort_sqn.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
