//! 관심종목 그룹별 종목조회 — GET /uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/quotations/intstock-stocklist-by-group";
pub const TR_ID: &str = "HHKCM113004C6";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub type_: String,
    pub user_id: String,
    pub data_rank: String,
    pub inter_grp_code: String,
    pub inter_grp_name: String,
    pub hts_kor_isnm: String,
    pub cntg_cls_code: String,
    pub fid_etc_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GroupMeta {
    #[serde(default)]
    pub data_rank: String,
    #[serde(default)]
    pub inter_grp_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stock {
    #[serde(default)]
    pub fid_mrkt_cls_code: String,
    #[serde(default)]
    pub data_rank: String,
    #[serde(default)]
    pub exch_code: String,
    #[serde(default)]
    pub jong_code: String,
    #[serde(default)]
    pub color_code: String,
    #[serde(default)]
    pub memo: String,
    #[serde(default)]
    pub hts_kor_isnm: String,
    #[serde(default)]
    pub fxdt_ntby_qty: String,
    #[serde(default)]
    pub cntg_unpr: String,
    #[serde(default)]
    pub cntg_cls_code: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub group_meta: Vec<GroupMeta>,
    pub stocks: Vec<Stock>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let params = [
        ("TYPE", req.type_.as_str()),
        ("USER_ID", req.user_id.as_str()),
        ("DATA_RANK", req.data_rank.as_str()),
        ("INTER_GRP_CODE", req.inter_grp_code.as_str()),
        ("INTER_GRP_NAME", req.inter_grp_name.as_str()),
        ("HTS_KOR_ISNM", req.hts_kor_isnm.as_str()),
        ("CNTG_CLS_CODE", req.cntg_cls_code.as_str()),
        ("FID_ETC_CLS_CODE", req.fid_etc_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let group_meta: Vec<GroupMeta> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    let stocks: Vec<Stock> = resp
        .output2
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { group_meta, stocks })
}
