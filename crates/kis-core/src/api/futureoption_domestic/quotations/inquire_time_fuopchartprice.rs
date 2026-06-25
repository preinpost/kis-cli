//! 선물옵션 분봉조회 — GET /uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__inquire_time_fuopchartprice.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/inquire-time-fuopchartprice";
pub const TR_ID: &str = "FHKIF03020200";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_hour_cls_code: String,
    pub fid_pw_data_incu_yn: String,
    pub fid_fake_tick_incu_yn: String,
    pub fid_input_date_1: String,
    pub fid_input_hour_1: String,
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
    pub prdy_nmix: String,
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
pub struct Bar {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub futs_prpr: String,
    #[serde(default)]
    pub futs_oprc: String,
    #[serde(default)]
    pub futs_hgpr: String,
    #[serde(default)]
    pub futs_lwpr: String,
    #[serde(default)]
    pub cntg_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Vec<Meta>,
    pub bars: Vec<Bar>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("선물옵션 분봉조회는 모의투자 미지원");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_HOUR_CLS_CODE", req.fid_hour_cls_code.as_str()),
        ("FID_PW_DATA_INCU_YN", req.fid_pw_data_incu_yn.as_str()),
        ("FID_FAKE_TICK_INCU_YN", req.fid_fake_tick_incu_yn.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .map(serde_json::from_value::<Vec<Meta>>)
        .transpose()?
        .unwrap_or_default();
    let bars = resp
        .output2
        .map(serde_json::from_value::<Vec<Bar>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, bars })
}
