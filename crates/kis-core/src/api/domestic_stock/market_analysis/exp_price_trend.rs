//! 국내주식 예상체결가 추이 — GET /uapi/domestic-stock/v1/quotations/exp-price-trend

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/exp-price-trend";
pub const TR_ID: &str = "FHPST01810000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_mkop_cls_code: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
    #[serde(default)]
    pub antc_cnpr: String,
    #[serde(default)]
    pub antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub antc_cntg_vrss: String,
    #[serde(default)]
    pub antc_cntg_prdy_ctrt: String,
    #[serde(default)]
    pub antc_vol: String,
    #[serde(default)]
    pub antc_tr_pbmn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("fid_mkop_cls_code", req.fid_mkop_cls_code.as_str()),
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let rows: Vec<Row> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, rows })
}
