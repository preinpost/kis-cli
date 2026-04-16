//! ELW 종목검색 — GET /uapi/elw/v1/quotations/cond-search
//!
//! 스펙: .agent/specs/domestic_stock__elw__cond_search.md
//!
//! 모의투자 미지원. 최대 100건. 40+ 검색 필터 조건.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/cond-search";
pub const TR_ID: &str = "FHKEW15100000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// 11510
    pub fid_cond_scr_div_code: String,
    pub fid_rank_sort_cls_code: String,
    pub fid_input_cnt_1: String,
    pub fid_rank_sort_cls_code_2: String,
    pub fid_input_cnt_2: String,
    pub fid_rank_sort_cls_code_3: String,
    pub fid_input_cnt_3: String,
    pub fid_trgt_cls_code: String,
    pub fid_input_iscd: String,
    pub fid_unas_input_iscd: String,
    pub fid_mrkt_cls_code: String,
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
    pub fid_input_iscd_2: String,
    pub fid_etc_cls_code: String,
    pub fid_input_rmnn_dynu_1: String,
    pub fid_input_rmnn_dynu_2: String,
    pub fid_prpr_cnt1: String,
    pub fid_prpr_cnt2: String,
    pub fid_rsfl_rate1: String,
    pub fid_rsfl_rate2: String,
    pub fid_vol1: String,
    pub fid_vol2: String,
    pub fid_aply_rang_prc_1: String,
    pub fid_aply_rang_prc_2: String,
    pub fid_lvrg_val1: String,
    pub fid_lvrg_val2: String,
    pub fid_vol3: String,
    pub fid_vol4: String,
    pub fid_ints_vltl1: String,
    pub fid_ints_vltl2: String,
    pub fid_prmm_val1: String,
    pub fid_prmm_val2: String,
    pub fid_gear1: String,
    pub fid_gear2: String,
    pub fid_prls_qryr_rate1: String,
    pub fid_prls_qryr_rate2: String,
    pub fid_delta1: String,
    pub fid_delta2: String,
    pub fid_acpr1: String,
    pub fid_acpr2: String,
    pub fid_stck_cnvr_rate1: String,
    pub fid_stck_cnvr_rate2: String,
    pub fid_div_cls_code: String,
    pub fid_prit1: String,
    pub fid_prit2: String,
    pub fid_cfp1: String,
    pub fid_cfp2: String,
    pub fid_input_nmix_price_1: String,
    pub fid_input_nmix_price_2: String,
    pub fid_egea_val1: String,
    pub fid_egea_val2: String,
    pub fid_input_dvdn_ert: String,
    pub fid_input_hist_vltl: String,
    pub fid_theta1: String,
    pub fid_theta2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bond_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub rght_type_name: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acpr: String,
    #[serde(default)]
    pub stck_cnvr_rate: String,
    #[serde(default)]
    pub stck_lstn_date: String,
    #[serde(default)]
    pub stck_last_tr_date: String,
    #[serde(default)]
    pub hts_rmnn_dynu: String,
    #[serde(default)]
    pub unas_isnm: String,
    #[serde(default)]
    pub unas_prpr: String,
    #[serde(default)]
    pub unas_prdy_vrss: String,
    #[serde(default)]
    pub unas_prdy_vrss_sign: String,
    #[serde(default)]
    pub unas_prdy_ctrt: String,
    #[serde(default)]
    pub unas_acml_vol: String,
    #[serde(default)]
    pub moneyness: String,
    #[serde(default)]
    pub atm_cls_name: String,
    #[serde(default)]
    pub prit: String,
    #[serde(default)]
    pub delta_val: String,
    #[serde(default)]
    pub hts_ints_vltl: String,
    #[serde(default)]
    pub tmvl_val: String,
    #[serde(default)]
    pub gear: String,
    #[serde(default)]
    pub lvrg_val: String,
    #[serde(default)]
    pub prls_qryr_rate: String,
    #[serde(default)]
    pub cfp: String,
    #[serde(default)]
    pub lstn_stcn: String,
    #[serde(default)]
    pub pblc_co_name: String,
    #[serde(default)]
    pub lp_mbcr_name: String,
    #[serde(default)]
    pub lp_hldn_rate: String,
    #[serde(default)]
    pub elw_rght_form: String,
    #[serde(default)]
    pub elw_ko_barrier: String,
    #[serde(default)]
    pub apprch_rate: String,
    #[serde(default)]
    pub unas_shrn_iscd: String,
    #[serde(default)]
    pub mtrt_date: String,
    #[serde(default)]
    pub prmm_val: String,
    #[serde(default)]
    pub stck_lp_fin_date: String,
    #[serde(default)]
    pub tick_conv_prc: String,
    #[serde(default)]
    pub prls_qryr_stpr_prc: String,
    #[serde(default)]
    pub lp_hvol: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 종목검색은 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_INPUT_CNT_1", req.fid_input_cnt_1.as_str()),
        ("FID_RANK_SORT_CLS_CODE_2", req.fid_rank_sort_cls_code_2.as_str()),
        ("FID_INPUT_CNT_2", req.fid_input_cnt_2.as_str()),
        ("FID_RANK_SORT_CLS_CODE_3", req.fid_rank_sort_cls_code_3.as_str()),
        ("FID_INPUT_CNT_3", req.fid_input_cnt_3.as_str()),
        ("FID_TRGT_CLS_CODE", req.fid_trgt_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_UNAS_INPUT_ISCD", req.fid_unas_input_iscd.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
        ("FID_INPUT_ISCD_2", req.fid_input_iscd_2.as_str()),
        ("FID_ETC_CLS_CODE", req.fid_etc_cls_code.as_str()),
        ("FID_INPUT_RMNN_DYNU_1", req.fid_input_rmnn_dynu_1.as_str()),
        ("FID_INPUT_RMNN_DYNU_2", req.fid_input_rmnn_dynu_2.as_str()),
        ("FID_PRPR_CNT1", req.fid_prpr_cnt1.as_str()),
        ("FID_PRPR_CNT2", req.fid_prpr_cnt2.as_str()),
        ("FID_RSFL_RATE1", req.fid_rsfl_rate1.as_str()),
        ("FID_RSFL_RATE2", req.fid_rsfl_rate2.as_str()),
        ("FID_VOL1", req.fid_vol1.as_str()),
        ("FID_VOL2", req.fid_vol2.as_str()),
        ("FID_APLY_RANG_PRC_1", req.fid_aply_rang_prc_1.as_str()),
        ("FID_APLY_RANG_PRC_2", req.fid_aply_rang_prc_2.as_str()),
        ("FID_LVRG_VAL1", req.fid_lvrg_val1.as_str()),
        ("FID_LVRG_VAL2", req.fid_lvrg_val2.as_str()),
        ("FID_VOL3", req.fid_vol3.as_str()),
        ("FID_VOL4", req.fid_vol4.as_str()),
        ("FID_INTS_VLTL1", req.fid_ints_vltl1.as_str()),
        ("FID_INTS_VLTL2", req.fid_ints_vltl2.as_str()),
        ("FID_PRMM_VAL1", req.fid_prmm_val1.as_str()),
        ("FID_PRMM_VAL2", req.fid_prmm_val2.as_str()),
        ("FID_GEAR1", req.fid_gear1.as_str()),
        ("FID_GEAR2", req.fid_gear2.as_str()),
        ("FID_PRLS_QRYR_RATE1", req.fid_prls_qryr_rate1.as_str()),
        ("FID_PRLS_QRYR_RATE2", req.fid_prls_qryr_rate2.as_str()),
        ("FID_DELTA1", req.fid_delta1.as_str()),
        ("FID_DELTA2", req.fid_delta2.as_str()),
        ("FID_ACPR1", req.fid_acpr1.as_str()),
        ("FID_ACPR2", req.fid_acpr2.as_str()),
        ("FID_STCK_CNVR_RATE1", req.fid_stck_cnvr_rate1.as_str()),
        ("FID_STCK_CNVR_RATE2", req.fid_stck_cnvr_rate2.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_PRIT1", req.fid_prit1.as_str()),
        ("FID_PRIT2", req.fid_prit2.as_str()),
        ("FID_CFP1", req.fid_cfp1.as_str()),
        ("FID_CFP2", req.fid_cfp2.as_str()),
        ("FID_INPUT_NMIX_PRICE_1", req.fid_input_nmix_price_1.as_str()),
        ("FID_INPUT_NMIX_PRICE_2", req.fid_input_nmix_price_2.as_str()),
        ("FID_EGEA_VAL1", req.fid_egea_val1.as_str()),
        ("FID_EGEA_VAL2", req.fid_egea_val2.as_str()),
        ("FID_INPUT_DVDN_ERT", req.fid_input_dvdn_ert.as_str()),
        ("FID_INPUT_HIST_VLTL", req.fid_input_hist_vltl.as_str()),
        ("FID_THETA1", req.fid_theta1.as_str()),
        ("FID_THETA2", req.fid_theta2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
