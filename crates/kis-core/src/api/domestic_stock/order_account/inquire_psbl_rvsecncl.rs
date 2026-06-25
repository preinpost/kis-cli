//! 주식정정취소가능주문조회 — GET /uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_psbl_rvsecncl.md
//!
//! 모의투자 미지원. 연속조회(tr_cont / CTX_AREA_FK100/NK100)는 호출자가 tr_cont 헤더와
//! output의 ctx 토큰을 직접 관리.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl";
pub const TR_ID: &str = "TTTC0084R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    /// 연속조회검색조건 (최초 호출 시 빈 문자열)
    pub ctx_area_fk100: String,
    /// 연속조회키 (최초 호출 시 빈 문자열)
    pub ctx_area_nk100: String,
    /// 조회구분1 — 0=주문, 1=종목
    pub inqr_dvsn_1: String,
    /// 조회구분2 — 0=전체, 1=매도, 2=매수
    pub inqr_dvsn_2: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub ord_gno_brno: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub ord_dvsn_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub rvse_cncl_dvsn_name: String,
    #[serde(default)]
    pub ord_qty: String,
    #[serde(default)]
    pub ord_unpr: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub psbl_qty: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub ord_dvsn_cd: String,
    #[serde(default)]
    pub mgco_aptm_odno: String,
    #[serde(default)]
    pub excg_dvsn_cd: String,
    #[serde(default)]
    pub excg_id_dvsn_cd: String,
    #[serde(default)]
    pub excg_id_dvsn_name: String,
    #[serde(default)]
    pub stpm_cndt_pric: String,
    #[serde(default)]
    pub stpm_efct_occr_yn: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("주식정정취소가능주문조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
        ("INQR_DVSN_1", req.inqr_dvsn_1.as_str()),
        ("INQR_DVSN_2", req.inqr_dvsn_2.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
