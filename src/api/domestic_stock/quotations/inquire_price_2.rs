//! 주식현재가 시세2 — GET /uapi/domestic-stock/v1/quotations/inquire-price-2
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_price_2.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-price-2";
pub const TR_ID: &str = "FHPST01010000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
    #[serde(default)]
    pub new_hgpr_lwpr_cls_code: String,
    #[serde(default)]
    pub mxpr_llam_cls_code: String,
    #[serde(default)]
    pub crdt_able_yn: String,
    #[serde(default)]
    pub stck_mxpr: String,
    #[serde(default)]
    pub elw_pblc_yn: String,
    #[serde(default)]
    pub prdy_clpr_vrss_oprc_rate: String,
    #[serde(default)]
    pub crdt_rate: String,
    #[serde(default)]
    pub marg_rate: String,
    #[serde(default)]
    pub lwpr_vrss_prpr: String,
    #[serde(default)]
    pub lwpr_vrss_prpr_sign: String,
    #[serde(default)]
    pub prdy_clpr_vrss_lwpr_rate: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub hgpr_vrss_prpr: String,
    #[serde(default)]
    pub hgpr_vrss_prpr_sign: String,
    #[serde(default)]
    pub prdy_clpr_vrss_hgpr_rate: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub oprc_vrss_prpr: String,
    #[serde(default)]
    pub oprc_vrss_prpr_sign: String,
    #[serde(default)]
    pub mang_issu_yn: String,
    #[serde(default)]
    pub divi_app_cls_code: String,
    #[serde(default)]
    pub short_over_yn: String,
    #[serde(default)]
    pub mrkt_warn_cls_code: String,
    #[serde(default)]
    pub invt_caful_yn: String,
    #[serde(default)]
    pub stange_runup_yn: String,
    #[serde(default)]
    pub ssts_hot_yn: String,
    #[serde(default)]
    pub low_current_yn: String,
    #[serde(default)]
    pub vi_cls_code: String,
    #[serde(default)]
    pub short_over_cls_code: String,
    #[serde(default)]
    pub stck_llam: String,
    #[serde(default)]
    pub new_lstn_cls_name: String,
    #[serde(default)]
    pub vlnt_deal_cls_name: String,
    #[serde(default)]
    pub flng_cls_name: String,
    #[serde(default)]
    pub revl_issu_reas_name: String,
    #[serde(default)]
    pub mrkt_warn_cls_name: String,
    #[serde(default)]
    pub stck_sdpr: String,
    #[serde(default)]
    pub bstp_cls_code: String,
    #[serde(default)]
    pub stck_prdy_clpr: String,
    #[serde(default)]
    pub insn_pbnt_yn: String,
    #[serde(default)]
    pub fcam_mod_cls_name: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vrss_vol_rate: String,
    #[serde(default)]
    pub bstp_kor_isnm: String,
    #[serde(default)]
    pub sltr_yn: String,
    #[serde(default)]
    pub trht_yn: String,
    #[serde(default)]
    pub oprc_rang_cont_yn: String,
    #[serde(default)]
    pub vlnt_fin_cls_code: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub prdy_vol: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("주식현재가 시세2는 모의투자 미지원 API입니다");
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
