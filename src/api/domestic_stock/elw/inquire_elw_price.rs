//! ELW 현재가 시세 — GET /uapi/domestic-stock/v1/quotations/inquire-elw-price
//!
//! 스펙: .agent/specs/domestic_stock__elw__inquire_elw_price.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-elw-price";
pub const TR_ID: &str = "FHKEW15010000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// W (ELW 고정)
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vrss_vol_rate: String,
    #[serde(default)]
    pub unas_shrn_iscd: String,
    #[serde(default)]
    pub unas_isnm: String,
    #[serde(default)]
    pub unas_prpr: String,
    #[serde(default)]
    pub unas_prdy_vrss: String,
    #[serde(default)]
    pub unas_prdy_vrss_sign: String,
    #[serde(default)]
    pub unas_prdy_ctrt: String,
    #[serde(default)]
    pub bidp: String,
    #[serde(default)]
    pub askp: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub vol_tnrt: String,
    #[serde(default)]
    pub elw_oprc: String,
    #[serde(default)]
    pub elw_hgpr: String,
    #[serde(default)]
    pub elw_lwpr: String,
    #[serde(default)]
    pub stck_prdy_clpr: String,
    #[serde(default)]
    pub hts_thpr: String,
    #[serde(default)]
    pub dprt: String,
    #[serde(default)]
    pub atm_cls_name: String,
    #[serde(default)]
    pub hts_ints_vltl: String,
    #[serde(default)]
    pub acpr: String,
    #[serde(default)]
    pub pvt_scnd_dmrs_prc: String,
    #[serde(default)]
    pub pvt_frst_dmrs_prc: String,
    #[serde(default)]
    pub pvt_pont_val: String,
    #[serde(default)]
    pub pvt_frst_dmsp_prc: String,
    #[serde(default)]
    pub pvt_scnd_dmsp_prc: String,
    #[serde(default)]
    pub dmsp_val: String,
    #[serde(default)]
    pub dmrs_val: String,
    #[serde(default)]
    pub elw_sdpr: String,
    #[serde(default)]
    pub apprch_rate: String,
    #[serde(default)]
    pub tick_conv_prc: String,
    #[serde(default)]
    pub invt_epmd_cntt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    // output1이 array일 수도 object일 수도 있음 (단일 ELW 종목 조회)
    let parsed: Response = match output {
        serde_json::Value::Array(mut arr) => {
            let first = arr.pop().ok_or_else(|| anyhow!("output1 배열 비어있음"))?;
            serde_json::from_value(first)?
        }
        v => serde_json::from_value(v)?,
    };
    Ok(parsed)
}
