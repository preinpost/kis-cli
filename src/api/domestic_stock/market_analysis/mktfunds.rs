//! 국내 증시자금 종합 — GET /uapi/domestic-stock/v1/quotations/mktfunds

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/mktfunds";
pub const TR_ID: &str = "FHKST649100C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_input_date_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bsop_date: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub hts_avls: String,
    #[serde(default)]
    pub cust_dpmn_amt: String,
    #[serde(default)]
    pub cust_dpmn_amt_prdy_vrss: String,
    #[serde(default)]
    pub amt_tnrt: String,
    #[serde(default)]
    pub uncl_amt: String,
    #[serde(default)]
    pub crdt_loan_rmnd: String,
    #[serde(default)]
    pub futs_tfam_amt: String,
    #[serde(default)]
    pub sttp_amt: String,
    #[serde(default)]
    pub mxtp_amt: String,
    #[serde(default)]
    pub bntp_amt: String,
    #[serde(default)]
    pub mmf_amt: String,
    #[serde(default)]
    pub secu_lend_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [("FID_INPUT_DATE_1", req.fid_input_date_1.as_str())];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
