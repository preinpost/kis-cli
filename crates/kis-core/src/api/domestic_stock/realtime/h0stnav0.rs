//! 국내ETF NAV추이 — WebSocket /tryitout/H0STNAV0
//!
//! 스펙: .agent/specs/domestic_stock__realtime__h0stnav0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0STNAV0";

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
    pub mksc_shrn_iscd: String,
    pub nav: String,
    pub nav_prdy_vrss_sign: String,
    pub nav_prdy_vrss: String,
    pub nav_prdy_ctrt: String,
    pub oprc_nav: String,
    pub hprc_nav: String,
    pub lprc_nav: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 8 {
        return Err(anyhow!("필드 수 부족: {} < 8", f.len()));
    }
    Ok(Response {
        mksc_shrn_iscd: f[0].to_string(),
        nav: f[1].to_string(),
        nav_prdy_vrss_sign: f[2].to_string(),
        nav_prdy_vrss: f[3].to_string(),
        nav_prdy_ctrt: f[4].to_string(),
        oprc_nav: f[5].to_string(),
        hprc_nav: f[6].to_string(),
        lprc_nav: f[7].to_string(),
    })
}
