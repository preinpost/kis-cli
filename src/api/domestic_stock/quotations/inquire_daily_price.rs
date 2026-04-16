//! 주식현재가 일자별 — GET /uapi/domestic-stock/v1/quotations/inquire-daily-price
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_daily_price.md
//!
//! 일/주/월별 주가 (최근 30개). output이 array.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-daily-price";
pub const TR_ID: &str = "FHKST01010400";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J:KRX, NX:NXT, UN:통합
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    /// D 일, W 주, M 월
    pub fid_period_div_code: String,
    /// 0 미반영, 1 반영
    pub fid_org_adj_prc: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub stck_clpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vrss_vol_rate: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub hts_frgn_ehrt: String,
    #[serde(default)]
    pub frgn_ntby_qty: String,
    #[serde(default)]
    pub flng_cls_code: String,
    #[serde(default)]
    pub acml_prtt_rate: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_PERIOD_DIV_CODE", req.fid_period_div_code.as_str()),
        ("FID_ORG_ADJ_PRC", req.fid_org_adj_prc.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
