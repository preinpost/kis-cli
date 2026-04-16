//! KRX야간옵션 실시간호가 — WebSocket /tryitout/H0EUASP0
//!
//! 스펙: .agent/specs/futureoption_domestic__realtime__h0euasp0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0EUASP0";

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
    pub optn_shrn_iscd: String,
    pub bsop_hour: String,
    pub optn_askp1: String,
    pub optn_askp2: String,
    pub optn_askp3: String,
    pub optn_askp4: String,
    pub optn_askp5: String,
    pub optn_bidp1: String,
    pub optn_bidp2: String,
    pub optn_bidp3: String,
    pub optn_bidp4: String,
    pub optn_bidp5: String,
    pub askp_csnu1: String,
    pub askp_csnu2: String,
    pub askp_csnu3: String,
    pub askp_csnu4: String,
    pub askp_csnu5: String,
    pub bidp_csnu1: String,
    pub bidp_csnu2: String,
    pub bidp_csnu3: String,
    pub bidp_csnu4: String,
    pub bidp_csnu5: String,
    pub askp_rsqn1: String,
    pub askp_rsqn2: String,
    pub askp_rsqn3: String,
    pub askp_rsqn4: String,
    pub askp_rsqn5: String,
    pub bidp_rsqn1: String,
    pub bidp_rsqn2: String,
    pub bidp_rsqn3: String,
    pub bidp_rsqn4: String,
    pub bidp_rsqn5: String,
    pub total_askp_csnu: String,
    pub total_bidp_csnu: String,
    pub total_askp_rsqn: String,
    pub total_bidp_rsqn: String,
    pub total_askp_rsqn_icdc: String,
    pub total_bidp_rsqn_icdc: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 38 {
        return Err(anyhow!("필드 수 부족: {} < 38", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        optn_shrn_iscd: g(0),
        bsop_hour: g(1),
        optn_askp1: g(2),
        optn_askp2: g(3),
        optn_askp3: g(4),
        optn_askp4: g(5),
        optn_askp5: g(6),
        optn_bidp1: g(7),
        optn_bidp2: g(8),
        optn_bidp3: g(9),
        optn_bidp4: g(10),
        optn_bidp5: g(11),
        askp_csnu1: g(12),
        askp_csnu2: g(13),
        askp_csnu3: g(14),
        askp_csnu4: g(15),
        askp_csnu5: g(16),
        bidp_csnu1: g(17),
        bidp_csnu2: g(18),
        bidp_csnu3: g(19),
        bidp_csnu4: g(20),
        bidp_csnu5: g(21),
        askp_rsqn1: g(22),
        askp_rsqn2: g(23),
        askp_rsqn3: g(24),
        askp_rsqn4: g(25),
        askp_rsqn5: g(26),
        bidp_rsqn1: g(27),
        bidp_rsqn2: g(28),
        bidp_rsqn3: g(29),
        bidp_rsqn4: g(30),
        bidp_rsqn5: g(31),
        total_askp_csnu: g(32),
        total_bidp_csnu: g(33),
        total_askp_rsqn: g(34),
        total_bidp_rsqn: g(35),
        total_askp_rsqn_icdc: g(36),
        total_bidp_rsqn_icdc: g(37),
    })
}
