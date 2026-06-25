//! 국내주식 장운영정보 (통합) — WebSocket /tryitout/H0UNMKO0
//!
//! 스펙: .agent/specs/domestic_stock__realtime__h0unmko0.md
//! 실시간 WebSocket API. 모의투자 미지원. KRX+NXT 통합.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0UNMKO0";

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
    pub trht_yn: String,
    pub tr_susp_reas_cntt: String,
    pub mkop_cls_code: String,
    pub antc_mkop_cls_code: String,
    pub mrkt_trtm_cls_code: String,
    pub divi_app_cls_code: String,
    pub iscd_stat_cls_code: String,
    pub vi_cls_code: String,
    pub ovtm_vi_cls_code: String,
    pub exch_cls_code: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 10 {
        return Err(anyhow!("필드 수 부족: {} < 10", f.len()));
    }
    Ok(Response {
        trht_yn: f[0].to_string(),
        tr_susp_reas_cntt: f[1].to_string(),
        mkop_cls_code: f[2].to_string(),
        antc_mkop_cls_code: f[3].to_string(),
        mrkt_trtm_cls_code: f[4].to_string(),
        divi_app_cls_code: f[5].to_string(),
        iscd_stat_cls_code: f[6].to_string(),
        vi_cls_code: f[7].to_string(),
        ovtm_vi_cls_code: f[8].to_string(),
        exch_cls_code: f[9].to_string(),
    })
}
