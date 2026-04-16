//! 주식일별주문체결조회 — GET /uapi/domestic-stock/v1/trading/inquire-daily-ccld
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_daily_ccld.md
//!
//! 3개월이내/이전 × 실전/모의 = TR_ID 4종. `Period` enum으로 분기.
//! Response는 output1 (Vec<Row>) + output2 (Summary, single).

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-daily-ccld";
pub const TR_ID_REAL_WITHIN3M: &str = "TTTC0081R";
pub const TR_ID_REAL_OVER3M: &str = "CTSC9215R";
pub const TR_ID_MOCK_WITHIN3M: &str = "VTTC0081R";
pub const TR_ID_MOCK_OVER3M: &str = "VTSC9215R";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Period {
    Within3Months,
    Over3Months,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    /// YYYYMMDD
    pub inqr_strt_dt: String,
    pub inqr_end_dt: String,
    /// 00 전체, 01 매도, 02 매수
    pub sll_buy_dvsn_cd: String,
    pub pdno: String,
    pub ord_gno_brno: String,
    pub odno: String,
    /// 00 전체, 01 체결, 02 미체결
    pub ccld_dvsn: String,
    /// 00 역순, 01 정순
    pub inqr_dvsn: String,
    /// 없음 전체, 1 ELW, 2 프리보드
    pub inqr_dvsn_1: String,
    /// 00 전체, 01~07 현금/신용 등
    pub inqr_dvsn_3: String,
    /// KRX/NXT/SOR/ALL
    pub excg_id_dvsn_cd: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub ord_gno_brno: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub ord_dvsn_name: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub ord_qty: String,
    #[serde(default)]
    pub ord_unpr: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub avg_prvs: String,
    #[serde(default)]
    pub cncl_yn: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub ordr_empno: String,
    #[serde(default)]
    pub ord_dvsn_cd: String,
    #[serde(default)]
    pub cnc_cfrm_qty: String,
    #[serde(default)]
    pub rmn_qty: String,
    #[serde(default)]
    pub rjct_qty: String,
    #[serde(default)]
    pub ccld_cndt_name: String,
    #[serde(default)]
    pub inqr_ip_addr: String,
    #[serde(default)]
    pub cpbc_ordp_ord_rcit_dvsn_cd: String,
    #[serde(default)]
    pub cpbc_ordp_infm_mthd_dvsn_cd: String,
    #[serde(default)]
    pub infm_tmd: String,
    #[serde(default)]
    pub ctac_tlno: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub excg_dvsn_cd: String,
    #[serde(default)]
    pub cpbc_ordp_mtrl_dvsn_cd: String,
    #[serde(default)]
    pub ord_orgno: String,
    #[serde(default)]
    pub rsvn_ord_end_dt: String,
    #[serde(default, rename = "excg_id_dvsn_Cd")]
    pub excg_id_dvsn_cd: String,
    #[serde(default)]
    pub stpm_cndt_pric: String,
    #[serde(default)]
    pub stpm_efct_occr_dtmd: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub tot_ord_qty: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub prsm_tlex_smtl: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub rows: Vec<Row>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, period: Period, req: &Request) -> Result<Response> {
    let tr_id = match (client.is_mock(), period) {
        (false, Period::Within3Months) => TR_ID_REAL_WITHIN3M,
        (false, Period::Over3Months) => TR_ID_REAL_OVER3M,
        (true, Period::Within3Months) => TR_ID_MOCK_WITHIN3M,
        (true, Period::Over3Months) => TR_ID_MOCK_OVER3M,
    };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("INQR_STRT_DT", req.inqr_strt_dt.as_str()),
        ("INQR_END_DT", req.inqr_end_dt.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("ORD_GNO_BRNO", req.ord_gno_brno.as_str()),
        ("ODNO", req.odno.as_str()),
        ("CCLD_DVSN", req.ccld_dvsn.as_str()),
        ("INQR_DVSN", req.inqr_dvsn.as_str()),
        ("INQR_DVSN_1", req.inqr_dvsn_1.as_str()),
        ("INQR_DVSN_3", req.inqr_dvsn_3.as_str()),
        ("EXCG_ID_DVSN_CD", req.excg_id_dvsn_cd.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, tr_id, &params).await?;
    let rows: Vec<Row> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { rows, summary })
}
