//! 해외주식 실시간체결통보 — WebSocket /tryitout/H0GSCNI0
//!
//! 스펙: .agent/specs/overseas_stock__realtime__h0gscni0.md
//! 실시간 WebSocket API. 실전: H0GSCNI0, 모의: H0GSCNI9.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID_REAL: &str = "H0GSCNI0";
pub const TR_ID_MOCK: &str = "H0GSCNI9";

pub fn tr_id(is_mock: bool) -> &'static str {
    if is_mock { TR_ID_MOCK } else { TR_ID_REAL }
}

pub fn subscribe_payload(
    approval_key: &str,
    custtype: &str,
    tr_type: &str,
    is_mock: bool,
    tr_key: &str,
) -> serde_json::Value {
    serde_json::json!({
        "header": {
            "approval_key": approval_key,
            "custtype": custtype,
            "tr_type": tr_type,
            "content-type": "utf-8",
        },
        "body": {
            "input": {
                "tr_id": tr_id(is_mock),
                "tr_key": tr_key,
            },
        },
    })
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Response {
    pub cust_id: String,
    pub acnt_no: String,
    pub oder_no: String,
    pub ooder_no: String,
    pub seln_byov_cls: String,
    pub rctf_cls: String,
    pub oder_kind2: String,
    pub stck_shrn_iscd: String,
    pub cntg_qty: String,
    pub cntg_unpr: String,
    pub stck_cntg_hour: String,
    pub rfus_yn: String,
    pub cntg_yn: String,
    pub acpt_yn: String,
    pub brnc_no: String,
    pub oder_qty: String,
    pub acnt_name: String,
    pub cntg_isnm: String,
    pub oder_cond: String,
    pub debt_gb: String,
    pub debt_date: String,
    pub start_tm: String,
    pub end_tm: String,
    pub tm_div_tp: String,
    pub cntg_unpr12: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 25 {
        return Err(anyhow!("필드 수 부족: {} < 25", f.len()));
    }
    Ok(Response {
        cust_id: f[0].to_string(),
        acnt_no: f[1].to_string(),
        oder_no: f[2].to_string(),
        ooder_no: f[3].to_string(),
        seln_byov_cls: f[4].to_string(),
        rctf_cls: f[5].to_string(),
        oder_kind2: f[6].to_string(),
        stck_shrn_iscd: f[7].to_string(),
        cntg_qty: f[8].to_string(),
        cntg_unpr: f[9].to_string(),
        stck_cntg_hour: f[10].to_string(),
        rfus_yn: f[11].to_string(),
        cntg_yn: f[12].to_string(),
        acpt_yn: f[13].to_string(),
        brnc_no: f[14].to_string(),
        oder_qty: f[15].to_string(),
        acnt_name: f[16].to_string(),
        cntg_isnm: f[17].to_string(),
        oder_cond: f[18].to_string(),
        debt_gb: f[19].to_string(),
        debt_date: f[20].to_string(),
        start_tm: f[21].to_string(),
        end_tm: f[22].to_string(),
        tm_div_tp: f[23].to_string(),
        cntg_unpr12: f[24].to_string(),
    })
}
