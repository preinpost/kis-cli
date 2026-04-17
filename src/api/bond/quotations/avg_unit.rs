//! 장내채권 평균단가조회 — GET /uapi/domestic-bond/v1/quotations/avg-unit
//!
//! 스펙: .agent/specs/bond__quotations__avg_unit.md
//! 모의투자 미지원. output1(평가단가·수익률), output2(평가금액), output3(통화별 평가가격) 3종.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-bond/v1/quotations/avg-unit";
pub const TR_ID: &str = "CTPF2005R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub pdno: String,
    pub prdt_type_cd: String,
    pub vrfc_kind_cd: String,
    pub ctx_area_nk30: String,
    pub ctx_area_fk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EvalUnit {
    #[serde(default)]
    pub evlu_dt: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub kis_unpr: String,
    #[serde(default)]
    pub kbp_unpr: String,
    #[serde(default)]
    pub nice_evlu_unpr: String,
    #[serde(default)]
    pub fnp_unpr: String,
    #[serde(default)]
    pub avg_evlu_unpr: String,
    #[serde(default)]
    pub kis_crdt_grad_text: String,
    #[serde(default)]
    pub kbp_crdt_grad_text: String,
    #[serde(default)]
    pub nice_crdt_grad_text: String,
    #[serde(default)]
    pub fnp_crdt_grad_text: String,
    #[serde(default)]
    pub chng_yn: String,
    #[serde(default)]
    pub kis_erng_rt: String,
    #[serde(default)]
    pub kbp_erng_rt: String,
    #[serde(default)]
    pub nice_evlu_erng_rt: String,
    #[serde(default)]
    pub fnp_erng_rt: String,
    #[serde(default)]
    pub avg_evlu_erng_rt: String,
    #[serde(default)]
    pub kis_rf_unpr: String,
    #[serde(default)]
    pub kbp_rf_unpr: String,
    #[serde(default)]
    pub nice_evlu_rf_unpr: String,
    #[serde(default)]
    pub avg_evlu_rf_unpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EvalAmount {
    #[serde(default)]
    pub evlu_dt: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub kis_evlu_amt: String,
    #[serde(default)]
    pub kbp_evlu_amt: String,
    #[serde(default)]
    pub nice_evlu_amt: String,
    #[serde(default)]
    pub fnp_evlu_amt: String,
    #[serde(default)]
    pub avg_evlu_amt: String,
    #[serde(default)]
    pub chng_yn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EvalCurrency {
    #[serde(default)]
    pub evlu_dt: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub kis_crcy_cd: String,
    #[serde(default)]
    pub kis_evlu_unit_pric: String,
    #[serde(default)]
    pub kis_evlu_pric: String,
    #[serde(default)]
    pub kbp_crcy_cd: String,
    #[serde(default)]
    pub kbp_evlu_unit_pric: String,
    #[serde(default)]
    pub kbp_evlu_pric: String,
    #[serde(default)]
    pub nice_crcy_cd: String,
    #[serde(default)]
    pub nice_evlu_unit_pric: String,
    #[serde(default)]
    pub nice_evlu_pric: String,
    #[serde(default)]
    pub avg_evlu_unit_pric: String,
    #[serde(default)]
    pub avg_evlu_pric: String,
    #[serde(default)]
    pub chng_yn: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub eval_units: Vec<EvalUnit>,
    pub eval_amounts: Vec<EvalAmount>,
    pub eval_currencies: Vec<EvalCurrency>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("장내채권 평균단가조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("VRFC_KIND_CD", req.vrfc_kind_cd.as_str()),
        ("CTX_AREA_NK30", req.ctx_area_nk30.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let eval_units = resp
        .output1
        .map(serde_json::from_value::<Vec<EvalUnit>>)
        .transpose()?
        .unwrap_or_default();
    let eval_amounts = resp
        .output2
        .map(serde_json::from_value::<Vec<EvalAmount>>)
        .transpose()?
        .unwrap_or_default();
    let eval_currencies = resp
        .output3
        .map(serde_json::from_value::<Vec<EvalCurrency>>)
        .transpose()?
        .unwrap_or_default();
    Ok(Response { eval_units, eval_amounts, eval_currencies })
}
