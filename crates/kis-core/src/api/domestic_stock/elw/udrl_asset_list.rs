//! ELW 기초자산 목록조회 — GET /uapi/elw/v1/quotations/udrl-asset-list
//!
//! 스펙: .agent/specs/domestic_stock__elw__udrl_asset_list.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/udrl-asset-list";
pub const TR_ID: &str = "FHKEW154100C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 11541
    pub fid_cond_scr_div_code: String,
    /// 0 종목명, 1 콜발행, 2 풋발행, 3 상승율, 4 하락율, 5 현재가, 6 종목코드
    pub fid_rank_sort_cls_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
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
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 기초자산 목록조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
