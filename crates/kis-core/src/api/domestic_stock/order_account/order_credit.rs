//! 주식주문(신용) — POST /uapi/domestic-stock/v1/trading/order-credit
//!
//! 스펙: .agent/specs/domestic_stock__order_account__order_credit.md
//!
//! 모의투자 미지원. `is_mock`일 경우 호출 거부.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/order-credit";
pub const TR_ID_BUY: &str = "TTTC0052U";
pub const TR_ID_SELL: &str = "TTTC0051U";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    #[serde(rename = "CANO")]
    pub cano: String,
    #[serde(rename = "ACNT_PRDT_CD")]
    pub acnt_prdt_cd: String,
    #[serde(rename = "PDNO")]
    pub pdno: String,
    /// 매도유형 (공란 입력)
    #[serde(rename = "SLL_TYPE", skip_serializing_if = "Option::is_none")]
    pub sll_type: Option<String>,
    /// [매도] 22/24/25/27  [매수] 21/23/26/28
    #[serde(rename = "CRDT_TYPE")]
    pub crdt_type: String,
    /// yyyyMMdd. 매수는 오늘, 매도는 매도할 종목의 대출일자
    #[serde(rename = "LOAN_DT")]
    pub loan_dt: String,
    #[serde(rename = "ORD_DVSN")]
    pub ord_dvsn: String,
    #[serde(rename = "ORD_QTY")]
    pub ord_qty: String,
    /// 시장가는 "0"
    #[serde(rename = "ORD_UNPR")]
    pub ord_unpr: String,
    #[serde(rename = "RSVN_ORD_YN", skip_serializing_if = "Option::is_none")]
    pub rsvn_ord_yn: Option<String>,
    #[serde(rename = "EMGC_ORD_YN", skip_serializing_if = "Option::is_none")]
    pub emgc_ord_yn: Option<String>,
    #[serde(rename = "PGTR_DVSN", skip_serializing_if = "Option::is_none")]
    pub pgtr_dvsn: Option<String>,
    #[serde(rename = "MGCO_APTM_ODNO", skip_serializing_if = "Option::is_none")]
    pub mgco_aptm_odno: Option<String>,
    #[serde(rename = "LQTY_TR_NGTN_DTL_NO", skip_serializing_if = "Option::is_none")]
    pub lqty_tr_ngtn_dtl_no: Option<String>,
    #[serde(rename = "LQTY_TR_AGMT_NO", skip_serializing_if = "Option::is_none")]
    pub lqty_tr_agmt_no: Option<String>,
    #[serde(rename = "LQTY_TR_NGTN_ID", skip_serializing_if = "Option::is_none")]
    pub lqty_tr_ngtn_id: Option<String>,
    #[serde(rename = "LP_ORD_YN", skip_serializing_if = "Option::is_none")]
    pub lp_ord_yn: Option<String>,
    #[serde(rename = "MDIA_ODNO", skip_serializing_if = "Option::is_none")]
    pub mdia_odno: Option<String>,
    #[serde(rename = "ORD_SVR_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub ord_svr_dvsn_cd: Option<String>,
    #[serde(rename = "PGM_NMPR_STMT_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub pgm_nmpr_stmt_dvsn_cd: Option<String>,
    #[serde(rename = "CVRG_SLCT_RSON_CD", skip_serializing_if = "Option::is_none")]
    pub cvrg_slct_rson_cd: Option<String>,
    #[serde(rename = "CVRG_SEQ", skip_serializing_if = "Option::is_none")]
    pub cvrg_seq: Option<String>,
    #[serde(rename = "EXCG_ID_DVSN_CD", skip_serializing_if = "Option::is_none")]
    pub excg_id_dvsn_cd: Option<String>,
    /// 스탑지정가(ORD_DVSN=22) 사용 시 필수
    #[serde(rename = "CNDT_PRIC", skip_serializing_if = "Option::is_none")]
    pub cndt_pric: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub krx_fwdg_ord_orgno: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub ord_tmd: String,
}

pub async fn call(client: &KisClient, side: Side, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("주식주문(신용)은 모의투자 미지원 API입니다");
    }
    let tr_id = match side {
        Side::Buy => TR_ID_BUY,
        Side::Sell => TR_ID_SELL,
    };
    let resp = client.post_json(ENDPOINT, tr_id, req, &[]).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let parsed: Response = serde_json::from_value(output)?;
    Ok(parsed)
}
