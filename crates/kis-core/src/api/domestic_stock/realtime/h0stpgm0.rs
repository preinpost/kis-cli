//! 국내주식 실시간프로그램매매 (KRX) — WebSocket /tryitout/H0STPGM0
//!
//! 스펙: .agent/specs/domestic_stock__realtime__h0stpgm0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0STPGM0";

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
    pub stck_cntg_hour: String,
    pub seln_cnqn: String,
    pub seln_tr_pbmn: String,
    pub shnu_cnqn: String,
    pub shnu_tr_pbmn: String,
    pub ntby_cnqn: String,
    pub ntby_tr_pbmn: String,
    pub seln_rsqn: String,
    pub shnu_rsqn: String,
    pub whol_ntby_qty: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 11 {
        return Err(anyhow!("필드 수 부족: {} < 11", f.len()));
    }
    Ok(Response {
        mksc_shrn_iscd: f[0].to_string(),
        stck_cntg_hour: f[1].to_string(),
        seln_cnqn: f[2].to_string(),
        seln_tr_pbmn: f[3].to_string(),
        shnu_cnqn: f[4].to_string(),
        shnu_tr_pbmn: f[5].to_string(),
        ntby_cnqn: f[6].to_string(),
        ntby_tr_pbmn: f[7].to_string(),
        seln_rsqn: f[8].to_string(),
        shnu_rsqn: f[9].to_string(),
        whol_ntby_qty: f[10].to_string(),
    })
}
