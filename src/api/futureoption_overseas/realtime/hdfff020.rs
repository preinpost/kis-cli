//! 해외선물옵션 실시간체결가 — WebSocket /tryitout/HDFFF020
//!
//! 스펙: .agent/specs/futureoption_overseas__realtime__hdfff020.md
//! 모의투자 미지원. CME, SGX 실시간시세는 유료시세 신청 필수.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "HDFFF020";

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
    pub series_cd: String,
    pub bsns_date: String,
    pub mrkt_open_date: String,
    pub mrkt_open_time: String,
    pub mrkt_close_date: String,
    pub mrkt_close_time: String,
    pub prev_price: String,
    pub recv_date: String,
    pub recv_time: String,
    pub active_flag: String,
    pub last_price: String,
    pub last_qntt: String,
    pub prev_diff_price: String,
    pub prev_diff_rate: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub vol: String,
    pub prev_sign: String,
    pub quotsign: String,
    pub recv_time2: String,
    pub psttl_price: String,
    pub psttl_sign: String,
    pub psttl_diff_price: String,
    pub psttl_diff_rate: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 25 {
        return Err(anyhow!("필드 수 부족: {} < 25", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        series_cd: g(0),
        bsns_date: g(1),
        mrkt_open_date: g(2),
        mrkt_open_time: g(3),
        mrkt_close_date: g(4),
        mrkt_close_time: g(5),
        prev_price: g(6),
        recv_date: g(7),
        recv_time: g(8),
        active_flag: g(9),
        last_price: g(10),
        last_qntt: g(11),
        prev_diff_price: g(12),
        prev_diff_rate: g(13),
        open_price: g(14),
        high_price: g(15),
        low_price: g(16),
        vol: g(17),
        prev_sign: g(18),
        quotsign: g(19),
        recv_time2: g(20),
        psttl_price: g(21),
        psttl_sign: g(22),
        psttl_diff_price: g(23),
        psttl_diff_rate: g(24),
    })
}
