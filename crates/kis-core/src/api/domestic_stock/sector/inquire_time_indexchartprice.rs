//! 업종 분봉조회 — GET /uapi/domestic-stock/v1/quotations/inquire-time-indexchartprice
//!
//! 스펙: .agent/specs/domestic_stock__sector__inquire_time_indexchartprice.md
//!
//! Output1/Output2 대문자 (스펙 표기). 실제 응답은 소문자일 수 있음.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-time-indexchartprice";
pub const TR_ID: &str = "FHKUP03500200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// 0 기본, 1 장마감/시간외 제외
    pub fid_etc_cls_code: String,
    pub fid_input_iscd: String,
    /// 30, 60 → 1분, 600 → 10분, 3600 → 1시간
    pub fid_input_hour_1: String,
    pub fid_pw_data_incu_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
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
pub struct Bar {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_oprc: String,
    #[serde(default)]
    pub bstp_nmix_hgpr: String,
    #[serde(default)]
    pub bstp_nmix_lwpr: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub bars: Vec<Bar>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_ETC_CLS_CODE", req.fid_etc_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
        ("FID_PW_DATA_INCU_YN", req.fid_pw_data_incu_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    // output1은 array지만 단일 meta로 간주 (첫 요소)
    let meta = resp.output1.and_then(|v| match v {
        serde_json::Value::Array(mut arr) => arr.pop().and_then(|x| serde_json::from_value::<Meta>(x).ok()),
        other => serde_json::from_value::<Meta>(other).ok(),
    });
    let bars: Vec<Bar> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, bars })
}
