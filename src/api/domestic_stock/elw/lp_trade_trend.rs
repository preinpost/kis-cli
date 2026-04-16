//! ELW LP매매추이 — GET /uapi/elw/v1/quotations/lp-trade-trend
//!
//! 스펙: .agent/specs/domestic_stock__elw__lp_trade_trend.md
//!
//! 모의투자 미지원. output1(ELW 기본정보) + output2(일자별 LP 매매).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/lp-trade-trend";
pub const TR_ID: &str = "FHPEW03760000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub stck_cnvr_rate: String,
    #[serde(default)]
    pub prit: String,
    #[serde(default)]
    pub lvrg_val: String,
    #[serde(default)]
    pub gear: String,
    #[serde(default)]
    pub prls_qryr_rate: String,
    #[serde(default)]
    pub cfp: String,
    #[serde(default)]
    pub invl_val: String,
    #[serde(default)]
    pub tmvl_val: String,
    #[serde(default)]
    pub acpr: String,
    #[serde(default)]
    pub elw_ko_barrier: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub lp_seln_qty: String,
    #[serde(default)]
    pub lp_seln_avrg_unpr: String,
    #[serde(default)]
    pub lp_shnu_qty: String,
    #[serde(default)]
    pub lp_shnu_avrg_unpr: String,
    #[serde(default)]
    pub lp_hvol: String,
    #[serde(default)]
    pub lp_hldn_rate: String,
    #[serde(default)]
    pub prsn_deal_qty: String,
    #[serde(default)]
    pub apprch_rate: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("ELW LP매매추이는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
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
