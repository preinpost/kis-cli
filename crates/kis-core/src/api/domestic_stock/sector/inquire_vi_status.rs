//! 변동성완화장치(VI) 현황 — GET /uapi/domestic-stock/v1/quotations/inquire-vi-status
//!
//! 스펙: .agent/specs/domestic_stock__sector__inquire_vi_status.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-vi-status";
pub const TR_ID: &str = "FHPST01390000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 0 전체, 1 상승, 2 하락
    pub fid_div_cls_code: String,
    /// 20139
    pub fid_cond_scr_div_code: String,
    /// 0 전체, K 거래소, Q 코스닥
    pub fid_mrkt_cls_code: String,
    pub fid_input_iscd: String,
    /// 0 전체, 1 정적, 2 동적, 3 정적&동적
    pub fid_rank_sort_cls_code: String,
    /// YYYYMMDD
    pub fid_input_date_1: String,
    pub fid_trgt_cls_code: String,
    pub fid_trgt_exls_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    /// Y 발동 / N 해제
    #[serde(default)]
    pub vi_cls_code: String,
    #[serde(default)]
    pub bsop_date: String,
    #[serde(default)]
    pub cntg_vi_hour: String,
    #[serde(default)]
    pub vi_cncl_hour: String,
    /// 1 정적, 2 동적, 3 정적&동적
    #[serde(default)]
    pub vi_kind_code: String,
    #[serde(default)]
    pub vi_prc: String,
    #[serde(default)]
    pub vi_stnd_prc: String,
    #[serde(default)]
    pub vi_dprt: String,
    #[serde(default)]
    pub vi_dmc_stnd_prc: String,
    #[serde(default)]
    pub vi_dmc_dprt: String,
    #[serde(default)]
    pub vi_count: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_RANK_SORT_CLS_CODE", req.fid_rank_sort_cls_code.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_TRGT_CLS_CODE", req.fid_trgt_cls_code.as_str()),
        ("FID_TRGT_EXLS_CLS_CODE", req.fid_trgt_exls_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    // output이 Array일 수도 Object일 수도 있음 → 첫 요소
    let parsed: Response = match output {
        serde_json::Value::Array(mut arr) => {
            let first = arr.pop().ok_or_else(|| anyhow!("output 배열 비어있음"))?;
            serde_json::from_value(first)?
        }
        v => serde_json::from_value(v)?,
    };
    Ok(parsed)
}
