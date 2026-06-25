//! 종목별 외국계 매매동향 — GET /uapi/domestic-stock/v1/quotations/frgnmem-trade-trend

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/frgnmem-trade-trend";
pub const TR_ID: &str = "FHPST04320000";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_scr_div_code: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_input_iscd: String,
    pub fid_input_iscd_2: String,
    pub fid_mrkt_cls_code: String,
    pub fid_vol_cnt: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub total_seln_qty: String,
    #[serde(default)]
    pub total_shnu_qty: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bsop_hour: String,
    #[serde(default)]
    pub mbcr_name: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub stck_prpr: String,
    #[serde(default)]
    pub prdy_vrss: String,
    #[serde(default)]
    pub prdy_vrss_sign: String,
    #[serde(default)]
    pub cntg_vol: String,
    #[serde(default)]
    pub acml_ntby_qty: String,
    #[serde(default)]
    pub glob_ntby_qty: String,
    #[serde(default)]
    pub frgn_ntby_qty_icdc: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub meta: Option<Meta>,
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
        ("FID_INPUT_ISCD_2", req.fid_input_iscd_2.as_str()),
        ("FID_MRKT_CLS_CODE", req.fid_mrkt_cls_code.as_str()),
        ("FID_VOL_CNT", req.fid_vol_cnt.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let meta = resp
        .output1
        .and_then(|v| serde_json::from_value::<Meta>(v).ok());
    let rows: Vec<Row> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { meta, rows })
}
