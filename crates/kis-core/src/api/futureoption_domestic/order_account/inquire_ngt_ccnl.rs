//! (야간)선물옵션 주문체결 내역조회 — GET /uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl
//!
//! 스펙: .agent/specs/futureoption_domestic__order_account__inquire_ngt_ccnl.md
//! 모의투자 미지원. 신 TR_ID `STTN5201R` 사용.

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-futureoption/v1/trading/inquire-ngt-ccnl";
pub const TR_ID: &str = "STTN5201R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    pub strt_ord_dt: String,
    pub end_ord_dt: String,
    pub sll_buy_dvsn_cd: String,
    pub ccld_nccs_dvsn: String,
    pub sort_sqn: String,
    pub strt_odno: String,
    pub pdno: String,
    pub mket_id_cd: String,
    pub fuop_dvsn_cd: String,
    pub scrn_dvsn: String,
    pub ctx_area_fk200: String,
    pub ctx_area_nk200: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Order {
    #[serde(default)]
    pub ord_gno_brno: String,
    #[serde(default)]
    pub cano: String,
    #[serde(default)]
    pub csac_name: String,
    #[serde(default)]
    pub acnt_prdt_cd: String,
    #[serde(default)]
    pub ord_dt: String,
    #[serde(default)]
    pub odno: String,
    #[serde(default)]
    pub orgn_odno: String,
    #[serde(default)]
    pub sll_buy_dvsn_cd: String,
    #[serde(default)]
    pub trad_dvsn_name: String,
    #[serde(default)]
    pub nmpr_type_name: String,
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub prdt_type_cd: String,
    #[serde(default)]
    pub ord_qty: String,
    #[serde(default)]
    pub ord_idx4: String,
    #[serde(default)]
    pub qty: String,
    #[serde(default)]
    pub ord_tmd: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default)]
    pub avg_idx: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default)]
    pub rjct_qty: String,
    #[serde(default)]
    pub ingr_trad_rjct_rson_cd: String,
    #[serde(default)]
    pub ingr_trad_rjct_rson_name: String,
    #[serde(default)]
    pub ord_stfno: String,
    #[serde(default)]
    pub sprd_item_yn: String,
    #[serde(default)]
    pub ord_ip_addr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub tot_ord_qty: String,
    #[serde(default)]
    pub tot_ccld_qty: String,
    #[serde(default, rename = "tot_ccld_qty_SMTL")]
    pub tot_ccld_qty_smtl: String,
    #[serde(default)]
    pub tot_ccld_amt: String,
    #[serde(default, rename = "tot_ccld_amt_SMTL")]
    pub tot_ccld_amt_smtl: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub ctac_tlno: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub orders: Vec<Order>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    if client.is_mock() {
        bail!("(야간)선물옵션 주문체결 내역조회는 모의투자 미지원");
    }
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("STRT_ORD_DT", req.strt_ord_dt.as_str()),
        ("END_ORD_DT", req.end_ord_dt.as_str()),
        ("SLL_BUY_DVSN_CD", req.sll_buy_dvsn_cd.as_str()),
        ("CCLD_NCCS_DVSN", req.ccld_nccs_dvsn.as_str()),
        ("SORT_SQN", req.sort_sqn.as_str()),
        ("STRT_ODNO", req.strt_odno.as_str()),
        ("PDNO", req.pdno.as_str()),
        ("MKET_ID_CD", req.mket_id_cd.as_str()),
        ("FUOP_DVSN_CD", req.fuop_dvsn_cd.as_str()),
        ("SCRN_DVSN", req.scrn_dvsn.as_str()),
        ("CTX_AREA_FK200", req.ctx_area_fk200.as_str()),
        ("CTX_AREA_NK200", req.ctx_area_nk200.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let orders = resp
        .output1
        .map(serde_json::from_value::<Vec<Order>>)
        .transpose()?
        .unwrap_or_default();
    let summary = resp
        .output2
        .and_then(|v| serde_json::from_value::<Summary>(v).ok());
    Ok(Response { orders, summary })
}
