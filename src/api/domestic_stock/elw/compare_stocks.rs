//! ELW 비교대상종목조회 — GET /uapi/elw/v1/quotations/compare-stocks
//!
//! 스펙: .agent/specs/domestic_stock__elw__compare_stocks.md
//!
//! 모의투자 미지원. 단일 종목 비교대상 조회.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/compare-stocks";
pub const TR_ID: &str = "FHKEW151701C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 11517
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub elw_kor_isnm: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("ELW 비교대상종목조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
