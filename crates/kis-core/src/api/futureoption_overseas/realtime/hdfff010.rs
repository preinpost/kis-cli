//! 해외선물옵션 실시간호가 — WebSocket /tryitout/HDFFF010
//!
//! 스펙: .agent/specs/futureoption_overseas__realtime__hdfff010.md
//! 모의투자 미지원. CME, SGX 실시간시세는 유료시세 신청 필수.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "HDFFF010";

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
    pub recv_date: String,
    pub recv_time: String,
    pub prev_price: String,
    pub bid_qntt_1: String,
    pub bid_num_1: String,
    pub bid_price_1: String,
    pub ask_qntt_1: String,
    pub ask_num_1: String,
    pub ask_price_1: String,
    pub bid_qntt_2: String,
    pub bid_num_2: String,
    pub bid_price_2: String,
    pub ask_qntt_2: String,
    pub ask_num_2: String,
    pub ask_price_2: String,
    pub bid_qntt_3: String,
    pub bid_num_3: String,
    pub bid_price_3: String,
    pub ask_qntt_3: String,
    pub ask_num_3: String,
    pub ask_price_3: String,
    pub bid_qntt_4: String,
    pub bid_num_4: String,
    pub bid_price_4: String,
    pub ask_qntt_4: String,
    pub ask_num_4: String,
    pub ask_price_4: String,
    pub bid_qntt_5: String,
    pub bid_num_5: String,
    pub bid_price_5: String,
    pub ask_qntt_5: String,
    pub ask_num_5: String,
    pub ask_price_5: String,
    pub sttl_price: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 35 {
        return Err(anyhow!("필드 수 부족: {} < 35", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        series_cd: g(0),
        recv_date: g(1),
        recv_time: g(2),
        prev_price: g(3),
        bid_qntt_1: g(4),
        bid_num_1: g(5),
        bid_price_1: g(6),
        ask_qntt_1: g(7),
        ask_num_1: g(8),
        ask_price_1: g(9),
        bid_qntt_2: g(10),
        bid_num_2: g(11),
        bid_price_2: g(12),
        ask_qntt_2: g(13),
        ask_num_2: g(14),
        ask_price_2: g(15),
        bid_qntt_3: g(16),
        bid_num_3: g(17),
        bid_price_3: g(18),
        ask_qntt_3: g(19),
        ask_num_3: g(20),
        ask_price_3: g(21),
        bid_qntt_4: g(22),
        bid_num_4: g(23),
        bid_price_4: g(24),
        ask_qntt_4: g(25),
        ask_num_4: g(26),
        ask_price_4: g(27),
        bid_qntt_5: g(28),
        bid_num_5: g(29),
        bid_price_5: g(30),
        ask_qntt_5: g(31),
        ask_num_5: g(32),
        ask_price_5: g(33),
        sttl_price: g(34),
    })
}
