//! 관심종목(멀티종목) 시세조회 — GET /uapi/domestic-stock/v1/quotations/intstock-multprice
//!
//! 스펙: .agent/specs/domestic_stock__market_analysis__intstock_multprice.md
//!
//! 최대 30개 종목을 한번에 조회. Request는 (market_code, iscd) 튜플 Vec.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/intstock-multprice";
pub const TR_ID: &str = "FHKST11300006";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 최대 30개. (FID_COND_MRKT_DIV_CODE, FID_INPUT_ISCD).
    pub stocks: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub kospi_kosdaq_cls_name: String,
    #[serde(default)]
    pub mrkt_trtm_cls_name: String,
    #[serde(default)]
    pub hour_cls_code: String,
    #[serde(default)]
    pub inter_shrn_iscd: String,
    #[serde(default)]
    pub inter_kor_isnm: String,
    #[serde(default)]
    pub inter2_prpr: String,
    #[serde(default)]
    pub inter2_prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub inter2_oprc: String,
    #[serde(default)]
    pub inter2_hgpr: String,
    #[serde(default)]
    pub inter2_lwpr: String,
    #[serde(default)]
    pub inter2_llam: String,
    #[serde(default)]
    pub inter2_mxpr: String,
    #[serde(default)]
    pub inter2_askp: String,
    #[serde(default)]
    pub inter2_bidp: String,
    #[serde(default)]
    pub seln_rsqn: String,
    #[serde(default)]
    pub shnu_rsqn: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub inter2_prdy_clpr: String,
    #[serde(default)]
    pub oprc_vrss_hgpr_rate: String,
    #[serde(default)]
    pub intr_antc_cntg_vrss: String,
    #[serde(default)]
    pub intr_antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub intr_antc_cntg_prdy_ctrt: String,
    #[serde(default)]
    pub intr_antc_vol: String,
    #[serde(default)]
    pub inter2_sdpr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if req.stocks.len() > 30 {
        return Err(anyhow!("최대 30종목까지만 조회 가능 (주어진 수: {})", req.stocks.len()));
    }
    // FID_COND_MRKT_DIV_CODE_1 ~ _30, FID_INPUT_ISCD_1 ~ _30 쌍을 생성
    let mut keys: Vec<String> = Vec::with_capacity(60);
    let mut values: Vec<String> = Vec::with_capacity(60);
    for (i, (mkt, iscd)) in req.stocks.iter().enumerate() {
        let idx = i + 1;
        keys.push(format!("FID_COND_MRKT_DIV_CODE_{idx}"));
        values.push(mkt.clone());
        keys.push(format!("FID_INPUT_ISCD_{idx}"));
        values.push(iscd.clone());
    }
    let params: Vec<(&str, &str)> = keys.iter().zip(values.iter()).map(|(k, v)| (k.as_str(), v.as_str())).collect();
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output2.ok_or_else(|| anyhow!("응답에 output2 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
