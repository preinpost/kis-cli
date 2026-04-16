//! 기간별계좌권리현황조회 — GET /uapi/domestic-stock/v1/trading/period-rights
//!
//! 스펙: .agent/specs/domestic_stock__order_account__period_rights.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/period-rights";
pub const TR_ID: &str = "CTRGA011R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// 03 입력
    pub inqr_dvsn: String,
    pub cust_rncno25: String,
    pub hmid: String,
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    pub rght_type_cd: String,
    pub pdno: String,
    pub prdt_type_cd: String,
    pub ctx_area_nk100: String,
    pub ctx_area_fk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub acno10: String,
    #[serde(default)]
    pub rght_type_cd: String,
    #[serde(default)]
    pub bass_dt: String,
    #[serde(default)]
    pub rght_cblc_type_cd: String,
    #[serde(default)]
    pub rptt_pdno: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub shtn_pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub cblc_qty: String,
    #[serde(default)]
    pub last_alct_qty: String,
    #[serde(default)]
    pub excs_alct_qty: String,
    #[serde(default)]
    pub tot_alct_qty: String,
    #[serde(default)]
    pub last_ftsk_qty: String,
    #[serde(default)]
    pub last_alct_amt: String,
    #[serde(default)]
    pub last_ftsk_chgs: String,
    #[serde(default)]
    pub rdpt_prca: String,
    #[serde(default)]
    pub dlay_int_amt: String,
    #[serde(default)]
    pub lstg_dt: String,
    #[serde(default)]
    pub sbsc_end_dt: String,
    #[serde(default)]
    pub cash_dfrm_dt: String,
    #[serde(default)]
    pub rqst_qty: String,
    #[serde(default)]
    pub rqst_amt: String,
    #[serde(default)]
    pub rqst_dt: String,
    #[serde(default)]
    pub rfnd_dt: String,
    #[serde(default)]
    pub rfnd_amt: String,
    #[serde(default)]
    pub lstg_stqt: String,
    #[serde(default)]
    pub tax_amt: String,
    #[serde(default)]
    pub sbsc_unpr: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("기간별계좌권리현황조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("INQR_DVSN", req.inqr_dvsn.as_str()),
        ("CUST_RNCNO25", req.cust_rncno25.as_str()),
        ("HMID", req.hmid.as_str()),
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("RGHT_TYPE_CD", req.rght_type_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("PRDT_TYPE_CD", req.prdt_type_cd.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp
        .output1
        .ok_or_else(|| anyhow!("응답에 output1 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
