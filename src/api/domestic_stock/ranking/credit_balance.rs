//! 국내주식 신용잔고 상위 — GET /uapi/domestic-stock/v1/ranking/credit-balance
//!
//! 스펙: .agent/specs/domestic_stock__ranking__credit_balance.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/ranking/credit-balance";
pub const TR_ID: &str = "FHKST17010000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_option: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_rank_sort_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub bstp_cls_code: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub stnd_date1: String,
    #[serde(default)]
    pub stnd_date2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub whol_loan_rmnd_stcn: String,
    #[serde(default)]
    pub whol_loan_rmnd_amt: String,
    #[serde(default)]
    pub whol_loan_rmnd_rate: String,
    #[serde(default)]
    pub whol_stln_rmnd_stcn: String,
    #[serde(default)]
    pub whol_stln_rmnd_amt: String,
    #[serde(default)]
    pub whol_stln_rmnd_rate: String,
    #[serde(default)]
    pub nday_vrss_loan_rmnd_inrt: String,
    #[serde(default)]
    pub nday_vrss_stln_rmnd_inrt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내주식 신용잔고 상위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_OPTION", req.fid_option.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Vec<Meta>>(v).ok())
        .and_then(|mut arr| arr.pop())
        .or_else(|| None);
    let rows: Vec<Row> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, rows })
}
