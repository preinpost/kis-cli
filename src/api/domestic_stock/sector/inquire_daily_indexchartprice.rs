//! 국내주식업종기간별시세(일/주/월/년) — GET /uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice
//!
//! 스펙: .agent/specs/domestic_stock__sector__inquire_daily_indexchartprice.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice";
pub const TR_ID: &str = "FHKUP03500100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// U
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// YYYYMMDD
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
    /// D/W/M/Y
    pub fid_period_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub bstp_nmix_prdy_ctrt: String,
    #[serde(default)]
    pub prdy_nmix: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_cls_code: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub bstp_nmix_oprc: String,
    #[serde(default)]
    pub bstp_nmix_hgpr: String,
    #[serde(default)]
    pub bstp_nmix_lwpr: String,
    #[serde(default)]
    pub futs_prdy_oprc: String,
    #[serde(default)]
    pub futs_prdy_hgpr: String,
    #[serde(default)]
    pub futs_prdy_lwpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Candle {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_oprc: String,
    #[serde(default)]
    pub bstp_nmix_hgpr: String,
    #[serde(default)]
    pub bstp_nmix_lwpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub mod_yn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub candles: Vec<Candle>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
        ("FID_PERIOD_DIV_CODE", req.fid_period_div_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let candles: Vec<Candle> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, candles })
}
