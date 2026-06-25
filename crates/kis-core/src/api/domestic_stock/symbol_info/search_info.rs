//! 상품기본조회 — GET /uapi/domestic-stock/v1/quotations/search-info
//!
//! 스펙: .agent/specs/domestic_stock__symbol_info__search_info.md
//!
//! 다양한 상품유형(주식/선물옵션/채권/미국/홍콩/일본/베트남/중국)의 기본정보.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/search-info";
pub const TR_ID: &str = "CTPF1604R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub pdno: String,
    /// 300 주식, 301 선물옵션, 302 채권, 512~ 해외
    pub prdt_type_cd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub prdt_name120: String,
    #[serde(default)]
    pub prdt_abrv_name: String,
    #[serde(default)]
    pub prdt_eng_name: String,
    #[serde(default)]
    pub prdt_eng_name120: String,
    #[serde(default)]
    pub prdt_eng_abrv_name: String,
    #[serde(default)]
    pub std_pdno: String,
    #[serde(default)]
    pub shtn_pdno: String,
    #[serde(default)]
    pub prdt_sale_stat_cd: String,
    #[serde(default)]
    pub prdt_risk_grad_cd: String,
    #[serde(default)]
    pub prdt_clsf_cd: String,
    #[serde(default)]
    pub prdt_clsf_name: String,
    #[serde(default)]
    pub sale_strt_dt: String,
    #[serde(default)]
    pub sale_end_dt: String,
    #[serde(default)]
    pub wrap_asst_type_cd: String,
    #[serde(default)]
    pub ivst_prdt_type_cd: String,
    #[serde(default)]
    pub ivst_prdt_type_cd_name: String,
    #[serde(default)]
    pub frst_erlm_dt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("PDNO", req.pdno.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
