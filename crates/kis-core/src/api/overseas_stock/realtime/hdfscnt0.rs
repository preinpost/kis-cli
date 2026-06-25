//! 해외주식 실시간체결가 — WebSocket /tryitout/HDFSCNT0
//!
//! 스펙: .agent/specs/overseas_stock__realtime__hdfscnt0.md
//! 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "HDFSCNT0";

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
    pub rsym: String,
    pub symb: String,
    pub zdiv: String,
    pub tymd: String,
    pub xymd: String,
    pub xhms: String,
    pub kymd: String,
    pub khms: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub last: String,
    pub sign: String,
    pub diff: String,
    pub rate: String,
    pub pbid: String,
    pub pask: String,
    pub vbid: String,
    pub vask: String,
    pub evol: String,
    pub tvol: String,
    pub tamt: String,
    pub bivl: String,
    pub asvl: String,
    pub strn: String,
    pub mtyp: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 26 {
        return Err(anyhow!("필드 수 부족: {} < 26", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        rsym: g(0),
        symb: g(1),
        zdiv: g(2),
        tymd: g(3),
        xymd: g(4),
        xhms: g(5),
        kymd: g(6),
        khms: g(7),
        open: g(8),
        high: g(9),
        low: g(10),
        last: g(11),
        sign: g(12),
        diff: g(13),
        rate: g(14),
        pbid: g(15),
        pask: g(16),
        vbid: g(17),
        vask: g(18),
        evol: g(19),
        tvol: g(20),
        tamt: g(21),
        bivl: g(22),
        asvl: g(23),
        strn: g(24),
        mtyp: g(25),
    })
}
