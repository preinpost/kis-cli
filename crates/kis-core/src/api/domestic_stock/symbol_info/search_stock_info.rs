//! 주식기본조회 — GET /uapi/domestic-stock/v1/quotations/search-stock-info
//!
//! 스펙: .agent/specs/domestic_stock__symbol_info__search_stock_info.md
//!
//! 주식/ETF/ETN/ELW/선물옵션/채권/ELS 종목의 상세 기본정보. 70+ 필드.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/search-stock-info";
pub const TR_ID: &str = "CTPF1002R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 300 주식/ETF/ETN/ELW, 301 선물옵션, 302 채권, 306 ELS
    pub prdt_type_cd: String,
    pub pdno: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub mket_id_cd: String,
    #[serde(default)]
    pub scty_grp_id_cd: String,
    #[serde(default)]
    pub excg_dvsn_cd: String,
    #[serde(default)]
    pub setl_mmdd: String,
    #[serde(default)]
    pub lstg_stqt: String,
    #[serde(default)]
    pub lstg_cptl_amt: String,
    #[serde(default)]
    pub cpta: String,
    #[serde(default)]
    pub papr: String,
    #[serde(default)]
    pub issu_pric: String,
    #[serde(default)]
    pub kospi200_item_yn: String,
    #[serde(default)]
    pub scts_mket_lstg_dt: String,
    #[serde(default)]
    pub scts_mket_lstg_abol_dt: String,
    #[serde(default)]
    pub kosdaq_mket_lstg_dt: String,
    #[serde(default)]
    pub kosdaq_mket_lstg_abol_dt: String,
    #[serde(default)]
    pub frbd_mket_lstg_dt: String,
    #[serde(default)]
    pub frbd_mket_lstg_abol_dt: String,
    #[serde(default)]
    pub reits_kind_cd: String,
    #[serde(default)]
    pub etf_dvsn_cd: String,
    #[serde(default)]
    pub oilf_fund_yn: String,
    #[serde(default)]
    pub idx_bztp_lcls_cd: String,
    #[serde(default)]
    pub idx_bztp_mcls_cd: String,
    #[serde(default)]
    pub idx_bztp_scls_cd: String,
    #[serde(default)]
    pub stck_kind_cd: String,
    #[serde(default)]
    pub mfnd_opng_dt: String,
    #[serde(default)]
    pub mfnd_end_dt: String,
    #[serde(default)]
    pub dpsi_erlm_cncl_dt: String,
    #[serde(default)]
    pub etf_cu_qty: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub prdt_name120: String,
    #[serde(default)]
    pub prdt_abrv_name: String,
    #[serde(default)]
    pub std_pdno: String,
    #[serde(default)]
    pub prdt_eng_name: String,
    #[serde(default)]
    pub prdt_eng_name120: String,
    #[serde(default)]
    pub prdt_eng_abrv_name: String,
    #[serde(default)]
    pub dpsi_aptm_erlm_yn: String,
    #[serde(default)]
    pub etf_txtn_type_cd: String,
    #[serde(default)]
    pub etf_type_cd: String,
    #[serde(default)]
    pub lstg_abol_dt: String,
    #[serde(default)]
    pub nwst_odst_dvsn_cd: String,
    #[serde(default)]
    pub sbst_pric: String,
    #[serde(default)]
    pub thco_sbst_pric: String,
    #[serde(default)]
    pub thco_sbst_pric_chng_dt: String,
    #[serde(default)]
    pub tr_stop_yn: String,
    #[serde(default)]
    pub admn_item_yn: String,
    #[serde(default)]
    pub thdt_clpr: String,
    #[serde(default)]
    pub bfdy_clpr: String,
    #[serde(default)]
    pub clpr_chng_dt: String,
    #[serde(default)]
    pub std_idst_clsf_cd: String,
    #[serde(default)]
    pub std_idst_clsf_cd_name: String,
    #[serde(default)]
    pub idx_bztp_lcls_cd_name: String,
    #[serde(default)]
    pub idx_bztp_mcls_cd_name: String,
    #[serde(default)]
    pub idx_bztp_scls_cd_name: String,
    #[serde(default)]
    pub ocr_no: String,
    #[serde(default)]
    pub crfd_item_yn: String,
    #[serde(default)]
    pub elec_scty_yn: String,
    #[serde(default)]
    pub issu_istt_cd: String,
    #[serde(default)]
    pub etf_chas_erng_rt_dbnb: String,
    #[serde(default)]
    pub etf_etn_ivst_heed_item_yn: String,
    #[serde(default)]
    pub stln_int_rt_dvsn_cd: String,
    #[serde(default)]
    pub frnr_psnl_lmt_rt: String,
    #[serde(default)]
    pub lstg_rqsr_issu_istt_cd: String,
    #[serde(default)]
    pub lstg_rqsr_item_cd: String,
    #[serde(default)]
    pub trst_istt_issu_istt_cd: String,
    #[serde(default)]
    pub cptt_trad_tr_psbl_yn: String,
    #[serde(default)]
    pub nxt_tr_stop_yn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
