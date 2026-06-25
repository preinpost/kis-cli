//! ELW 투자지표추이(분별) — GET /uapi/elw/v1/quotations/indicator-trend-minute
//!
//! 스펙: .agent/specs/domestic_stock__elw__indicator_trend_minute.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/indicator-trend-minute";
pub const TR_ID: &str = "FHPEW02740300";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// 60, 180, 300, 600, 1800, 3600, 7200
    pub fid_hour_cls_code: String,
    pub fid_pw_data_incu_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_cntg_hour: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub elw_oprc: String,
    #[serde(default)]
    pub elw_hgpr: String,
    #[serde(default)]
    pub elw_lwpr: String,
    #[serde(default)]
    pub lvrg_val: String,
    #[serde(default)]
    pub gear: String,
    #[serde(default)]
    pub prmm_val: String,
    #[serde(default)]
    pub invl_val: String,
    #[serde(default)]
    pub prit: String,
    #[serde(default)]
    pub acml_vol: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 투자지표추이(분별)은 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_HOUR_CLS_CODE", req.fid_hour_cls_code.as_str()),
        ("FID_PW_DATA_INCU_YN", req.fid_pw_data_incu_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
