//! 주식현재가 시세 — GET /uapi/domestic-stock/v1/quotations/inquire-price
//!
//! 스펙: .agent/specs/domestic_stock__quotations__inquire_price.md

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/inquire-price";
pub const TR_ID: &str = "FHKST01010100";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// J:KRX, NX:NXT, UN:통합
    pub fid_cond_mrkt_div_code: String,
    /// 종목코드. ETN은 앞에 Q
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub iscd_stat_cls_code: String,
    #[serde(default)]
    pub marg_rate: String,
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
    #[serde(default)]
    pub new_hgpr_lwpr_cls_code: String,
    #[serde(default)]
    pub bstp_kor_isnm: String,
    #[serde(default)]
    pub temp_stop_yn: String,
    #[serde(default)]
    pub oprc_rang_cont_yn: String,
    #[serde(default)]
    pub clpr_rang_cont_yn: String,
    #[serde(default)]
    pub crdt_able_yn: String,
    #[serde(default)]
    pub grmn_rate_cls_code: String,
    #[serde(default)]
    pub elw_pblc_yn: String,
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
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub stck_mxpr: String,
    #[serde(default)]
    pub stck_llam: String,
    #[serde(default)]
    pub stck_sdpr: String,
    #[serde(default)]
    pub wghn_avrg_stck_prc: String,
    #[serde(default)]
    pub hts_frgn_ehrt: String,
    #[serde(default)]
    pub frgn_ntby_qty: String,
    #[serde(default)]
    pub pgtr_ntby_qty: String,
    #[serde(default)]
    pub pvt_scnd_dmrs_prc: String,
    #[serde(default)]
    pub pvt_frst_dmrs_prc: String,
    #[serde(default)]
    pub pvt_pont_val: String,
    #[serde(default)]
    pub pvt_frst_dmsp_prc: String,
    #[serde(default)]
    pub pvt_scnd_dmsp_prc: String,
    #[serde(default)]
    pub dmrs_val: String,
    #[serde(default)]
    pub dmsp_val: String,
    #[serde(default)]
    pub cpfn: String,
    #[serde(default)]
    pub rstc_wdth_prc: String,
    #[serde(default)]
    pub stck_fcam: String,
    #[serde(default)]
    pub stck_sspr: String,
    #[serde(default)]
    pub aspr_unit: String,
    #[serde(default)]
    pub hts_deal_qty_unit_val: String,
    #[serde(default)]
    pub lstn_stcn: String,
    #[serde(default)]
    pub hts_avls: String,
    #[serde(default)]
    pub per: String,
    #[serde(default)]
    pub pbr: String,
    #[serde(default)]
    pub stac_month: String,
    #[serde(default)]
    pub vol_tnrt: String,
    #[serde(default)]
    pub eps: String,
    #[serde(default)]
    pub bps: String,
    #[serde(default)]
    pub d250_hgpr: String,
    #[serde(default)]
    pub d250_hgpr_date: String,
    #[serde(default)]
    pub d250_hgpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub d250_lwpr: String,
    #[serde(default)]
    pub d250_lwpr_date: String,
    #[serde(default)]
    pub d250_lwpr_vrss_prpr_rate: String,
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
    pub w52_hgpr: String,
    #[serde(default)]
    pub w52_hgpr_vrss_prpr_ctrt: String,
    #[serde(default)]
    pub w52_hgpr_date: String,
    #[serde(default)]
    pub w52_lwpr: String,
    #[serde(default)]
    pub w52_lwpr_vrss_prpr_ctrt: String,
    #[serde(default)]
    pub w52_lwpr_date: String,
    #[serde(default)]
    pub whol_loan_rmnd_rate: String,
    #[serde(default)]
    pub ssts_yn: String,
    #[serde(default)]
    pub stck_shrn_iscd: String,
    #[serde(default)]
    pub fcam_cnnm: String,
    #[serde(default)]
    pub cpfn_cnnm: String,
    #[serde(default)]
    pub apprch_rate: String,
    #[serde(default)]
    pub frgn_hldn_qty: String,
    #[serde(default)]
    pub vi_cls_code: String,
    #[serde(default)]
    pub ovtm_vi_cls_code: String,
    #[serde(default)]
    pub last_ssts_cntg_qty: String,
    #[serde(default)]
    pub invt_caful_yn: String,
    #[serde(default)]
    pub mrkt_warn_cls_code: String,
    #[serde(default)]
    pub short_over_yn: String,
    #[serde(default)]
    pub sltr_yn: String,
    #[serde(default)]
    pub mang_issu_cls_code: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
