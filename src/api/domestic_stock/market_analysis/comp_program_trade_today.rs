//! 프로그램매매 종합현황(시간) — GET /uapi/domestic-stock/v1/quotations/comp-program-trade-today

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/comp-program-trade-today";
pub const TR_ID: &str = "FHPPG04600101";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_mrkt_cls_code: String,
    pub fid_sctn_cls_code: String,
    pub fid_input_iscd: String,
    pub fid_cond_mrkt_div_code1: String,
    pub fid_input_hour_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bsop_hour: String,
    #[serde(default)]
    pub arbt_smtn_seln_tr_pbmn: String,
    #[serde(default)]
    pub arbt_smtm_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_smtn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub arbt_smtm_shun_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_smtn_seln_tr_pbmn: String,
    #[serde(default)]
    pub nabt_smtm_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_smtn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub nabt_smtm_shun_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_smtn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub arbt_smtm_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_smtn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub nabt_smtm_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub whol_smtn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub whol_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub bstp_nmix_prpr: String,
    #[serde(default)]
    pub bstp_nmix_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_SCTN_CLS_CODE", req.fid_sctn_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_COND_MRKT_DIV_CODE1", req.fid_cond_mrkt_div_code1.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
