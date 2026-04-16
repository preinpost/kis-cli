//! 퇴직연금 미체결내역 — GET /uapi/domestic-stock/v1/trading/pension/inquire-daily-ccld
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_daily_ccld_2.md
//!
//! 모의투자 미지원. TR_ID 2종 (KRX전용 / NXT·SOR 포함).

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/pension/inquire-daily-ccld";
pub const TR_ID_KRX: &str = "TTTC2201R";
pub const TR_ID_NXT_SOR: &str = "TTTC2210R";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Market {
    KrxOnly,
    KrxNxtSor,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    /// 29
    pub acnt_prdt_cd: String,
    pub user_dvsn_cd: String,
    /// 00 전체, 01 매도, 02 매수
    pub sll_buy_dvsn_cd: String,
    /// %% 전체, 01 체결, 02 미체결
    pub ccld_nccs_dvsn: String,
    /// 00 전체
    pub inqr_dvsn_3: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub ord_gno_brno: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub trad_dvsn_name: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub ord_unpr: String,
    #[serde(default)]
    pub ord_qty: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub nccs_qty: String,
    #[serde(default)]
    pub ord_dvsn_cd: String,
    #[serde(default)]
    pub ord_dvsn_name: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub objt_cust_dvsn_name: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
    #[serde(default)]
    pub stpm_cndt_pric: String,
    #[serde(default)]
    pub stpm_efct_occr_dtmd: String,
    #[serde(default)]
    pub stpm_efct_occr_yn: String,
    #[serde(default)]
    pub excg_id_dvsn_cd: String,
}

pub async fn call(client: &KisClient, market: Market, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("퇴직연금 미체결내역은 모의투자 미지원 API입니다");
    }
    let tr_id = match market {
        Market::KrxOnly => TR_ID_KRX,
        Market::KrxNxtSor => TR_ID_NXT_SOR,
    };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("USER_DVSN_CD", req.user_dvsn_cd.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("CCLD_NCCS_DVSN", req.ccld_nccs_dvsn.as_str()),
        ("INQR_DVSN_3", req.inqr_dvsn_3.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, tr_id, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
