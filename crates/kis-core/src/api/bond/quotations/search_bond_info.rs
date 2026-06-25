//! 장내채권 기본조회 — GET /uapi/domestic-bond/v1/quotations/search-bond-info
//!
//! 스펙: .agent/specs/bond__quotations__search_bond_info.md
//! 모의투자 미지원.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/quotations/search-bond-info";
pub const TR_ID: &str = "CTPF1114R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub pdno: String,
    pub prdt_type_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub ksd_bond_item_name: String,
    #[serde(default)]
    pub ksd_bond_item_eng_name: String,
    #[serde(default)]
    pub ksd_bond_lstg_type_cd: String,
    #[serde(default)]
    pub ksd_ofrg_dvsn_cd: String,
    #[serde(default)]
    pub ksd_bond_int_dfrm_dvsn_cd: String,
    #[serde(default)]
    pub issu_dt: String,
    #[serde(default)]
    pub rdpt_dt: String,
    #[serde(default)]
    pub rvnu_dt: String,
    #[serde(default)]
    pub iso_crcy_cd: String,
    #[serde(default)]
    pub mdwy_rdpt_dt: String,
    #[serde(default)]
    pub ksd_rcvg_bond_dsct_rt: String,
    #[serde(default)]
    pub ksd_rcvg_bond_srfc_inrt: String,
    #[serde(default)]
    pub bond_expd_rdpt_rt: String,
    #[serde(default)]
    pub ksd_prca_rdpt_mthd_cd: String,
    #[serde(default)]
    pub int_caltm_mcnt: String,
    #[serde(default)]
    pub ksd_int_calc_unit_cd: String,
    #[serde(default)]
    pub uval_cut_dvsn_cd: String,
    #[serde(default)]
    pub uval_cut_dcpt_dgit: String,
    #[serde(default)]
    pub ksd_dydv_caltm_aply_dvsn_cd: String,
    #[serde(default)]
    pub dydv_calc_dcnt: String,
    #[serde(default)]
    pub bond_expd_asrc_erng_rt: String,
    #[serde(default)]
    pub padf_plac_hdof_name: String,
    #[serde(default)]
    pub lstg_dt: String,
    #[serde(default)]
    pub lstg_abol_dt: String,
    #[serde(default)]
    pub ksd_bond_issu_mthd_cd: String,
    #[serde(default)]
    pub laps_indf_yn: String,
    #[serde(default)]
    pub ksd_lhdy_pnia_dfrm_mthd_cd: String,
    #[serde(default)]
    pub frst_int_dfrm_dt: String,
    #[serde(default)]
    pub ksd_prcm_lnkg_gvbd_yn: String,
    #[serde(default)]
    pub dpsi_end_dt: String,
    #[serde(default)]
    pub dpsi_strt_dt: String,
    #[serde(default)]
    pub dpsi_psbl_yn: String,
    #[serde(default)]
    pub atyp_rdpt_bond_erlm_yn: String,
    #[serde(default)]
    pub dshn_occr_yn: String,
    #[serde(default)]
    pub expd_exts_yn: String,
    #[serde(default)]
    pub pclr_ptcr_text: String,
    #[serde(default)]
    pub dpsi_psbl_excp_stat_cd: String,
    #[serde(default)]
    pub expd_exts_srdp_rcnt: String,
    #[serde(default)]
    pub expd_exts_srdp_rt: String,
    #[serde(default)]
    pub expd_rdpt_rt: String,
    #[serde(default)]
    pub expd_asrc_erng_rt: String,
    #[serde(default)]
    pub bond_int_dfrm_mthd_cd: String,
    #[serde(default)]
    pub int_dfrm_day_type_cd: String,
    #[serde(default)]
    pub prca_dfmt_term_mcnt: String,
    #[serde(default)]
    pub splt_rdpt_rcnt: String,
    #[serde(default)]
    pub rgbf_int_dfrm_dt: String,
    #[serde(default)]
    pub nxtm_int_dfrm_dt: String,
    #[serde(default)]
    pub sprx_psbl_yn: String,
    #[serde(default)]
    pub ictx_rt_dvsn_cd: String,
    #[serde(default)]
    pub bond_clsf_cd: String,
    #[serde(default)]
    pub bond_clsf_kor_name: String,
    #[serde(default)]
    pub int_mned_dvsn_cd: String,
    #[serde(default)]
    pub pnia_int_calc_unpr: String,
    #[serde(default)]
    pub frn_intr: String,
    #[serde(default)]
    pub aply_day_prcm_idx_lnkg_cefc: String,
    #[serde(default)]
    pub ksd_expd_dydv_calc_bass_cd: String,
    #[serde(default)]
    pub expd_dydv_calc_dcnt: String,
    #[serde(default)]
    pub ksd_cbbw_dvsn_cd: String,
    #[serde(default)]
    pub crfd_item_yn: String,
    #[serde(default)]
    pub pnia_bank_ofdy_dfrm_mthd_cd: String,
    #[serde(default)]
    pub qib_yn: String,
    #[serde(default)]
    pub qib_cclc_dt: String,
    #[serde(default)]
    pub csbd_yn: String,
    #[serde(default)]
    pub csbd_cclc_dt: String,
    #[serde(default)]
    pub ksd_opcb_yn: String,
    #[serde(default)]
    pub ksd_sodn_yn: String,
    #[serde(default)]
    pub ksd_rqdi_scty_yn: String,
    #[serde(default)]
    pub elec_scty_yn: String,
    #[serde(default)]
    pub rght_ecis_mbdy_dvsn_cd: String,
    #[serde(default)]
    pub int_rkng_mthd_dvsn_cd: String,
    #[serde(default)]
    pub ofrg_dvsn_cd: String,
    #[serde(default)]
    pub ksd_tot_issu_amt: String,
    #[serde(default)]
    pub next_indf_chk_ecls_yn: String,
    #[serde(default)]
    pub ksd_bond_intr_dvsn_cd: String,
    #[serde(default)]
    pub ksd_inrt_aply_dvsn_cd: String,
    #[serde(default)]
    pub krx_issu_istt_cd: String,
    #[serde(default)]
    pub ksd_indf_frqc_uder_calc_cd: String,
    #[serde(default)]
    pub ksd_indf_frqc_uder_calc_dcnt: String,
    #[serde(default)]
    pub tlg_rcvg_dtl_dtime: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 기본조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("PDNO", req.pdno.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.context("응답에 output 없음")?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
