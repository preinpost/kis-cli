//! 해외주식 지연호가(아시아) — WebSocket /tryitout/HDFSASP1
//!
//! 스펙: .agent/specs/overseas_stock__realtime__hdfsasp1.md
//! 아시아 1호가 무료 지연시세. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "HDFSASP1";

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
    pub xymd: String,
    pub xhms: String,
    pub kymd: String,
    pub khms: String,
    pub bvol: String,
    pub avol: String,
    pub bdvl: String,
    pub advl: String,
    pub pbid1: String,
    pub pask1: String,
    pub vbid1: String,
    pub vask1: String,
    pub dbid1: String,
    pub dask1: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 17 {
        return Err(anyhow!("필드 수 부족: {} < 17", f.len()));
    }
    Ok(Response {
        rsym: f[0].to_string(),
        symb: f[1].to_string(),
        zdiv: f[2].to_string(),
        xymd: f[3].to_string(),
        xhms: f[4].to_string(),
        kymd: f[5].to_string(),
        khms: f[6].to_string(),
        bvol: f[7].to_string(),
        avol: f[8].to_string(),
        bdvl: f[9].to_string(),
        advl: f[10].to_string(),
        pbid1: f[11].to_string(),
        pask1: f[12].to_string(),
        vbid1: f[13].to_string(),
        vask1: f[14].to_string(),
        dbid1: f[15].to_string(),
        dask1: f[16].to_string(),
    })
}
