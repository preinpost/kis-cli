//! 채권지수 실시간체결가 — WebSocket /tryitout/H0BICNT0
//!
//! 스펙: .agent/specs/bond__realtime__h0bicnt0.md
//! 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0BICNT0";

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
    pub nmix_id: String,
    pub stnd_date1: String,
    pub trnm_hour: String,
    pub totl_ernn_nmix_oprc: String,
    pub totl_ernn_nmix_hgpr: String,
    pub totl_ernn_nmix_lwpr: String,
    pub totl_ernn_nmix: String,
    pub prdy_totl_ernn_nmix: String,
    pub totl_ernn_nmix_prdy_vrss: String,
    pub totl_ernn_nmix_prdy_vrss_sign: String,
    pub totl_ernn_nmix_prdy_ctrt: String,
    pub clen_prc_nmix: String,
    pub mrkt_prc_nmix: String,
    pub bond_call_rnvs_nmix: String,
    pub bond_zero_rnvs_nmix: String,
    pub bond_futs_thpr: String,
    pub bond_avrg_drtn_val: String,
    pub bond_avrg_cnvx_val: String,
    pub bond_avrg_ytm_val: String,
    pub bond_avrg_frdl_ytm_val: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 20 {
        return Err(anyhow!("필드 수 부족: {} < 20", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        nmix_id: g(0),
        stnd_date1: g(1),
        trnm_hour: g(2),
        totl_ernn_nmix_oprc: g(3),
        totl_ernn_nmix_hgpr: g(4),
        totl_ernn_nmix_lwpr: g(5),
        totl_ernn_nmix: g(6),
        prdy_totl_ernn_nmix: g(7),
        totl_ernn_nmix_prdy_vrss: g(8),
        totl_ernn_nmix_prdy_vrss_sign: g(9),
        totl_ernn_nmix_prdy_ctrt: g(10),
        clen_prc_nmix: g(11),
        mrkt_prc_nmix: g(12),
        bond_call_rnvs_nmix: g(13),
        bond_zero_rnvs_nmix: g(14),
        bond_futs_thpr: g(15),
        bond_avrg_drtn_val: g(16),
        bond_avrg_ytm_val: g(17),
        bond_avrg_cnvx_val: g(18),
        bond_avrg_frdl_ytm_val: g(19),
    })
}
