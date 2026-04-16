//! ETF/ETN 현재가 — GET /uapi/etfetn/v1/quotations/inquire-price
//!
//! 스펙: .agent/specs/domestic_stock__quotations__etfetn_inquire_price.md
//!
//! 모의투자 미지원. NAV 및 ETF 구성정보 포함.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/etfetn/v1/quotations/inquire-price";
pub const TR_ID: &str = "FHPST02400000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_input_iscd: String,
    pub fid_cond_mrkt_div_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub stck_mxpr: String,
    #[serde(default)]
    pub stck_llam: String,
    #[serde(default)]
    pub stck_prdy_clpr: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub prdy_clpr_vrss_oprc_rate: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub prdy_clpr_vrss_hgpr_rate: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub prdy_clpr_vrss_lwpr_rate: String,
    #[serde(default)]
    pub prdy_last_nav: String,
    #[serde(default)]
    pub nav: String,
    #[serde(default)]
    pub nav_prdy_vrss: String,
    #[serde(default)]
    pub nav_prdy_vrss_sign: String,
    #[serde(default)]
    pub nav_prdy_ctrt: String,
    #[serde(default)]
    pub trc_errt: String,
    #[serde(default)]
    pub stck_sdpr: String,
    #[serde(default)]
    pub stck_sspr: String,
    #[serde(default)]
    pub nmix_ctrt: String,
    #[serde(default)]
    pub etf_crcl_stcn: String,
    #[serde(default)]
    pub etf_ntas_ttam: String,
    #[serde(default)]
    pub etf_frcr_ntas_ttam: String,
    #[serde(default)]
    pub frgn_limt_rate: String,
    #[serde(default)]
    pub frgn_oder_able_qty: String,
    #[serde(default)]
    pub etf_cu_unit_scrt_cnt: String,
    #[serde(default)]
    pub etf_cnfg_issu_cnt: String,
    #[serde(default)]
    pub etf_dvdn_cycl: String,
    #[serde(default)]
    pub crcd: String,
    #[serde(default)]
    pub etf_crcl_ntas_ttam: String,
    #[serde(default)]
    pub etf_frcr_crcl_ntas_ttam: String,
    #[serde(default)]
    pub etf_frcr_last_ntas_wrth_val: String,
    #[serde(default)]
    pub lp_oder_able_cls_code: String,
    #[serde(default)]
    pub stck_dryy_hgpr: String,
    #[serde(default)]
    pub dryy_hgpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub dryy_hgpr_date: String,
    #[serde(default)]
    pub stck_dryy_lwpr: String,
    #[serde(default)]
    pub dryy_lwpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub dryy_lwpr_date: String,
    #[serde(default)]
    pub bstp_kor_isnm: String,
    #[serde(default)]
    pub vi_cls_code: String,
    #[serde(default)]
    pub lstn_stcn: String,
    #[serde(default)]
    pub frgn_hldn_qty: String,
    #[serde(default)]
    pub frgn_hldn_qty_rate: String,
    #[serde(default)]
    pub etf_trc_ert_mltp: String,
    #[serde(default)]
    pub dprt: String,
    #[serde(default)]
    pub mbcr_name: String,
    #[serde(default)]
    pub stck_lstn_date: String,
    #[serde(default)]
    pub mtrt_date: String,
    #[serde(default)]
    pub shrg_type_code: String,
    #[serde(default)]
    pub lp_hldn_rate: String,
    #[serde(default)]
    pub etf_trgt_nmix_bstp_code: String,
    #[serde(default)]
    pub etf_div_name: String,
    #[serde(default)]
    pub etf_rprs_bstp_kor_isnm: String,
    #[serde(default)]
    pub lp_hldn_vol: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("ETF/ETN 현재가는 모의투자 미지원 API입니다");
    }
    let params = [
        ("fid_input_iscd", req.fid_input_iscd.as_str()),
        ("fid_cond_mrkt_div_code", req.fid_cond_mrkt_div_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
