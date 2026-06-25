//! 주식예약주문조회 — GET /uapi/domestic-stock/v1/trading/order-resv-ccnl
//!
//! 스펙: .agent/specs/domestic_stock__order_account__order_resv_ccnl.md
//!
//! 모의투자 미지원. output이 array.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/order-resv-ccnl";
pub const TR_ID: &str = "CTSC0004R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub rsvn_ord_ord_dt: String,
    pub rsvn_ord_end_dt: String,
    pub rsvn_ord_seq: String,
    /// "00" 입력
    pub tmnl_mdia_kind_cd: String,
    pub cano: String,
    pub acnt_prdt_cd: String,
    /// 0 전체, 1 처리내역, 2 미처리내역
    pub prcs_dvsn_cd: String,
    /// "Y" 유효한 주문만
    pub cncl_yn: String,
    /// 공백 입력 시 전체 조회
    pub pdno: String,
    pub sll_buy_dvsn_cd: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub rsvn_ord_seq: String,
    #[serde(default)]
    pub rsvn_ord_ord_dt: String,
    #[serde(default)]
    pub rsvn_ord_rcit_dt: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub ord_dvsn_cd: String,
    #[serde(default)]
    pub ord_rsvn_qty: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub cncl_ord_dt: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub ctac_tlno: String,
    #[serde(default)]
    pub rjct_rson2: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub rsvn_ord_rcit_tmd: String,
    #[serde(default)]
    pub kor_item_shtn_name: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub ord_rsvn_unpr: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub cncl_rcit_tmd: String,
    #[serde(default)]
    pub prcs_rslt: String,
    #[serde(default)]
    pub ord_dvsn_name: String,
    #[serde(default)]
    pub tmnl_mdia_kind_cd: String,
    #[serde(default)]
    pub rsvn_end_dt: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("주식예약주문조회는 모의투자 미지원 API입니다");
    }
    let params = [
        ("RSVN_ORD_ORD_DT", req.rsvn_ord_ord_dt.as_str()),
        ("RSVN_ORD_END_DT", req.rsvn_ord_end_dt.as_str()),
        ("RSVN_ORD_SEQ", req.rsvn_ord_seq.as_str()),
        ("TMNL_MDIA_KIND_CD", req.tmnl_mdia_kind_cd.as_str()),
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("PRCS_DVSN_CD", req.prcs_dvsn_cd.as_str()),
        ("CNCL_YN", req.cncl_yn.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
