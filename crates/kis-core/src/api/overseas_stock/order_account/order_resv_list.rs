//! 해외주식 예약주문조회 — GET /uapi/overseas-stock/v1/trading/order-resv-list
//!
//! 스펙: .agent/specs/overseas_stock__order_account__order_resv_list.md
//! 모의투자 미지원. 미국(TTTT3039R) / 아시아(TTTS3014R) 분기.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;
use crate::models::ApiResponse;

pub const ENDPOINT: &str = "/uapi/overseas-stock/v1/trading/order-resv-list";
pub const TR_ID_USA: &str = "TTTT3039R";
pub const TR_ID_ASIA: &str = "TTTS3014R";

#[derive(Debug, Clone, Copy)]
pub enum Region {
    Usa,
    Asia,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub inqr_dvsn_cd: String,
    pub prdt_type_cd: String,
    pub ovrs_excg_cd: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub cncl_yn: String,
    #[serde(default)]
    pub rsvn_ord_rcit_dt: String,
    #[serde(default)]
    pub ovrs_rsvn_odno: String,
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub ord_gno_brno: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd_name: String,
    #[serde(default)]
    pub ovrs_rsvn_ord_stat_cd: String,
    #[serde(default)]
    pub ovrs_rsvn_ord_stat_cd_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub ord_rcit_tmd: String,
    #[serde(default)]
    pub ord_fwdg_tmd: String,
    #[serde(default)]
    pub tr_dvsn_name: String,
    #[serde(default)]
    pub ovrs_excg_cd: String,
    #[serde(default)]
    pub tr_mket_name: String,
    #[serde(default)]
    pub ord_stfno: String,
    #[serde(default)]
    pub ft_ord_qty: String,
    #[serde(default)]
    pub ft_ord_unpr3: String,
    #[serde(default)]
    pub ft_ccld_qty: String,
    #[serde(default)]
    pub nprc_rson_text: String,
    #[serde(default)]
    pub splt_buy_attr_name: String,
}

pub async fn call(client: &KisClient, region: Region, req: &Request) -> Result<Option<Response>> {
    if client.is_mock() {
        bail!("해외주식 예약주문조회는 모의투자 미지원");
    }
    let tr = match region {
        Region::Usa => TR_ID_USA,
        Region::Asia => TR_ID_ASIA,
    };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("INQR_DVSN_CD", req.inqr_dvsn_cd.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("OVRS_EXCG_CD", req.ovrs_excg_cd.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp: ApiResponse = client.get(ENDPOINT, tr, &params).await?;
    Ok(resp
        .output
        .and_then(|v| serde_json::from_value::<Response>(v).ok()))
}
