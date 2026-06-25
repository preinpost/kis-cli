//! 관심종목 그룹조회 — GET /uapi/domestic-stock/v1/quotations/intstock-grouplist

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/intstock-grouplist";
pub const TR_ID: &str = "HHKCM113004C7";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 1 관심종목, 2 구분그룹
    pub type_: String,
    pub fid_etc_cls_code: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub trnm_hour: String,
    #[serde(default)]
    pub data_rank: String,
    #[serde(default)]
    pub inter_grp_code: String,
    #[serde(default)]
    pub inter_grp_name: String,
    #[serde(default)]
    pub ask_cnt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    let params = [
        ("TYPE", req.type_.as_str()),
        ("FID_ETC_CLS_CODE", req.fid_etc_cls_code.as_str()),
        ("USER_ID", req.user_id.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output2.ok_or_else(|| anyhow!("응답에 output2 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
