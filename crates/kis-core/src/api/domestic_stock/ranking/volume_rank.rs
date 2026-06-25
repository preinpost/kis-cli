//! 거래량순위 — GET /uapi/domestic-stock/v1/quotations/volume-rank
//!
//! 스펙: .agent/specs/domestic_stock__ranking__volume_rank.md
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/volume-rank";
pub const TR_ID: &str = "FHPST01710000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
    pub fid_div_cls_code: String,
    pub fid_blng_cls_code: String,
    pub fid_trgt_cls_code: String,
    pub fid_trgt_exls_cls_code: String,
    pub fid_input_price_1: String,
    pub fid_input_price_2: String,
    pub fid_vol_cnt: String,
    pub fid_input_date_1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub mksc_shrn_iscd: String,
    #[serde(default)]
    pub data_rank: String,
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
    pub lstn_stcn: String,
    #[serde(default)]
    pub avrg_vol: String,
    #[serde(default)]
    pub n_befr_clpr_vrss_prpr_rate: String,
    #[serde(default)]
    pub vol_inrt: String,
    #[serde(default)]
    pub vol_tnrt: String,
    #[serde(default)]
    pub nday_vol_tnrt: String,
    #[serde(default)]
    pub avrg_tr_pbmn: String,
    #[serde(default)]
    pub tr_pbmn_tnrt: String,
    #[serde(default)]
    pub nday_tr_pbmn_tnrt: String,
    #[serde(default)]
    pub acml_tr_pbmn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("거래량순위는 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_BLNG_CLS_CODE", req.fid_blng_cls_code.as_str()),
        ("FID_TRGT_CLS_CODE", req.fid_trgt_cls_code.as_str()),
        ("FID_TRGT_EXLS_CLS_CODE", req.fid_trgt_exls_cls_code.as_str()),
        ("FID_INPUT_PRICE_1", req.fid_input_price_1.as_str()),
        ("FID_INPUT_PRICE_2", req.fid_input_price_2.as_str()),
        ("FID_VOL_CNT", req.fid_vol_cnt.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
