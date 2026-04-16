//! ETF 구성종목시세 — GET /uapi/etfetn/v1/quotations/inquire-component-stock-price
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_component_stock_price.md
//!
//! 모의투자 미지원. output1(ETF 자체 + NAV) + output2(구성종목 Vec).

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/etfetn/v1/quotations/inquire-component-stock-price";
pub const TR_ID: &str = "FHKST121600C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// Unique key 11216
    pub fid_cond_scr_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub etf_cnfg_issu_avls: String,
    #[serde(default)]
    pub nav: String,
    #[serde(default)]
    pub nav_prdy_vrss_sign: String,
    #[serde(default)]
    pub nav_prdy_vrss: String,
    #[serde(default)]
    pub nav_prdy_ctrt: String,
    #[serde(default)]
    pub etf_ntas_ttam: String,
    #[serde(default)]
    pub prdy_clpr_nav: String,
    #[serde(default)]
    pub oprc_nav: String,
    #[serde(default)]
    pub hprc_nav: String,
    #[serde(default)]
    pub lprc_nav: String,
    #[serde(default)]
    pub etf_cu_unit_scrt_cnt: String,
    #[serde(default)]
    pub etf_cnfg_issu_cnt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Component {
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub tday_rsfl_rate: String,
    #[serde(default)]
    pub prdy_vrss_vol: String,
    #[serde(default)]
    pub tr_pbmn_tnrt: String,
    #[serde(default)]
    pub hts_avls: String,
    #[serde(default)]
    pub etf_cnfg_issu_avls: String,
    #[serde(default)]
    pub etf_cnfg_issu_rlim: String,
    #[serde(default)]
    pub etf_vltn_amt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub components: Vec<Component>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("ETF 구성종목시세는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let components: Vec<Component> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, components })
}
