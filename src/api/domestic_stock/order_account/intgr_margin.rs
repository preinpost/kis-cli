//! 주식통합증거금 현황 — GET /uapi/domestic-stock/v1/trading/intgr-margin
//!
//! 스펙: .agent/specs/domestic_stock__order_account__intgr_margin.md
//!
//! 모의투자 미지원. output이 단일 Object, 필드 100+개 (한번에 포괄적 증거금 정보).

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/intgr-margin";
pub const TR_ID: &str = "TTTC0869R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    /// N 입력
    pub cma_evlu_amt_icld_yn: String,
    /// 01 외화기준, 02 원화기준
    pub wcrc_frcr_dvsn_cd: String,
    /// 01 외화기준, 02 원화기준
    pub fwex_ctrt_frcr_dvsn_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub acmga_rt: String,
    #[serde(default)]
    pub acmga_pct100_aptm_rson: String,
    #[serde(default)]
    pub stck_cash_objt_amt: String,
    #[serde(default)]
    pub stck_sbst_objt_amt: String,
    #[serde(default)]
    pub stck_evlu_objt_amt: String,
    #[serde(default)]
    pub stck_ruse_psbl_objt_amt: String,
    #[serde(default)]
    pub stck_fund_rpch_chgs_objt_amt: String,
    #[serde(default)]
    pub stck_fncg_rdpt_objt_atm: String,
    #[serde(default)]
    pub bond_ruse_psbl_objt_amt: String,
    #[serde(default)]
    pub stck_cash_use_amt: String,
    #[serde(default)]
    pub stck_sbst_use_amt: String,
    #[serde(default)]
    pub stck_evlu_use_amt: String,
    #[serde(default)]
    pub stck_ruse_psbl_amt_use_amt: String,
    #[serde(default)]
    pub stck_fund_rpch_chgs_use_amt: String,
    #[serde(default)]
    pub stck_fncg_rdpt_amt_use_amt: String,
    #[serde(default)]
    pub bond_ruse_psbl_amt_use_amt: String,
    #[serde(default)]
    pub stck_cash_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_sbst_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_evlu_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_ruse_psbl_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_fund_rpch_ord_psbl_amt: String,
    #[serde(default)]
    pub bond_ruse_psbl_ord_psbl_amt: String,
    #[serde(default)]
    pub rcvb_amt: String,
    #[serde(default)]
    pub stck_loan_grta_ruse_psbl_amt: String,
    #[serde(default)]
    pub stck_cash20_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_cash30_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_cash40_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_cash50_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_cash60_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_cash100_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_rsip100_max_ord_psbl_amt: String,
    #[serde(default)]
    pub bond_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_fncg45_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_fncg50_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_fncg60_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_fncg70_max_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_stln_max_ord_psbl_amt: String,
    #[serde(default)]
    pub lmt_amt: String,
    #[serde(default)]
    pub ovrs_stck_itgr_mgna_dvsn_name: String,
    #[serde(default)]
    pub usd_objt_amt: String,
    #[serde(default)]
    pub usd_use_amt: String,
    #[serde(default)]
    pub usd_ord_psbl_amt: String,
    #[serde(default)]
    pub hkd_objt_amt: String,
    #[serde(default)]
    pub hkd_use_amt: String,
    #[serde(default)]
    pub hkd_ord_psbl_amt: String,
    #[serde(default)]
    pub jpy_objt_amt: String,
    #[serde(default)]
    pub jpy_use_amt: String,
    #[serde(default)]
    pub jpy_ord_psbl_amt: String,
    #[serde(default)]
    pub cny_objt_amt: String,
    #[serde(default)]
    pub cny_use_amt: String,
    #[serde(default)]
    pub cny_ord_psbl_amt: String,
    #[serde(default)]
    pub usd_ruse_objt_amt: String,
    #[serde(default)]
    pub usd_ruse_amt: String,
    #[serde(default)]
    pub usd_ruse_ord_psbl_amt: String,
    #[serde(default)]
    pub hkd_ruse_objt_amt: String,
    #[serde(default)]
    pub hkd_ruse_amt: String,
    #[serde(default)]
    pub hkd_ruse_ord_psbl_amt: String,
    #[serde(default)]
    pub jpy_ruse_objt_amt: String,
    #[serde(default)]
    pub jpy_ruse_amt: String,
    #[serde(default)]
    pub jpy_ruse_ord_psbl_amt: String,
    #[serde(default)]
    pub cny_ruse_objt_amt: String,
    #[serde(default)]
    pub cny_ruse_amt: String,
    #[serde(default)]
    pub cny_ruse_ord_psbl_amt: String,
    #[serde(default)]
    pub usd_gnrl_ord_psbl_amt: String,
    #[serde(default)]
    pub usd_itgr_ord_psbl_amt: String,
    #[serde(default)]
    pub hkd_gnrl_ord_psbl_amt: String,
    #[serde(default)]
    pub hkd_itgr_ord_psbl_amt: String,
    #[serde(default)]
    pub jpy_gnrl_ord_psbl_amt: String,
    #[serde(default)]
    pub jpy_itgr_ord_psbl_amt: String,
    #[serde(default)]
    pub cny_gnrl_ord_psbl_amt: String,
    #[serde(default)]
    pub cny_itgr_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_cash20_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_cash30_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_cash40_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_cash50_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_cash60_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_cash100_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_100_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_fncg45_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_fncg50_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_fncg60_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_fncg70_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_itgr_stln_ord_psbl_amt: String,
    #[serde(default)]
    pub bond_itgr_ord_psbl_amt: String,
    #[serde(default)]
    pub stck_cash_ovrs_use_amt: String,
    #[serde(default)]
    pub stck_sbst_ovrs_use_amt: String,
    #[serde(default)]
    pub stck_evlu_ovrs_use_amt: String,
    #[serde(default)]
    pub stck_re_use_amt_ovrs_use_amt: String,
    #[serde(default)]
    pub stck_fund_rpch_ovrs_use_amt: String,
    #[serde(default)]
    pub stck_fncg_rdpt_ovrs_use_amt: String,
    #[serde(default)]
    pub bond_re_use_ovrs_use_amt: String,
    #[serde(default)]
    pub usd_oth_mket_use_amt: String,
    #[serde(default)]
    pub jpy_oth_mket_use_amt: String,
    #[serde(default)]
    pub cny_oth_mket_use_amt: String,
    #[serde(default)]
    pub hkd_oth_mket_use_amt: String,
    #[serde(default)]
    pub usd_re_use_oth_mket_use_amt: String,
    #[serde(default)]
    pub jpy_re_use_oth_mket_use_amt: String,
    #[serde(default)]
    pub cny_re_use_oth_mket_use_amt: String,
    #[serde(default)]
    pub hkd_re_use_oth_mket_use_amt: String,
    #[serde(default)]
    pub hgkg_cny_re_use_amt: String,
    #[serde(default)]
    pub usd_frst_bltn_exrt: String,
    #[serde(default)]
    pub hkd_frst_bltn_exrt: String,
    #[serde(default)]
    pub jpy_frst_bltn_exrt: String,
    #[serde(default)]
    pub cny_frst_bltn_exrt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("주식통합증거금 현황은 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("CMA_EVLU_AMT_ICLD_YN", req.cma_evlu_amt_icld_yn.as_str()),
        ("WCRC_FRCR_DVSN_CD", req.wcrc_frcr_dvsn_cd.as_str()),
        ("FWEX_CTRT_FRCR_DVSN_CD", req.fwex_ctrt_frcr_dvsn_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
