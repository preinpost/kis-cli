//! 주식현재가 투자자 — GET /uapi/domestic-stock/v1/quotations/inquire-investor
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_investor.md
//!
//! output이 Array (일자별 투자자 순매수). 당일 데이터는 장 종료 후 제공.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-investor";
pub const TR_ID: &str = "FHKST01010900";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J:KRX, NX:NXT, UN:통합
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
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
    pub prsn_ntby_qty: String,
    #[serde(default)]
    pub frgn_ntby_qty: String,
    #[serde(default)]
    pub orgn_ntby_qty: String,
    #[serde(default)]
    pub prsn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub frgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub orgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub prsn_shnu_vol: String,
    #[serde(default)]
    pub frgn_shnu_vol: String,
    #[serde(default)]
    pub orgn_shnu_vol: String,
    #[serde(default)]
    pub prsn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub frgn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub orgn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub prsn_seln_vol: String,
    #[serde(default)]
    pub frgn_seln_vol: String,
    #[serde(default)]
    pub orgn_seln_vol: String,
    #[serde(default)]
    pub prsn_seln_tr_pbmn: String,
    #[serde(default)]
    pub frgn_seln_tr_pbmn: String,
    #[serde(default)]
    pub orgn_seln_tr_pbmn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
