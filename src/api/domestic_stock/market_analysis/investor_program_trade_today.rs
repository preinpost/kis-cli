//! 프로그램매매 투자자매매동향(당일) — GET /uapi/domestic-stock/v1/quotations/investor-program-trade-today

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/investor-program-trade-today";
pub const TR_ID: &str = "HHPPG046600C1";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub exch_div_cls_code: String,
    pub mrkt_div_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub invr_cls_code: String,
    #[serde(default)]
    pub all_seln_qty: String,
    #[serde(default)]
    pub all_seln_amt: String,
    #[serde(default)]
    pub invr_cls_name: String,
    #[serde(default)]
    pub all_shnu_qty: String,
    #[serde(default)]
    pub all_shnu_amt: String,
    #[serde(default)]
    pub all_ntby_amt: String,
    #[serde(default)]
    pub arbt_seln_qty: String,
    #[serde(default)]
    pub all_ntby_qty: String,
    #[serde(default)]
    pub arbt_shnu_qty: String,
    #[serde(default)]
    pub arbt_ntby_qty: String,
    #[serde(default)]
    pub arbt_seln_amt: String,
    #[serde(default)]
    pub arbt_shnu_amt: String,
    #[serde(default)]
    pub arbt_ntby_amt: String,
    #[serde(default)]
    pub nabt_seln_qty: String,
    #[serde(default)]
    pub nabt_shnu_qty: String,
    #[serde(default)]
    pub nabt_ntby_qty: String,
    #[serde(default)]
    pub nabt_seln_amt: String,
    #[serde(default)]
    pub nabt_shnu_amt: String,
    #[serde(default)]
    pub nabt_ntby_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("EXCH_DIV_CLS_CODE", req.exch_div_cls_code.as_str()),
        ("MRKT_DIV_CLS_CODE", req.mrkt_div_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
