//! 국내지수 실시간예상체결 — WebSocket /tryitout/H0UPANC0
//!
//! 스펙: .agent/specs/domestic_stock__realtime__h0upanc0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0UPANC0";

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
    pub bstp_cls_code: String,
    pub bsop_hour: String,
    pub prpr_nmix: String,
    pub prdy_vrss_sign: String,
    pub bstp_nmix_prdy_vrss: String,
    pub acml_vol: String,
    pub acml_tr_pbmn: String,
    pub pcas_vol: String,
    pub pcas_tr_pbmn: String,
    pub prdy_ctrt: String,
    pub oprc_nmix: String,
    pub nmix_hgpr: String,
    pub nmix_lwpr: String,
    pub oprc_vrss_nmix_prpr: String,
    pub oprc_vrss_nmix_sign: String,
    pub hgpr_vrss_nmix_prpr: String,
    pub hgpr_vrss_nmix_sign: String,
    pub lwpr_vrss_nmix_prpr: String,
    pub lwpr_vrss_nmix_sign: String,
    pub prdy_clpr_vrss_oprc_rate: String,
    pub prdy_clpr_vrss_hgpr_rate: String,
    pub prdy_clpr_vrss_lwpr_rate: String,
    pub uplm_issu_cnt: String,
    pub ascn_issu_cnt: String,
    pub stnr_issu_cnt: String,
    pub down_issu_cnt: String,
    pub lslm_issu_cnt: String,
    pub qtqt_ascn_issu_cnt: String,
    pub qtqt_down_issu_cnt: String,
    pub tick_vrss: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 30 {
        return Err(anyhow!("필드 수 부족: {} < 30", f.len()));
    }
    Ok(Response {
        bstp_cls_code: f[0].to_string(),
        bsop_hour: f[1].to_string(),
        prpr_nmix: f[2].to_string(),
        prdy_vrss_sign: f[3].to_string(),
        bstp_nmix_prdy_vrss: f[4].to_string(),
        acml_vol: f[5].to_string(),
        acml_tr_pbmn: f[6].to_string(),
        pcas_vol: f[7].to_string(),
        pcas_tr_pbmn: f[8].to_string(),
        prdy_ctrt: f[9].to_string(),
        oprc_nmix: f[10].to_string(),
        nmix_hgpr: f[11].to_string(),
        nmix_lwpr: f[12].to_string(),
        oprc_vrss_nmix_prpr: f[13].to_string(),
        oprc_vrss_nmix_sign: f[14].to_string(),
        hgpr_vrss_nmix_prpr: f[15].to_string(),
        hgpr_vrss_nmix_sign: f[16].to_string(),
        lwpr_vrss_nmix_prpr: f[17].to_string(),
        lwpr_vrss_nmix_sign: f[18].to_string(),
        prdy_clpr_vrss_oprc_rate: f[19].to_string(),
        prdy_clpr_vrss_hgpr_rate: f[20].to_string(),
        prdy_clpr_vrss_lwpr_rate: f[21].to_string(),
        uplm_issu_cnt: f[22].to_string(),
        ascn_issu_cnt: f[23].to_string(),
        stnr_issu_cnt: f[24].to_string(),
        down_issu_cnt: f[25].to_string(),
        lslm_issu_cnt: f[26].to_string(),
        qtqt_ascn_issu_cnt: f[27].to_string(),
        qtqt_down_issu_cnt: f[28].to_string(),
        tick_vrss: f[29].to_string(),
    })
}
