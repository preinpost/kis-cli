//! ELW 당일급변종목 — GET /uapi/elw/v1/ranking/quick-change
//!
//! 스펙: .agent/specs/domestic_stock__elw__quick_change.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/ranking/quick-change";
pub const TR_ID: &str = "FHPEW02870000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    /// 20287
    pub fid_cond_scr_div_code: String,
    pub fid_unas_input_iscd: String,
    pub fid_input_iscd: String,
    /// A
    pub fid_mrkt_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_input_vol_1: String,
    pub fid_input_vol_2: String,
    /// 1 분, 2 일
    pub fid_hour_cls_code: String,
    pub fid_input_hour_1: String,
    pub fid_input_hour_2: String,
    /// 1 가격급등, 2 가격급락, 3 거래량급증, 4 매수잔량급증, 5 매도잔량급증
    pub fid_rank_sort_cls_code: String,
    /// 0 전체, 1 일반, 2 조기종료
    pub fid_blng_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub elw_kor_isnm: String,
    #[serde(default)]
    pub elw_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub askp: String,
    #[serde(default)]
    pub bidp: String,
    #[serde(default)]
    pub total_askp_rsqn: String,
    #[serde(default)]
    pub total_bidp_rsqn: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub stnd_val: String,
    #[serde(default)]
    pub stnd_val_vrss: String,
    #[serde(default)]
    pub stnd_val_ctrt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 당일급변종목은 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_UNAS_INPUT_ISCD", req.fid_unas_input_iscd.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_INPUT_PRICE_1", req.fid_input_price_1.as_str()),
        ("FID_INPUT_PRICE_2", req.fid_input_price_2.as_str()),
        ("FID_INPUT_VOL_1", req.fid_input_vol_1.as_str()),
        ("FID_INPUT_VOL_2", req.fid_input_vol_2.as_str()),
        ("FID_HOUR_CLS_CODE", req.fid_hour_cls_code.as_str()),
        ("FID_INPUT_HOUR_1", req.fid_input_hour_1.as_str()),
        ("FID_INPUT_HOUR_2", req.fid_input_hour_2.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
