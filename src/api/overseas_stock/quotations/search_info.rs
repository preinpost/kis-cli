//! 해외주식 상품기본정보 — GET /uapi/overseas-price/v1/quotations/search-info
//!
//! 스펙: .agent/specs/overseas_stock__quotations__search_info.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/search-info";
pub const TR_ID: &str = "CTPF1702R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub prdt_type_cd: String,
    pub pdno: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub std_pdno: String,
    #[serde(default)]
    pub prdt_eng_name: String,
    #[serde(default)]
    pub natn_cd: String,
    #[serde(default)]
    pub natn_name: String,
    #[serde(default)]
    pub tr_mket_cd: String,
    #[serde(default)]
    pub tr_mket_name: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
    #[serde(default)]
    pub ovrs_excg_name: String,
    #[serde(default)]
    pub tr_crcy_cd: String,
    #[serde(default)]
    pub ovrs_papr: String,
    #[serde(default)]
    pub crcy_name: String,
    #[serde(default)]
    pub ovrs_stck_dvsn_cd: String,
    #[serde(default)]
    pub prdt_clsf_cd: String,
    #[serde(default)]
    pub prdt_clsf_name: String,
    #[serde(default)]
    pub sll_unit_qty: String,
    #[serde(default)]
    pub buy_unit_qty: String,
    #[serde(default)]
    pub tr_unit_amt: String,
    #[serde(default)]
    pub lstg_stck_num: String,
    #[serde(default)]
    pub lstg_dt: String,
    #[serde(default)]
    pub ovrs_stck_tr_stop_dvsn_cd: String,
    #[serde(default)]
    pub lstg_abol_item_yn: String,
    #[serde(default)]
    pub ovrs_stck_prdt_grp_no: String,
    #[serde(default)]
    pub lstg_yn: String,
    #[serde(default)]
    pub tax_levy_yn: String,
    #[serde(default)]
    pub ovrs_stck_erlm_rosn_cd: String,
    #[serde(default)]
    pub ovrs_stck_hist_rght_dvsn_cd: String,
    #[serde(default)]
    pub chng_bf_pdno: String,
    #[serde(default)]
    pub prdt_type_cd_2: String,
    #[serde(default)]
    pub ovrs_item_name: String,
    #[serde(default)]
    pub sedol_no: String,
    #[serde(default)]
    pub blbg_tckr_text: String,
    #[serde(default)]
    pub ovrs_stck_etf_risk_drtp_cd: String,
    #[serde(default)]
    pub etp_chas_erng_rt_dbnb: String,
    #[serde(default)]
    pub istt_usge_isin_cd: String,
    #[serde(default)]
    pub mint_svc_yn: String,
    #[serde(default)]
    pub mint_svc_yn_chng_dt: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub lei_cd: String,
    #[serde(default)]
    pub ovrs_stck_stop_rson_cd: String,
    #[serde(default)]
    pub lstg_abol_dt: String,
    #[serde(default)]
    pub mini_stk_tr_stat_dvsn_cd: String,
    #[serde(default)]
    pub mint_frst_svc_erlm_dt: String,
    #[serde(default)]
    pub mint_dcpt_trad_psbl_yn: String,
    #[serde(default)]
    pub mint_fnum_trad_psbl_yn: String,
    #[serde(default)]
    pub mint_cblc_cvsn_ipsb_yn: String,
    #[serde(default)]
    pub ptp_item_yn: String,
    #[serde(default)]
    pub ptp_item_trfx_exmt_yn: String,
    #[serde(default)]
    pub ptp_item_trfx_exmt_strt_dt: String,
    #[serde(default)]
    pub ptp_item_trfx_exmt_end_dt: String,
    #[serde(default)]
    pub dtm_tr_psbl_yn: String,
    #[serde(default)]
    pub sdrf_stop_ecls_yn: String,
    #[serde(default)]
    pub sdrf_stop_ecls_erlm_dt: String,
    #[serde(default)]
    pub memo_text1: String,
    #[serde(default)]
    pub ovrs_now_pric1: String,
    #[serde(default)]
    pub last_rcvg_dtime: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외주식 상품기본정보는 모의투자 미지원");
    }
    let params = [
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
