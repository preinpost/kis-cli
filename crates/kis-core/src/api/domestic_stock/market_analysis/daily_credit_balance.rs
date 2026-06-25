//! 국내주식 신용잔고 일별추이 — GET /uapi/domestic-stock/v1/quotations/daily-credit-balance

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/daily-credit-balance";
pub const TR_ID: &str = "FHPST04760000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub deal_date: String,
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
    #[serde(default)]
    pub stlm_date: String,
    #[serde(default)]
    pub whol_loan_new_stcn: String,
    #[serde(default)]
    pub whol_loan_rdmp_stcn: String,
    #[serde(default)]
    pub whol_loan_rmnd_stcn: String,
    #[serde(default)]
    pub whol_loan_new_amt: String,
    #[serde(default)]
    pub whol_loan_rdmp_amt: String,
    #[serde(default)]
    pub whol_loan_rmnd_amt: String,
    #[serde(default)]
    pub whol_loan_rmnd_rate: String,
    #[serde(default)]
    pub whol_loan_gvrt: String,
    #[serde(default)]
    pub whol_stln_new_stcn: String,
    #[serde(default)]
    pub whol_stln_rdmp_stcn: String,
    #[serde(default)]
    pub whol_stln_rmnd_stcn: String,
    #[serde(default)]
    pub whol_stln_new_amt: String,
    #[serde(default)]
    pub whol_stln_rdmp_amt: String,
    #[serde(default)]
    pub whol_stln_rmnd_amt: String,
    #[serde(default)]
    pub whol_stln_rmnd_rate: String,
    #[serde(default)]
    pub whol_stln_gvrt: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
        ("fid_cond_scr_div_code", req.fid_cond_scr_div_code.as_str()),
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_input_date_1", req.fid_input_date_1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
