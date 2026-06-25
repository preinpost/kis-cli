//! 일반채권 실시간호가 — WebSocket /tryitout/H0BJASP0
//!
//! 스펙: .agent/specs/bond__realtime__h0bjasp0.md
//! 모의투자 미지원.
//! 주의: 스펙의 실전TRID/Request Body tr_id는 H0BJCNT0으로 표기되어 있으나,
//!       이는 체결가 API와 충돌하는 명백한 문서 오기. 엔드포인트 경로(H0BJASP0)를 따른다.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0BJASP0";

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
    pub stck_cntg_hour: String,
    pub askp_ert1: String,
    pub bidp_ert1: String,
    pub askp1: String,
    pub bidp1: String,
    pub askp_rsqn1: String,
    pub bidp_rsqn1: String,
    pub askp_ert2: String,
    pub bidp_ert2: String,
    pub askp2: String,
    pub bidp2: String,
    pub askp_rsqn2: String,
    pub bidp_rsqn2: String,
    pub askp_ert3: String,
    pub bidp_ert3: String,
    pub askp3: String,
    pub bidp3: String,
    pub askp_rsqn3: String,
    pub bidp_rsqn3: String,
    pub askp_ert4: String,
    pub bidp_ert4: String,
    pub askp4: String,
    pub bidp4: String,
    pub askp_rsqn4: String,
    pub bidp_rsqn4: String,
    pub askp_ert5: String,
    pub bidp_ert5: String,
    pub askp5: String,
    pub bidp5: String,
    pub askp_rsqn52: String,
    pub bidp_rsqn53: String,
    pub total_askp_rsqn: String,
    pub total_bidp_rsqn: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 34 {
        return Err(anyhow!("필드 수 부족: {} < 34", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        stnd_iscd: g(0),
        stck_cntg_hour: g(1),
        askp_ert1: g(2),
        bidp_ert1: g(3),
        askp1: g(4),
        bidp1: g(5),
        askp_rsqn1: g(6),
        bidp_rsqn1: g(7),
        askp_ert2: g(8),
        bidp_ert2: g(9),
        askp2: g(10),
        bidp2: g(11),
        askp_rsqn2: g(12),
        bidp_rsqn2: g(13),
        askp_ert3: g(14),
        bidp_ert3: g(15),
        askp3: g(16),
        bidp3: g(17),
        askp_rsqn3: g(18),
        bidp_rsqn3: g(19),
        askp_ert4: g(20),
        bidp_ert4: g(21),
        askp4: g(22),
        bidp4: g(23),
        askp_rsqn4: g(24),
        bidp_rsqn4: g(25),
        askp_ert5: g(26),
        bidp_ert5: g(27),
        askp5: g(28),
        bidp5: g(29),
        askp_rsqn52: g(30),
        bidp_rsqn53: g(31),
        total_askp_rsqn: g(32),
        total_bidp_rsqn: g(33),
    })
}
