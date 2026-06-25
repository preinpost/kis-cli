//! 종목별 투자자매매동향(일별) — GET /uapi/domestic-stock/v1/quotations/investor-trade-by-stock-daily

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/investor-trade-by-stock-daily";
pub const TR_ID: &str = "FHPTJ04160001";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_date_1: String,
    pub fid_org_adj_prc: String,
    pub fid_etc_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub prdy_vol: String,
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub stck_clpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub prdy_ctrt: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
    #[serde(default)]
    pub stck_oprc: String,
    #[serde(default)]
    pub stck_hgpr: String,
    #[serde(default)]
    pub stck_lwpr: String,
    #[serde(default)]
    pub frgn_ntby_qty: String,
    #[serde(default)]
    pub frgn_reg_ntby_qty: String,
    #[serde(default)]
    pub frgn_nreg_ntby_qty: String,
    #[serde(default)]
    pub prsn_ntby_qty: String,
    #[serde(default)]
    pub orgn_ntby_qty: String,
    #[serde(default)]
    pub scrt_ntby_qty: String,
    #[serde(default)]
    pub ivtr_ntby_qty: String,
    #[serde(default)]
    pub pe_fund_ntby_vol: String,
    #[serde(default)]
    pub bank_ntby_qty: String,
    #[serde(default)]
    pub insu_ntby_qty: String,
    #[serde(default)]
    pub mrbn_ntby_qty: String,
    #[serde(default)]
    pub fund_ntby_qty: String,
    #[serde(default)]
    pub etc_ntby_qty: String,
    #[serde(default)]
    pub etc_corp_ntby_vol: String,
    #[serde(default)]
    pub etc_orgt_ntby_vol: String,
    #[serde(default)]
    pub frgn_reg_ntby_pbmn: String,
    #[serde(default)]
    pub frgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub frgn_nreg_ntby_pbmn: String,
    #[serde(default)]
    pub prsn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub orgn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub scrt_ntby_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_ntby_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_ntby_tr_pbmn: String,
    #[serde(default)]
    pub bank_ntby_tr_pbmn: String,
    #[serde(default)]
    pub insu_ntby_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub fund_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_ntby_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_ntby_tr_pbmn: String,
    #[serde(default)]
    pub frgn_seln_vol: String,
    #[serde(default)]
    pub frgn_shnu_vol: String,
    #[serde(default)]
    pub frgn_seln_tr_pbmn: String,
    #[serde(default)]
    pub frgn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub frgn_reg_askp_qty: String,
    #[serde(default)]
    pub frgn_reg_bidp_qty: String,
    #[serde(default)]
    pub frgn_reg_askp_pbmn: String,
    #[serde(default)]
    pub frgn_reg_bidp_pbmn: String,
    #[serde(default)]
    pub frgn_nreg_askp_qty: String,
    #[serde(default)]
    pub frgn_nreg_bidp_qty: String,
    #[serde(default)]
    pub frgn_nreg_askp_pbmn: String,
    #[serde(default)]
    pub frgn_nreg_bidp_pbmn: String,
    #[serde(default)]
    pub prsn_seln_vol: String,
    #[serde(default)]
    pub prsn_shnu_vol: String,
    #[serde(default)]
    pub prsn_seln_tr_pbmn: String,
    #[serde(default)]
    pub prsn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub orgn_seln_vol: String,
    #[serde(default)]
    pub orgn_shnu_vol: String,
    #[serde(default)]
    pub orgn_seln_tr_pbmn: String,
    #[serde(default)]
    pub orgn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub scrt_seln_vol: String,
    #[serde(default)]
    pub scrt_shnu_vol: String,
    #[serde(default)]
    pub scrt_seln_tr_pbmn: String,
    #[serde(default)]
    pub scrt_shnu_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_seln_vol: String,
    #[serde(default)]
    pub ivtr_shnu_vol: String,
    #[serde(default)]
    pub ivtr_seln_tr_pbmn: String,
    #[serde(default)]
    pub ivtr_shnu_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_seln_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_seln_vol: String,
    #[serde(default)]
    pub pe_fund_shnu_tr_pbmn: String,
    #[serde(default)]
    pub pe_fund_shnu_vol: String,
    #[serde(default)]
    pub bank_seln_vol: String,
    #[serde(default)]
    pub bank_shnu_vol: String,
    #[serde(default)]
    pub bank_seln_tr_pbmn: String,
    #[serde(default)]
    pub bank_shnu_tr_pbmn: String,
    #[serde(default)]
    pub insu_seln_vol: String,
    #[serde(default)]
    pub insu_shnu_vol: String,
    #[serde(default)]
    pub insu_seln_tr_pbmn: String,
    #[serde(default)]
    pub insu_shnu_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_seln_vol: String,
    #[serde(default)]
    pub mrbn_shnu_vol: String,
    #[serde(default)]
    pub mrbn_seln_tr_pbmn: String,
    #[serde(default)]
    pub mrbn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub fund_seln_vol: String,
    #[serde(default)]
    pub fund_shnu_vol: String,
    #[serde(default)]
    pub fund_seln_tr_pbmn: String,
    #[serde(default)]
    pub fund_shnu_tr_pbmn: String,
    #[serde(default)]
    pub etc_seln_vol: String,
    #[serde(default)]
    pub etc_shnu_vol: String,
    #[serde(default)]
    pub etc_seln_tr_pbmn: String,
    #[serde(default)]
    pub etc_shnu_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_seln_vol: String,
    #[serde(default)]
    pub etc_orgt_shnu_vol: String,
    #[serde(default)]
    pub etc_orgt_seln_tr_pbmn: String,
    #[serde(default)]
    pub etc_orgt_shnu_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_seln_vol: String,
    #[serde(default)]
    pub etc_corp_shnu_vol: String,
    #[serde(default)]
    pub etc_corp_seln_tr_pbmn: String,
    #[serde(default)]
    pub etc_corp_shnu_tr_pbmn: String,
    #[serde(default)]
    pub bold_yn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_ORG_ADJ_PRC", req.fid_org_adj_prc.as_str()),
        ("FID_ETC_CLS_CODE", req.fid_etc_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp.output1.and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let rows: Vec<Row> = resp.output2.map(serde_json::from_value).transpose()?.unwrap_or_default();
    Ok(Response { meta, rows })
}
