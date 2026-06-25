//! 일반채권 실시간체결가 — WebSocket /tryitout/H0BJCNT0
//!
//! 스펙: .agent/specs/bond__realtime__h0bjcnt0.md
//! 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0BJCNT0";

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
    pub stnd_iscd: String,
    pub bond_isnm: String,
    pub stck_cntg_hour: String,
    pub prdy_vrss_sign: String,
    pub prdy_vrss: String,
    pub prdy_ctrt: String,
    pub stck_prpr: String,
    pub cntg_vol: String,
    pub stck_oprc: String,
    pub stck_hgpr: String,
    pub stck_lwpr: String,
    pub stck_prdy_clpr: String,
    pub bond_cntg_ert: String,
    pub oprc_ert: String,
    pub hgpr_ert: String,
    pub lwpr_ert: String,
    pub acml_vol: String,
    pub prdy_vol: String,
    pub cntg_type_cls_code: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 19 {
        return Err(anyhow!("필드 수 부족: {} < 19", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        stnd_iscd: g(0),
        bond_isnm: g(1),
        stck_cntg_hour: g(2),
        prdy_vrss_sign: g(3),
        prdy_vrss: g(4),
        prdy_ctrt: g(5),
        stck_prpr: g(6),
        cntg_vol: g(7),
        stck_oprc: g(8),
        stck_hgpr: g(9),
        stck_lwpr: g(10),
        stck_prdy_clpr: g(11),
        bond_cntg_ert: g(12),
        oprc_ert: g(13),
        hgpr_ert: g(14),
        lwpr_ert: g(15),
        acml_vol: g(16),
        prdy_vol: g(17),
        cntg_type_cls_code: g(18),
    })
}
