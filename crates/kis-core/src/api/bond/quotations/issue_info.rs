//! 장내채권 발행정보 — GET /uapi/domestic-bond/v1/quotations/issue-info
//!
//! 스펙: .agent/specs/bond__quotations__issue_info.md
//! 모의투자 미지원.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/quotations/issue-info";
pub const TR_ID: &str = "CTPF1101R";

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
    pub prdt_name: String,
    #[serde(default)]
    pub prdt_eng_name: String,
    #[serde(default)]
    pub ivst_heed_prdt_yn: String,
    #[serde(default)]
    pub exts_yn: String,
    #[serde(default)]
    pub bond_clsf_cd: String,
    #[serde(default)]
    pub bond_clsf_kor_name: String,
    #[serde(default)]
    pub papr: String,
    #[serde(default)]
    pub int_mned_dvsn_cd: String,
    #[serde(default)]
    pub rvnu_shap_cd: String,
    #[serde(default)]
    pub issu_amt: String,
    #[serde(default)]
    pub lstg_rmnd: String,
    #[serde(default)]
    pub int_dfrm_mcnt: String,
    #[serde(default)]
    pub bond_int_dfrm_mthd_cd: String,
    #[serde(default)]
    pub splt_rdpt_rcnt: String,
    #[serde(default)]
    pub prca_dfmt_term_mcnt: String,
    #[serde(default)]
    pub int_anap_dvsn_cd: String,
    #[serde(default)]
    pub bond_rght_dvsn_cd: String,
    #[serde(default)]
    pub prdt_pclc_text: String,
    #[serde(default)]
    pub prdt_abrv_name: String,
    #[serde(default)]
    pub prdt_eng_abrv_name: String,
    #[serde(default)]
    pub sprx_psbl_yn: String,
    #[serde(default)]
    pub pbff_pplc_ofrg_mthd_cd: String,
    #[serde(default)]
    pub cmco_cd: String,
    #[serde(default)]
    pub issu_istt_cd: String,
    #[serde(default)]
    pub issu_istt_name: String,
    #[serde(default)]
    pub pnia_dfrm_agcy_istt_cd: String,
    #[serde(default)]
    pub dsct_ec_rt: String,
    #[serde(default)]
    pub srfc_inrt: String,
    #[serde(default)]
    pub expd_rdpt_rt: String,
    #[serde(default)]
    pub expd_asrc_erng_rt: String,
    #[serde(default)]
    pub bond_grte_istt_name: String,
    #[serde(default)]
    pub int_dfrm_day_type_cd: String,
    #[serde(default)]
    pub ksd_int_calc_unit_cd: String,
    #[serde(default)]
    pub int_wunt_uder_prcs_dvsn_cd: String,
    #[serde(default)]
    pub rvnu_dt: String,
    #[serde(default)]
    pub issu_dt: String,
    #[serde(default)]
    pub lstg_dt: String,
    #[serde(default)]
    pub expd_dt: String,
    #[serde(default)]
    pub rdpt_dt: String,
    #[serde(default)]
    pub sbst_pric: String,
    #[serde(default)]
    pub rgbf_int_dfrm_dt: String,
    #[serde(default)]
    pub nxtm_int_dfrm_dt: String,
    #[serde(default)]
    pub frst_int_dfrm_dt: String,
    #[serde(default)]
    pub ecis_pric: String,
    #[serde(default)]
    pub rght_stck_std_pdno: String,
    #[serde(default)]
    pub ecis_opng_dt: String,
    #[serde(default)]
    pub ecis_end_dt: String,
    #[serde(default)]
    pub bond_rvnu_mthd_cd: String,
    #[serde(default)]
    pub oprt_stfno: String,
    #[serde(default)]
    pub oprt_stff_name: String,
    #[serde(default)]
    pub rgbf_int_dfrm_wday: String,
    #[serde(default)]
    pub nxtm_int_dfrm_wday: String,
    #[serde(default)]
    pub kis_crdt_grad_text: String,
    #[serde(default)]
    pub kbp_crdt_grad_text: String,
    #[serde(default)]
    pub nice_crdt_grad_text: String,
    #[serde(default)]
    pub fnp_crdt_grad_text: String,
    #[serde(default)]
    pub dpsi_psbl_yn: String,
    #[serde(default)]
    pub pnia_int_calc_unpr: String,
    #[serde(default)]
    pub prcm_idx_bond_yn: String,
    #[serde(default)]
    pub expd_exts_srdp_rcnt: String,
    #[serde(default)]
    pub expd_exts_srdp_rt: String,
    #[serde(default)]
    pub loan_psbl_yn: String,
    #[serde(default)]
    pub grte_dvsn_cd: String,
    #[serde(default)]
    pub fnrr_rank_dvsn_cd: String,
    #[serde(default)]
    pub krx_lstg_abol_dvsn_cd: String,
    #[serde(default)]
    pub asst_rqdi_dvsn_cd: String,
    #[serde(default)]
    pub opcb_dvsn_cd: String,
    #[serde(default)]
    pub crfd_item_yn: String,
    #[serde(default)]
    pub crfd_item_rstc_cclc_dt: String,
    #[serde(default)]
    pub bond_nmpr_unit_pric: String,
    #[serde(default)]
    pub ivst_heed_bond_dvsn_name: String,
    #[serde(default)]
    pub add_erng_rt: String,
    #[serde(default)]
    pub add_erng_rt_aply_dt: String,
    #[serde(default)]
    pub bond_tr_stop_dvsn_cd: String,
    #[serde(default)]
    pub ivst_heed_bond_dvsn_cd: String,
    #[serde(default)]
    pub pclr_cndt_text: String,
    #[serde(default)]
    pub hbbd_yn: String,
    #[serde(default)]
    pub cdtl_cptl_scty_type_cd: String,
    #[serde(default)]
    pub elec_scty_yn: String,
    #[serde(default)]
    pub sq1_clop_ecis_opng_dt: String,
    #[serde(default)]
    pub frst_erlm_stfno: String,
    #[serde(default)]
    pub frst_erlm_dt: String,
    #[serde(default)]
    pub frst_erlm_tmd: String,
    #[serde(default)]
    pub tlg_rcvg_dtl_dtime: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 발행정보는 모의투자 미지원 API입니다");
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
