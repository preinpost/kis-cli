//! 종목별 프로그램매매추이(일별) — GET /uapi/domestic-stock/v1/quotations/program-trade-by-stock-daily

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/program-trade-by-stock-daily";
pub const TR_ID: &str = "FHPPG04650201";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
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
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub whol_smtn_seln_vol: String,
    #[serde(default)]
    pub whol_smtn_shnu_vol: String,
    #[serde(default)]
    pub whol_smtn_ntby_qty: String,
    #[serde(default)]
    pub whol_smtn_seln_tr_pbmn: String,
    #[serde(default)]
    pub whol_smtn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub whol_smtn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub whol_ntby_vol_icdc: String,
    #[serde(default)]
    pub whol_ntby_tr_pbmn_icdc2: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
