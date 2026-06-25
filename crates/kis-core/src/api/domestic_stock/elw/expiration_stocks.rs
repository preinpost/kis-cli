//! ELW 만기예정/만기종목 — GET /uapi/elw/v1/quotations/expiration-stocks
//!
//! 스펙: .agent/specs/domestic_stock__elw__expiration_stocks.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/expiration-stocks";
pub const TR_ID: &str = "FHKEW154700C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// 11547
    pub fid_cond_scr_div_code: String,
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
    /// 0 콜, 1 풋, 2 전체
    pub fid_div_cls_code: String,
    pub fid_etc_cls_code: String,
    pub fid_unas_input_iscd: String,
    pub fid_input_iscd_2: String,
    /// 0 전체, 1 일반, 2 조기종료
    pub fid_blng_cls_code: String,
    pub fid_input_option_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub elw_kor_isnm: String,
    #[serde(default)]
    pub unas_isnm: String,
    #[serde(default)]
    pub unas_prpr: String,
    #[serde(default)]
    pub acpr: String,
    #[serde(default)]
    pub stck_cnvr_rate: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub stck_lstn_date: String,
    #[serde(default)]
    pub stck_last_tr_date: String,
    #[serde(default)]
    pub total_rdmp_amt: String,
    #[serde(default)]
    pub rdmp_amt: String,
    #[serde(default)]
    pub lstn_stcn: String,
    #[serde(default)]
    pub lp_hvol: String,
    #[serde(default)]
    pub ccls_paym_prc: String,
    #[serde(default)]
    pub mtrt_vltn_amt: String,
    #[serde(default)]
    pub evnt_prd_fin_date: String,
    #[serde(default)]
    pub stlm_date: String,
    #[serde(default)]
    pub pblc_prc: String,
    #[serde(default)]
    pub unas_shrn_iscd: String,
    #[serde(default)]
    pub stnd_iscd: String,
    #[serde(default)]
    pub rdmp_ask_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_ETC_CLS_CODE", req.fid_etc_cls_code.as_str()),
        ("FID_UNAS_INPUT_ISCD", req.fid_unas_input_iscd.as_str()),
        ("FID_INPUT_ISCD_2", req.fid_input_iscd_2.as_str()),
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
        ("FID_INPUT_OPTION_1", req.fid_input_option_1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output1.ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
