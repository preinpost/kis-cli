//! 국내주식 체결금액별 매매비중 — GET /uapi/domestic-stock/v1/quotations/tradprt-byamt

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/tradprt-byamt";
pub const TR_ID: &str = "FHKST111900C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_scr_div_code: String,
    pub fid_input_iscd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub prpr_name: String,
    #[serde(default)]
    pub smtn_avrg_prpr: String,
    #[serde(default)]
    pub acml_vol: String,
    #[serde(default)]
    pub whol_ntby_qty_rate: String,
    #[serde(default)]
    pub ntby_cntg_csnu: String,
    #[serde(default)]
    pub seln_cnqn_smtn: String,
    #[serde(default)]
    pub whol_seln_vol_rate: String,
    #[serde(default)]
    pub seln_cntg_csnu: String,
    #[serde(default)]
    pub shnu_cnqn_smtn: String,
    #[serde(default)]
    pub whol_shun_vol_rate: String,
    #[serde(default)]
    pub shnu_cntg_csnu: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_INPUT_ISCD", req.fid_input_iscd.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
