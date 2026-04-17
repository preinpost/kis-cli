//! 해외선물옵션 기간계좌거래내역 — GET /uapi/overseas-futureoption/v1/trading/inquire-period-trans
//!
//! 스펙: .agent/specs/futureoption_overseas__order_account__inquire_period_trans.md
//! 모의투자 미지원.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/overseas-futureoption/v1/trading/inquire-period-trans";
pub const TR_ID: &str = "OTFM3114R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub inqr_term_from_dt: String,
    pub inqr_term_to_dt: String,
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub acnt_tr_type_cd: String,
    pub crcy_cd: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
    pub pwd_chk_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub bass_dt: String,
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub fm_ldgr_inog_seq: String,
    #[serde(default)]
    pub acnt_tr_type_name: String,
    #[serde(default)]
    pub crcy_cd: String,
    #[serde(default)]
    pub tr_itm_name: String,
    #[serde(default)]
    pub fm_iofw_amt: String,
    #[serde(default)]
    pub fm_fee: String,
    #[serde(default)]
    pub fm_tax_amt: String,
    #[serde(default)]
    pub fm_sttl_amt: String,
    #[serde(default)]
    pub fm_bf_dncl_amt: String,
    #[serde(default)]
    pub fm_dncl_amt: String,
    #[serde(default)]
    pub fm_rcvb_occr_amt: String,
    #[serde(default)]
    pub fm_rcvb_pybk_amt: String,
    #[serde(default)]
    pub ovdu_int_pybk_amt: String,
    #[serde(default)]
    pub rmks_text: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("해외선물옵션 기간계좌거래내역은 모의투자 미지원 API입니다");
    }
    let params = [
        ("INQR_TERM_FROM_DT", req.inqr_term_from_dt.as_str()),
        ("INQR_TERM_TO_DT", req.inqr_term_to_dt.as_str()),
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("ACNT_TR_TYPE_CD", req.acnt_tr_type_cd.as_str()),
        ("CRCY_CD", req.crcy_cd.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
        ("PWD_CHK_YN", req.pwd_chk_yn.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let rows = resp
        .output
        .map(serde_json::from_value::<Vec<Row>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { rows })
}
