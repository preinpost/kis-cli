//! 해외선물옵션 장운영시간 — GET /uapi/overseas-futureoption/v1/quotations/market-time
//!
//! 스펙: .agent/specs/futureoption_overseas__quotations__market_time.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/quotations/market-time";
pub const TR_ID: &str = "OTFM2229R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub fm_pdgr_cd: String,
    pub fm_clas_cd: String,
    pub fm_excg_cd: String,
    pub opt_yn: String,
    pub ctx_area_nk200: String,
    pub ctx_area_fk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub fm_pdgr_cd: String,
    #[serde(default)]
    pub fm_pdgr_name: String,
    #[serde(default)]
    pub fm_excg_cd: String,
    #[serde(default)]
    pub fm_excg_name: String,
    #[serde(default)]
    pub fuop_dvsn_name: String,
    #[serde(default)]
    pub fm_clas_cd: String,
    #[serde(default)]
    pub fm_clas_name: String,
    #[serde(default)]
    pub am_mkmn_strt_tmd: String,
    #[serde(default)]
    pub am_mkmn_end_tmd: String,
    #[serde(default)]
    pub pm_mkmn_strt_tmd: String,
    #[serde(default)]
    pub pm_mkmn_end_tmd: String,
    #[serde(default)]
    pub mkmn_nxdy_strt_tmd: String,
    #[serde(default)]
    pub mkmn_nxdy_end_tmd: String,
    #[serde(default)]
    pub base_mket_strt_tmd: String,
    #[serde(default)]
    pub base_mket_end_tmd: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 장운영시간은 모의투자 미지원 API입니다");
    }
    let params = [
        ("FM_PDGR_CD", req.fm_pdgr_cd.as_str()),
        ("FM_CLAS_CD", req.fm_clas_cd.as_str()),
        ("FM_EXCG_CD", req.fm_excg_cd.as_str()),
        ("OPT_YN", req.opt_yn.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
