//! 국내주식기간별시세(일/주/월/년) — GET /uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_daily_itemchartprice.md
//!
//! 한 번 호출에 최대 100건. output1(종목 메타/현재가) + output2(캔들 Vec).

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice";
pub const TR_ID: &str = "FHKST03010100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J:KRX, NX:NXT, UN:통합
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// YYYYMMDD
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
    /// D 일봉, W 주봉, M 월봉, Y 년봉
    pub fid_period_div_code: String,
    /// 0 수정주가, 1 원주가
    pub fid_org_adj_prc: String,
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
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub stck_mxpr: String,
    #[serde(default)]
    pub stck_llam: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub stck_prdy_oprc: String,
    #[serde(default)]
    pub stck_prdy_hgpr: String,
    #[serde(default)]
    pub stck_prdy_lwpr: String,
    #[serde(default)]
    pub askp: String,
    #[serde(default)]
    pub bidp: String,
    #[serde(default)]
    pub prdy_vrss_vol: String,
    #[serde(default)]
    pub vol_tnrt: String,
    #[serde(default)]
    pub stck_fcam: String,
    #[serde(default)]
    pub lstn_stcn: String,
    #[serde(default)]
    pub cpfn: String,
    #[serde(default)]
    pub hts_avls: String,
    #[serde(default)]
    pub per: String,
    #[serde(default)]
    pub eps: String,
    #[serde(default)]
    pub pbr: String,
    #[serde(default)]
    pub itewhol_loan_rmnd_ratem: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Candle {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_clpr: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub flng_cls_code: String,
    #[serde(default)]
    pub prtt_rate: String,
    #[serde(default)]
    pub mod_yn: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub revl_issu_reas: String,
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
        ("FID_ORG_ADJ_PRC", req.fid_org_adj_prc.as_str()),
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
