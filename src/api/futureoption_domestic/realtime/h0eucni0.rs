//! KRX야간옵션 실시간체결통보 — WebSocket /tryitout/H0EUCNI0
//!
//! 스펙: .agent/specs/futureoption_domestic__realtime__h0eucni0.md
//! 모의투자 미지원. 스펙 명시 TR_ID: H0MFCNI0.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0MFCNI0";

pub fn subscribe_payload(approval_key: &str, custtype: &str, tr_type: &str, tr_key: &str) -> serde_json::Value {
    serde_json::json!({
        "header": {
            "approval_key": approval_key,
            "custtype": custtype,
            "tr_type": tr_type,
            "content-type": "utf-8",
        },
        "body": {
            "input": {
                "tr_id": TR_ID,
                "tr_key": tr_key,
            },
        },
    })
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Response {
    pub cust_id: String,
    pub acnt_no: String,
    pub oder_no: String,
    pub ooder_no: String,
    pub seln_byov_cls: String,
    pub rctf_cls: String,
    pub oder_kind2: String,
    pub stck_shrn_iscd: String,
    pub cntg_qty: String,
    pub cntg_unpr: String,
    pub stck_cntg_hour: String,
    pub rfus_yn: String,
    pub cntg_yn: String,
    pub acpt_yn: String,
    pub brnc_no: String,
    pub oder_qty: String,
    pub acnt_name: String,
    pub cntg_isnm: String,
    pub oder_cond: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 19 {
        return Err(anyhow!("필드 수 부족: {} < 19", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        cust_id: g(0),
        acnt_no: g(1),
        oder_no: g(2),
        ooder_no: g(3),
        seln_byov_cls: g(4),
        rctf_cls: g(5),
        oder_kind2: g(6),
        stck_shrn_iscd: g(7),
        cntg_qty: g(8),
        cntg_unpr: g(9),
        stck_cntg_hour: g(10),
        rfus_yn: g(11),
        cntg_yn: g(12),
        acpt_yn: g(13),
        brnc_no: g(14),
        oder_qty: g(15),
        acnt_name: g(16),
        cntg_isnm: g(17),
        oder_cond: g(18),
    })
}
