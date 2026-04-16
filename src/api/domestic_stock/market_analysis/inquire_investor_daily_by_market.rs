//! 시장별 투자자매매동향(일별) — GET /uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-investor-daily-by-market";
pub const TR_ID: &str = "FHPTJ04040000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
    pub fid_input_iscd_1: String,
    pub fid_input_date_2: String,
    pub fid_input_iscd_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub bstp_nmix_oprc: String,
    #[serde(default)]
    pub bstp_nmix_hgpr: String,
    #[serde(default)]
    pub bstp_nmix_lwpr: String,
    #[serde(default)]
    pub stck_prdy_clpr: String,
    #[serde(default)]
    pub frgn_ntby_qty: String,
    #[serde(default)]
    pub frgn_reg_ntby_qty: String,
    #[serde(default)]
    pub frgn_nreg_ntby_qty: String,
    #[serde(default)]
    pub prsn_ntby_qty: String,
    #[serde(default)]
    pub orgn_ntby_qty: String,
    #[serde(default)]
    pub scrt_ntby_qty: String,
    #[serde(default)]
    pub ivtr_ntby_qty: String,
    #[serde(default)]
    pub pe_fund_ntby_vol: String,
    #[serde(default)]
    pub bank_ntby_qty: String,
    #[serde(default)]
    pub insu_ntby_qty: String,
    #[serde(default)]
    pub mrbn_ntby_qty: String,
    #[serde(default)]
    pub fund_ntby_qty: String,
    #[serde(default)]
    pub etc_ntby_qty: String,
    #[serde(default)]
    pub etc_orgt_ntby_vol: String,
    #[serde(default)]
    pub etc_corp_ntby_vol: String,
    #[serde(default)]
    pub frgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub frgn_reg_ntby_pbmn: String,
    #[serde(default)]
    pub frgn_nreg_ntby_pbmn: String,
    #[serde(default)]
    pub prsn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub orgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub scrt_ntby_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_ntby_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_ntby_tr_pbmn: String,
    #[serde(default)]
    pub bank_ntby_tr_pbmn: String,
    #[serde(default)]
    pub insu_ntby_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub fund_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_ntby_tr_pbmn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_ISCD_1", req.fid_input_iscd_1.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
        ("FID_INPUT_ISCD_2", req.fid_input_iscd_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
