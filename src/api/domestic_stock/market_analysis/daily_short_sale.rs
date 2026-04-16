//! 국내주식 공매도 일별추이 — GET /uapi/domestic-stock/v1/quotations/daily-short-sale

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/daily-short-sale";
pub const TR_ID: &str = "FHPST04830000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_input_date_2: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
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
    pub prdy_vol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_clpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub stnd_vol_smtn: String,
    #[serde(default)]
    pub ssts_cntg_qty: String,
    #[serde(default)]
    pub ssts_vol_rlim: String,
    #[serde(default)]
    pub acml_ssts_cntg_qty: String,
    #[serde(default)]
    pub acml_ssts_cntg_qty_rlim: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub stnd_tr_pbmn_smtn: String,
    #[serde(default)]
    pub ssts_tr_pbmn: String,
    #[serde(default)]
    pub ssts_tr_pbmn_rlim: String,
    #[serde(default)]
    pub acml_ssts_tr_pbmn: String,
    #[serde(default)]
    pub acml_ssts_tr_pbmn_rlim: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub avrg_prc: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
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
