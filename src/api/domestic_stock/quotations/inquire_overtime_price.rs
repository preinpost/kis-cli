//! 국내주식 시간외현재가 — GET /uapi/domestic-stock/v1/quotations/inquire-overtime-price
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_overtime_price.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-overtime-price";
pub const TR_ID: &str = "FHPST02300000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J 주식
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub bstp_kor_isnm: String,
    #[serde(default)]
    pub mang_issu_cls_name: String,
    #[serde(default)]
    pub ovtm_untp_prpr: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss: String,
    #[serde(default)]
    pub ovtm_untp_prdy_vrss_sign: String,
    #[serde(default)]
    pub ovtm_untp_prdy_ctrt: String,
    #[serde(default)]
    pub ovtm_untp_vol: String,
    #[serde(default)]
    pub ovtm_untp_tr_pbmn: String,
    #[serde(default)]
    pub ovtm_untp_mxpr: String,
    #[serde(default)]
    pub ovtm_untp_llam: String,
    #[serde(default)]
    pub ovtm_untp_oprc: String,
    #[serde(default)]
    pub ovtm_untp_hgpr: String,
    #[serde(default)]
    pub ovtm_untp_lwpr: String,
    #[serde(default)]
    pub marg_rate: String,
    #[serde(default)]
    pub ovtm_untp_antc_cnpr: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_vrss: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_vrss_sign: String,
    #[serde(default)]
    pub ovtm_untp_antc_cntg_ctrt: String,
    #[serde(default)]
    pub ovtm_untp_antc_cnqn: String,
    #[serde(default)]
    pub crdt_able_yn: String,
    #[serde(default)]
    pub new_lstn_cls_name: String,
    #[serde(default)]
    pub sltr_yn: String,
    #[serde(default)]
    pub mang_issu_yn: String,
    #[serde(default)]
    pub mrkt_warn_cls_code: String,
    #[serde(default)]
    pub trht_yn: String,
    #[serde(default)]
    pub vlnt_deal_cls_name: String,
    #[serde(default)]
    pub ovtm_untp_sdpr: String,
    #[serde(default)]
    pub mrkt_warn_cls_name: String,
    #[serde(default)]
    pub revl_issu_reas_name: String,
    #[serde(default)]
    pub insn_pbnt_yn: String,
    #[serde(default)]
    pub flng_cls_name: String,
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
    #[serde(default)]
    pub ovtm_vi_cls_code: String,
    #[serde(default)]
    pub bidp: String,
    #[serde(default)]
    pub askp: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("국내주식 시간외현재가는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
