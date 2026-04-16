//! 종목별 일별 대차거래추이 — GET /uapi/domestic-stock/v1/quotations/daily-loan-trans

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/daily-loan-trans";
pub const TR_ID: &str = "HHPST074500C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub mrkt_div_cls_code: String,
    pub mksc_shrn_iscd: String,
    pub start_date: String,
    pub end_date: String,
    pub cts: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bsop_date: String,
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
    pub new_stcn: String,
    #[serde(default)]
    pub rdmp_stcn: String,
    #[serde(default)]
    pub prdy_rmnd_vrss: String,
    #[serde(default)]
    pub rmnd_stcn: String,
    #[serde(default)]
    pub rmnd_amt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("MRKT_DIV_CLS_CODE", req.mrkt_div_cls_code.as_str()),
        ("MKSC_SHRN_ISCD", req.mksc_shrn_iscd.as_str()),
        ("START_DATE", req.start_date.as_str()),
        ("END_DATE", req.end_date.as_str()),
        ("CTS", req.cts.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
