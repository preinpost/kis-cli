//! 주식현재가 회원사 — GET /uapi/domestic-stock/v1/quotations/inquire-member
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_member.md
//!
//! output은 Array이지만 단일 요소만 반환됨 (매도/매수 5개사 합산 구조).

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-member";
pub const TR_ID: &str = "FHKST01010600";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub seln_mbcr_no1: String,
    #[serde(default)]
    pub seln_mbcr_no2: String,
    #[serde(default)]
    pub seln_mbcr_no3: String,
    #[serde(default)]
    pub seln_mbcr_no4: String,
    #[serde(default)]
    pub seln_mbcr_no5: String,
    #[serde(default)]
    pub seln_mbcr_name1: String,
    #[serde(default)]
    pub seln_mbcr_name2: String,
    #[serde(default)]
    pub seln_mbcr_name3: String,
    #[serde(default)]
    pub seln_mbcr_name4: String,
    #[serde(default)]
    pub seln_mbcr_name5: String,
    #[serde(default)]
    pub total_seln_qty1: String,
    #[serde(default)]
    pub total_seln_qty2: String,
    #[serde(default)]
    pub total_seln_qty3: String,
    #[serde(default)]
    pub total_seln_qty4: String,
    #[serde(default)]
    pub total_seln_qty5: String,
    #[serde(default)]
    pub seln_mbcr_rlim1: String,
    #[serde(default)]
    pub seln_mbcr_rlim2: String,
    #[serde(default)]
    pub seln_mbcr_rlim3: String,
    #[serde(default)]
    pub seln_mbcr_rlim4: String,
    #[serde(default)]
    pub seln_mbcr_rlim5: String,
    #[serde(default)]
    pub seln_qty_icdc1: String,
    #[serde(default)]
    pub seln_qty_icdc2: String,
    #[serde(default)]
    pub seln_qty_icdc3: String,
    #[serde(default)]
    pub seln_qty_icdc4: String,
    #[serde(default)]
    pub seln_qty_icdc5: String,
    #[serde(default)]
    pub shnu_mbcr_no1: String,
    #[serde(default)]
    pub shnu_mbcr_no2: String,
    #[serde(default)]
    pub shnu_mbcr_no3: String,
    #[serde(default)]
    pub shnu_mbcr_no4: String,
    #[serde(default)]
    pub shnu_mbcr_no5: String,
    #[serde(default)]
    pub shnu_mbcr_name1: String,
    #[serde(default)]
    pub shnu_mbcr_name2: String,
    #[serde(default)]
    pub shnu_mbcr_name3: String,
    #[serde(default)]
    pub shnu_mbcr_name4: String,
    #[serde(default)]
    pub shnu_mbcr_name5: String,
    #[serde(default)]
    pub total_shnu_qty1: String,
    #[serde(default)]
    pub total_shnu_qty2: String,
    #[serde(default)]
    pub total_shnu_qty3: String,
    #[serde(default)]
    pub total_shnu_qty4: String,
    #[serde(default)]
    pub total_shnu_qty5: String,
    #[serde(default)]
    pub shnu_mbcr_rlim1: String,
    #[serde(default)]
    pub shnu_mbcr_rlim2: String,
    #[serde(default)]
    pub shnu_mbcr_rlim3: String,
    #[serde(default)]
    pub shnu_mbcr_rlim4: String,
    #[serde(default)]
    pub shnu_mbcr_rlim5: String,
    #[serde(default)]
    pub shnu_qty_icdc1: String,
    #[serde(default)]
    pub shnu_qty_icdc2: String,
    #[serde(default)]
    pub shnu_qty_icdc3: String,
    #[serde(default)]
    pub shnu_qty_icdc4: String,
    #[serde(default)]
    pub shnu_qty_icdc5: String,
    #[serde(default)]
    pub glob_total_seln_qty: String,
    #[serde(default)]
    pub glob_seln_rlim: String,
    #[serde(default)]
    pub glob_ntby_qty: String,
    #[serde(default)]
    pub glob_total_shnu_qty: String,
    #[serde(default)]
    pub glob_shnu_rlim: String,
    #[serde(default)]
    pub seln_mbcr_glob_yn_1: String,
    #[serde(default)]
    pub seln_mbcr_glob_yn_2: String,
    #[serde(default)]
    pub seln_mbcr_glob_yn_3: String,
    #[serde(default)]
    pub seln_mbcr_glob_yn_4: String,
    #[serde(default)]
    pub seln_mbcr_glob_yn_5: String,
    #[serde(default)]
    pub shnu_mbcr_glob_yn_1: String,
    #[serde(default)]
    pub shnu_mbcr_glob_yn_2: String,
    #[serde(default)]
    pub shnu_mbcr_glob_yn_3: String,
    #[serde(default)]
    pub shnu_mbcr_glob_yn_4: String,
    #[serde(default)]
    pub shnu_mbcr_glob_yn_5: String,
    #[serde(default)]
    pub glob_total_seln_qty_icdc: String,
    #[serde(default)]
    pub glob_total_shnu_qty_icdc: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    // output이 Array/Object 모두 가능. 단일 객체로 파싱.
    let parsed: Response = match output {
        serde_json::Value::Array(mut arr) => {
            let first = arr.pop().ok_or_else(|| anyhow!("output 배열 비어있음"))?;
            serde_json::from_value(first)?
        }
        v => serde_json::from_value(v)?,
    };
    Ok(parsed)
}
