//! 주식옵션 실시간예상체결 — WebSocket /tryitout/H0ZOANC0
//!
//! 스펙: .agent/specs/futureoption_domestic__realtime__h0zoanc0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0ZOANC0";

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
    pub antc_cnpr: String,
    pub antc_cntg_vrss: String,
    pub antc_cntg_vrss_sign: String,
    pub antc_cntg_prdy_ctrt: String,
    pub antc_mkop_cls_code: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 7 {
        return Err(anyhow!("필드 수 부족: {} < 7", f.len()));
    }
    Ok(Response {
        optn_shrn_iscd: f[0].to_string(),
        bsop_hour: f[1].to_string(),
        antc_cnpr: f[2].to_string(),
        antc_cntg_vrss: f[3].to_string(),
        antc_cntg_vrss_sign: f[4].to_string(),
        antc_cntg_prdy_ctrt: f[5].to_string(),
        antc_mkop_cls_code: f[6].to_string(),
    })
}
