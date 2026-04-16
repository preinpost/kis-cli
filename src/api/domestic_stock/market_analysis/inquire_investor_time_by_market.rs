//! 시장별 투자자매매동향(시세) — GET /uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-investor-time-by-market";
pub const TR_ID: &str = "FHPTJ04030000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_input_iscd: String,
    pub fid_input_iscd_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub frgn_seln_vol: String,
    #[serde(default)]
    pub frgn_shnu_vol: String,
    #[serde(default)]
    pub frgn_ntby_qty: String,
    #[serde(default)]
    pub frgn_seln_tr_pbmn: String,
    #[serde(default)]
    pub frgn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub frgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub prsn_seln_vol: String,
    #[serde(default)]
    pub prsn_shnu_vol: String,
    #[serde(default)]
    pub prsn_ntby_qty: String,
    #[serde(default)]
    pub prsn_seln_tr_pbmn: String,
    #[serde(default)]
    pub prsn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub prsn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub orgn_seln_vol: String,
    #[serde(default)]
    pub orgn_shnu_vol: String,
    #[serde(default)]
    pub orgn_ntby_qty: String,
    #[serde(default)]
    pub orgn_seln_tr_pbmn: String,
    #[serde(default)]
    pub orgn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub orgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub scrt_seln_vol: String,
    #[serde(default)]
    pub scrt_shnu_vol: String,
    #[serde(default)]
    pub scrt_ntby_qty: String,
    #[serde(default)]
    pub scrt_seln_tr_pbmn: String,
    #[serde(default)]
    pub scrt_shnu_tr_pbmn: String,
    #[serde(default)]
    pub scrt_ntby_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_seln_vol: String,
    #[serde(default)]
    pub ivtr_shnu_vol: String,
    #[serde(default)]
    pub ivtr_ntby_qty: String,
    #[serde(default)]
    pub ivtr_seln_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_shnu_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_ntby_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_seln_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_seln_vol: String,
    #[serde(default)]
    pub pe_fund_ntby_vol: String,
    #[serde(default)]
    pub pe_fund_shnu_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_shnu_vol: String,
    #[serde(default)]
    pub pe_fund_ntby_tr_pbmn: String,
    #[serde(default)]
    pub bank_seln_vol: String,
    #[serde(default)]
    pub bank_shnu_vol: String,
    #[serde(default)]
    pub bank_ntby_qty: String,
    #[serde(default)]
    pub bank_seln_tr_pbmn: String,
    #[serde(default)]
    pub bank_shnu_tr_pbmn: String,
    #[serde(default)]
    pub bank_ntby_tr_pbmn: String,
    #[serde(default)]
    pub insu_seln_vol: String,
    #[serde(default)]
    pub insu_shnu_vol: String,
    #[serde(default)]
    pub insu_ntby_qty: String,
    #[serde(default)]
    pub insu_seln_tr_pbmn: String,
    #[serde(default)]
    pub insu_shnu_tr_pbmn: String,
    #[serde(default)]
    pub insu_ntby_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_seln_vol: String,
    #[serde(default)]
    pub mrbn_shnu_vol: String,
    #[serde(default)]
    pub mrbn_ntby_qty: String,
    #[serde(default)]
    pub mrbn_seln_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub fund_seln_vol: String,
    #[serde(default)]
    pub fund_shnu_vol: String,
    #[serde(default)]
    pub fund_ntby_qty: String,
    #[serde(default)]
    pub fund_seln_tr_pbmn: String,
    #[serde(default)]
    pub fund_shnu_tr_pbmn: String,
    #[serde(default)]
    pub fund_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_seln_vol: String,
    #[serde(default)]
    pub etc_orgt_shnu_vol: String,
    #[serde(default)]
    pub etc_orgt_ntby_vol: String,
    #[serde(default)]
    pub etc_orgt_seln_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_shnu_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_seln_vol: String,
    #[serde(default)]
    pub etc_corp_shnu_vol: String,
    #[serde(default)]
    pub etc_corp_ntby_vol: String,
    #[serde(default)]
    pub etc_corp_seln_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_shnu_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_ntby_tr_pbmn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_input_iscd_2", req.fid_input_iscd_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
