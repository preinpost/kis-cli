//! 프로그램매매 종합현황(일별) — GET /uapi/domestic-stock/v1/quotations/comp-program-trade-daily

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/comp-program-trade-daily";
pub const TR_ID: &str = "FHPPG04600001";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_mrkt_cls_code: String,
    pub fid_input_date_1: String,
    pub fid_input_date_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_bsop_date: String,
    #[serde(default)]
    pub nabt_entm_seln_tr_pbmn: String,
    #[serde(default)]
    pub nabt_onsl_seln_vol: String,
    #[serde(default)]
    pub whol_onsl_seln_tr_pbmn: String,
    #[serde(default)]
    pub arbt_smtn_shnu_vol: String,
    #[serde(default)]
    pub nabt_smtn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub arbt_entm_ntby_qty: String,
    #[serde(default)]
    pub nabt_entm_ntby_tr_pbmn: String,
    #[serde(default)]
    pub arbt_entm_seln_vol: String,
    #[serde(default)]
    pub nabt_entm_seln_vol_rate: String,
    #[serde(default)]
    pub nabt_onsl_seln_vol_rate: String,
    #[serde(default)]
    pub whol_onsl_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_smtm_shun_vol_rate: String,
    #[serde(default)]
    pub nabt_smtm_shun_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_entm_ntby_qty_rate: String,
    #[serde(default)]
    pub nabt_entm_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_entm_seln_vol_rate: String,
    #[serde(default)]
    pub nabt_entm_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_onsl_seln_tr_pbmn: String,
    #[serde(default)]
    pub whol_smtn_seln_vol: String,
    #[serde(default)]
    pub arbt_smtn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub whol_entm_shnu_vol: String,
    #[serde(default)]
    pub arbt_entm_ntby_tr_pbmn: String,
    #[serde(default)]
    pub nabt_onsl_ntby_qty: String,
    #[serde(default)]
    pub arbt_entm_seln_tr_pbmn: String,
    #[serde(default)]
    pub nabt_onsl_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub whol_seln_vol_rate: String,
    #[serde(default)]
    pub arbt_smtm_shun_tr_pbmn_rate: String,
    #[serde(default)]
    pub whol_entm_shnu_vol_rate: String,
    #[serde(default)]
    pub arbt_entm_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_onsl_ntby_qty_rate: String,
    #[serde(default)]
    pub arbt_entm_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_smtn_seln_vol: String,
    #[serde(default)]
    pub whol_smtn_seln_tr_pbmn: String,
    #[serde(default)]
    pub nabt_entm_shnu_vol: String,
    #[serde(default)]
    pub whol_entm_shnu_tr_pbmn: String,
    #[serde(default)]
    pub arbt_onsl_ntby_qty: String,
    #[serde(default)]
    pub nabt_onsl_ntby_tr_pbmn: String,
    #[serde(default)]
    pub arbt_onsl_seln_tr_pbmn: String,
    #[serde(default)]
    pub nabt_smtm_seln_vol_rate: String,
    #[serde(default)]
    pub whol_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_entm_shnu_vol_rate: String,
    #[serde(default)]
    pub whol_entm_shnu_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_onsl_ntby_qty_rate: String,
    #[serde(default)]
    pub nabt_onsl_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_onsl_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_smtn_seln_tr_pbmn: String,
    #[serde(default)]
    pub arbt_entm_shnu_vol: String,
    #[serde(default)]
    pub nabt_entm_shnu_tr_pbmn: String,
    #[serde(default)]
    pub whol_onsl_shnu_vol: String,
    #[serde(default)]
    pub arbt_onsl_ntby_tr_pbmn: String,
    #[serde(default)]
    pub nabt_smtn_ntby_qty: String,
    #[serde(default)]
    pub arbt_onsl_seln_vol: String,
    #[serde(default)]
    pub nabt_smtm_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_entm_shnu_vol_rate: String,
    #[serde(default)]
    pub nabt_entm_shnu_tr_pbmn_rate: String,
    #[serde(default)]
    pub whol_onsl_shnu_tr_pbmn: String,
    #[serde(default)]
    pub arbt_onsl_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_smtm_ntby_qty_rate: String,
    #[serde(default)]
    pub arbt_onsl_seln_vol_rate: String,
    #[serde(default)]
    pub whol_entm_seln_vol: String,
    #[serde(default)]
    pub arbt_entm_shnu_tr_pbmn: String,
    #[serde(default)]
    pub nabt_onsl_shnu_vol: String,
    #[serde(default)]
    pub whol_onsl_shnu_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_smtn_ntby_qty: String,
    #[serde(default)]
    pub nabt_smtn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub arbt_smtn_seln_vol: String,
    #[serde(default)]
    pub whol_entm_seln_tr_pbmn: String,
    #[serde(default)]
    pub arbt_entm_shnu_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_onsl_shnu_vol_rate: String,
    #[serde(default)]
    pub whol_onsl_shnu_vol_rate: String,
    #[serde(default)]
    pub arbt_smtm_ntby_qty_rate: String,
    #[serde(default)]
    pub nabt_smtm_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_smtm_seln_vol_rate: String,
    #[serde(default)]
    pub whol_entm_seln_vol_rate: String,
    #[serde(default)]
    pub arbt_onsl_shnu_vol: String,
    #[serde(default)]
    pub nabt_onsl_shnu_tr_pbmn: String,
    #[serde(default)]
    pub whol_smtn_shnu_vol: String,
    #[serde(default)]
    pub arbt_smtn_ntby_tr_pbmn: String,
    #[serde(default)]
    pub whol_entm_ntby_qty: String,
    #[serde(default)]
    pub arbt_smtn_seln_tr_pbmn: String,
    #[serde(default)]
    pub whol_entm_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub arbt_onsl_shnu_vol_rate: String,
    #[serde(default)]
    pub nabt_onsl_shnu_tr_pbmn_rate: String,
    #[serde(default)]
    pub whol_shun_vol_rate: String,
    #[serde(default)]
    pub arbt_smtm_ntby_tr_pbmn_rate: String,
    #[serde(default)]
    pub whol_entm_ntby_qty_rate: String,
    #[serde(default)]
    pub arbt_smtm_seln_tr_pbmn_rate: String,
    #[serde(default)]
    pub whol_onsl_seln_vol: String,
    #[serde(default)]
    pub arbt_onsl_shnu_tr_pbmn: String,
    #[serde(default)]
    pub nabt_smtn_shnu_vol: String,
    #[serde(default)]
    pub whol_smtn_shnu_tr_pbmn: String,
    #[serde(default)]
    pub nabt_entm_ntby_qty: String,
    #[serde(default)]
    pub whol_entm_ntby_tr_pbmn: String,
    #[serde(default)]
    pub nabt_entm_seln_vol: String,
    #[serde(default)]
    pub whol_onsl_seln_vol_rate: String,
    #[serde(default)]
    pub arbt_onsl_shnu_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_smtm_shun_vol_rate: String,
    #[serde(default)]
    pub whol_shun_tr_pbmn_rate: String,
    #[serde(default)]
    pub nabt_entm_ntby_qty_rate: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_INPUT_DATE_2", req.fid_input_date_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
