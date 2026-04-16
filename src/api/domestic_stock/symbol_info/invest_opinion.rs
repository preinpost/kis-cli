//! 국내주식 종목투자의견 — GET /uapi/domestic-stock/v1/quotations/invest-opinion

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/invest-opinion";
pub const TR_ID: &str = "FHKST663300C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub invt_opnn: String,
    #[serde(default)]
    pub invt_opnn_cls_code: String,
    #[serde(default)]
    pub rgbf_invt_opnn: String,
    #[serde(default)]
    pub rgbf_invt_opnn_cls_code: String,
    #[serde(default)]
    pub mbcr_name: String,
    #[serde(default)]
    pub hts_goal_prc: String,
    #[serde(default)]
    pub stck_prdy_clpr: String,
    #[serde(default)]
    pub stck_nday_esdg: String,
    #[serde(default)]
    pub nday_dprt: String,
    #[serde(default)]
    pub stft_esdg: String,
    #[serde(default)]
    pub dprt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
