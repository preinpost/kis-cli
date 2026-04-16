//! ELW 기초자산별 종목시세 — GET /uapi/elw/v1/quotations/udrl-asset-price
//!
//! 스펙: .agent/specs/domestic_stock__elw__udrl_asset_price.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/udrl-asset-price";
pub const TR_ID: &str = "FHKEW154101C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// Unique 11541
    pub fid_cond_scr_div_code: String,
    /// A 전체, C 콜, P 풋
    pub fid_mrkt_cls_code: String,
    pub fid_input_iscd: String,
    pub fid_unas_input_iscd: String,
    pub fid_vol_cnt: String,
    /// 0 미체크, 1 체크
    pub fid_trgt_exls_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_input_vol_1: String,
    pub fid_input_vol_2: String,
    pub fid_input_rmnn_dynu_1: String,
    pub fid_input_rmnn_dynu_2: String,
    /// 0 없음, 1 ATM, 2 ITM, 3 OTM
    pub fid_option: String,
    pub fid_input_option_1: String,
    pub fid_input_option_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
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
    pub prls_qryr_stpr_prc: String,
    #[serde(default)]
    pub hts_rmnn_dynu: String,
    #[serde(default)]
    pub hts_ints_vltl: String,
    #[serde(default)]
    pub stck_cnvr_rate: String,
    #[serde(default)]
    pub lp_hvol: String,
    #[serde(default)]
    pub lp_rlim: String,
    #[serde(default)]
    pub lvrg_val: String,
    #[serde(default)]
    pub gear: String,
    #[serde(default)]
    pub delta_val: String,
    #[serde(default)]
    pub gama: String,
    #[serde(default)]
    pub vega: String,
    #[serde(default)]
    pub theta: String,
    #[serde(default)]
    pub prls_qryr_rate: String,
    #[serde(default)]
    pub cfp: String,
    #[serde(default)]
    pub prit: String,
    #[serde(default)]
    pub invl_val: String,
    #[serde(default)]
    pub tmvl_val: String,
    #[serde(default)]
    pub hts_thpr: String,
    #[serde(default)]
    pub stck_lstn_date: String,
    #[serde(default)]
    pub stck_last_tr_date: String,
    #[serde(default)]
    pub lp_ntby_qty: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 기초자산별 종목시세는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_UNAS_INPUT_ISCD", req.fid_unas_input_iscd.as_str()),
        ("FID_VOL_CNT", req.fid_vol_cnt.as_str()),
        ("FID_TRGT_EXLS_CLS_CODE", req.fid_trgt_exls_cls_code.as_str()),
        ("FID_INPUT_PRICE_1", req.fid_input_price_1.as_str()),
        ("FID_INPUT_PRICE_2", req.fid_input_price_2.as_str()),
        ("FID_INPUT_VOL_1", req.fid_input_vol_1.as_str()),
        ("FID_INPUT_VOL_2", req.fid_input_vol_2.as_str()),
        ("FID_INPUT_RMNN_DYNU_1", req.fid_input_rmnn_dynu_1.as_str()),
        ("FID_INPUT_RMNN_DYNU_2", req.fid_input_rmnn_dynu_2.as_str()),
        ("FID_OPTION", req.fid_option.as_str()),
        ("FID_INPUT_OPTION_1", req.fid_input_option_1.as_str()),
        ("FID_INPUT_OPTION_2", req.fid_input_option_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
