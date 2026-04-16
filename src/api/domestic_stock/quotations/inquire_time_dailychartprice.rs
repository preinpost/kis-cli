//! 주식일별분봉조회 — GET /uapi/domestic-stock/v1/quotations/inquire-time-dailychartprice
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_time_dailychartprice.md
//!
//! 모의투자 미지원. 과거일자 분봉 (최대 1년 보관). output1(Meta) + output2(분봉 Vec).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-time-dailychartprice";
pub const TR_ID: &str = "FHKST03010230";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// 13시 = "130000"
    pub fid_input_hour_1: String,
    /// YYYYMMDD
    pub fid_input_date_1: String,
    pub fid_pw_data_incu_yn: String,
    /// 공백 필수
    pub fid_fake_tick_incu_yn: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub stck_prdy_clpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub stck_prpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bar {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub cntg_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub bars: Vec<Bar>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("주식일별분봉조회는 모의투자 미지원 API입니다");
    }
    let fake = req.fid_fake_tick_incu_yn.as_deref().unwrap_or("");
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_PW_DATA_INCU_YN", req.fid_pw_data_incu_yn.as_str()),
        ("FID_FAKE_TICK_INCU_YN", fake),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let bars: Vec<Bar> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, bars })
}
