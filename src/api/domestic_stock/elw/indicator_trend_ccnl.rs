//! ELW 투자지표추이(체결) — GET /uapi/elw/v1/quotations/indicator-trend-ccnl
//!
//! 스펙: .agent/specs/domestic_stock__elw__indicator_trend_ccnl.md
//!
//! 모의투자 미지원. 체결 단위 지표 추이.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/indicator-trend-ccnl";
pub const TR_ID: &str = "FHPEW02740100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_cntg_hour: String,
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
    pub lvrg_val: String,
    #[serde(default)]
    pub gear: String,
    #[serde(default)]
    pub tmvl_val: String,
    #[serde(default)]
    pub invl_val: String,
    #[serde(default)]
    pub prit: String,
    #[serde(default)]
    pub apprch_rate: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 투자지표추이(체결)은 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
