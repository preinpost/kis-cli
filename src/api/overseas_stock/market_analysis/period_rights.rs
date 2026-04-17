//! 해외주식 기간별권리조회 — GET /uapi/overseas-price/v1/quotations/period-rights
//!
//! 스펙: .agent/specs/overseas_stock__market_analysis__period_rights.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-price/v1/quotations/period-rights";
pub const TR_ID: &str = "CTRGT011R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub rght_type_cd: String,
    pub inqr_dvsn_cd: String,
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub pdno: String,
    pub prdt_type_cd: String,
    pub ctx_area_nk50: String,
    pub ctx_area_fk50: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bass_dt: String,
    #[serde(default)]
    pub rght_type_cd: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub std_pdno: String,
    #[serde(default)]
    pub acpl_bass_dt: String,
    #[serde(default)]
    pub sbsc_strt_dt: String,
    #[serde(default)]
    pub sbsc_end_dt: String,
    #[serde(default)]
    pub cash_alct_rt: String,
    #[serde(default)]
    pub stck_alct_rt: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub crcy_cd2: String,
    #[serde(default)]
    pub crcy_cd3: String,
    #[serde(default)]
    pub crcy_cd4: String,
    #[serde(default)]
    pub alct_frcr_unpr: String,
    #[serde(default)]
    pub stkp_dvdn_frcr_amt2: String,
    #[serde(default)]
    pub stkp_dvdn_frcr_amt3: String,
    #[serde(default)]
    pub stkp_dvdn_frcr_amt4: String,
    #[serde(default)]
    pub dfnt_yn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("해외주식 기간별권리조회는 모의투자 미지원");
    }
    let params = [
        ("RGHT_TYPE_CD", req.rght_type_cd.as_str()),
        ("INQR_DVSN_CD", req.inqr_dvsn_cd.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("CTX_AREA_NK50", req.ctx_area_nk50.as_str()),
        ("CTX_AREA_FK50", req.ctx_area_fk50.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
