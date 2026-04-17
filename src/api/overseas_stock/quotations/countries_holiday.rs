//! 해외결제일자조회 — GET /uapi/overseas-stock/v1/quotations/countries-holiday
//!
//! 스펙: .agent/specs/overseas_stock__quotations__countries_holiday.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/quotations/countries-holiday";
pub const TR_ID: &str = "CTOS5011R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub trad_dt: String,
    pub ctx_area_nk: String,
    pub ctx_area_fk: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub tr_natn_cd: String,
    #[serde(default)]
    pub tr_natn_name: String,
    #[serde(default)]
    pub natn_eng_abrv_cd: String,
    #[serde(default)]
    pub tr_mket_cd: String,
    #[serde(default)]
    pub tr_mket_name: String,
    #[serde(default)]
    pub acpl_sttl_dt: String,
    #[serde(default)]
    pub dmst_sttl_dt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외결제일자조회는 모의투자 미지원");
    }
    let params = [
        ("TRAD_DT", req.trad_dt.as_str()),
        ("CTX_AREA_NK", req.ctx_area_nk.as_str()),
        ("CTX_AREA_FK", req.ctx_area_fk.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
