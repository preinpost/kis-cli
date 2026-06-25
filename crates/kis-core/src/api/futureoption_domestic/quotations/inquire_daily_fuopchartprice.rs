//! 선물옵션기간별시세(일/주/월/년) — GET /uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__inquire_daily_fuopchartprice.md

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice";
pub const TR_ID: &str = "FHKIF03020100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
    pub fid_period_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub futs_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub futs_prdy_ctrt: String,
    #[serde(default)]
    pub futs_prdy_clpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub futs_prpr: String,
    #[serde(default)]
    pub futs_shrn_iscd: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub futs_mxpr: String,
    #[serde(default)]
    pub futs_llam: String,
    #[serde(default)]
    pub futs_oprc: String,
    #[serde(default)]
    pub futs_hgpr: String,
    #[serde(default)]
    pub futs_lwpr: String,
    #[serde(default)]
    pub futs_prdy_oprc: String,
    #[serde(default)]
    pub futs_prdy_hgpr: String,
    #[serde(default)]
    pub futs_prdy_lwpr: String,
    #[serde(default)]
    pub futs_askp: String,
    #[serde(default)]
    pub futs_bidp: String,
    #[serde(default)]
    pub basis: String,
    #[serde(default)]
    pub kospi200_nmix: String,
    #[serde(default)]
    pub kospi200_prdy_vrss: String,
    #[serde(default)]
    pub kospi200_prdy_ctrt: String,
    #[serde(default)]
    pub kospi200_prdy_vrss_sign: String,
    #[serde(default)]
    pub hts_otst_stpl_qty: String,
    #[serde(default)]
    pub otst_stpl_qty_icdc: String,
    #[serde(default)]
    pub tday_rltv: String,
    #[serde(default)]
    pub hts_thpr: String,
    #[serde(default)]
    pub dprt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Candle {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub futs_prpr: String,
    #[serde(default)]
    pub futs_oprc: String,
    #[serde(default)]
    pub futs_hgpr: String,
    #[serde(default)]
    pub futs_lwpr: String,
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
    let candles = resp
        .output2
        .map(serde_json::from_value::<Vec<Candle>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, candles })
}
