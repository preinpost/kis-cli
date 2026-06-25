//! 국내옵션전광판_옵션월물리스트 — GET /uapi/domestic-futureoption/v1/quotations/display-board-option-list
//!
//! 스펙: .agent/specs/futureoption_domestic__quotations__display_board_option_list.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/quotations/display-board-option-list";
pub const TR_ID: &str = "FHPIO056104C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fid_cond_scr_div_code: String,
    pub fid_cond_mrkt_div_code: String,
    pub fid_cond_mrkt_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub mtrt_yymm_code: String,
    #[serde(default)]
    pub mtrt_yymm: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("국내옵션전광판_옵션월물리스트는 모의투자 미지원");
    }
    let params = [
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_MRKT_CLS_CODE", req.fid_cond_mrkt_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output1
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(rows)
}
