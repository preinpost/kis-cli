//! 국내주식 실시간예상체결 (KRX) — WebSocket /tryitout/H0STANC0
//!
//! 스펙: .agent/specs/domestic_stock__realtime__h0stanc0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0STANC0";

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
    pub stck_prpr: String,
    pub prdy_vrss_sign: String,
    pub prdy_vrss: String,
    pub prdy_ctrt: String,
    pub wghn_avrg_stck_prc: String,
    pub stck_oprc: String,
    pub stck_hgpr: String,
    pub stck_lwpr: String,
    pub askp1: String,
    pub bidp1: String,
    pub cntg_vol: String,
    pub acml_vol: String,
    pub acml_tr_pbmn: String,
    pub seln_cntg_csnu: String,
    pub shnu_cntg_csnu: String,
    pub ntby_cntg_csnu: String,
    pub cttr: String,
    pub seln_cntg_smtn: String,
    pub shnu_cntg_smtn: String,
    pub cntg_cls_code: String,
    pub shnu_rate: String,
    pub prdy_vol_vrss_acml_vol_rate: String,
    pub oprc_hour: String,
    pub oprc_vrss_prpr_sign: String,
    pub oprc_vrss_prpr: String,
    pub hgpr_hour: String,
    pub hgpr_vrss_prpr_sign: String,
    pub hgpr_vrss_prpr: String,
    pub lwpr_hour: String,
    pub lwpr_vrss_prpr_sign: String,
    pub lwpr_vrss_prpr: String,
    pub bsop_date: String,
    pub new_mkop_cls_code: String,
    pub trht_yn: String,
    pub askp_rsqn1: String,
    pub bidp_rsqn1: String,
    pub total_askp_rsqn: String,
    pub total_bidp_rsqn: String,
    pub vol_tnrt: String,
    pub prdy_smns_hour_acml_vol: String,
    pub prdy_smns_hour_acml_vol_rate: String,
    pub hour_cls_code: String,
    pub mrkt_trtm_cls_code: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 45 {
        return Err(anyhow!("필드 수 부족: {} < 45", f.len()));
    }
    Ok(Response {
        mksc_shrn_iscd: f[0].to_string(),
        stck_cntg_hour: f[1].to_string(),
        stck_prpr: f[2].to_string(),
        prdy_vrss_sign: f[3].to_string(),
        prdy_vrss: f[4].to_string(),
        prdy_ctrt: f[5].to_string(),
        wghn_avrg_stck_prc: f[6].to_string(),
        stck_oprc: f[7].to_string(),
        stck_hgpr: f[8].to_string(),
        stck_lwpr: f[9].to_string(),
        askp1: f[10].to_string(),
        bidp1: f[11].to_string(),
        cntg_vol: f[12].to_string(),
        acml_vol: f[13].to_string(),
        acml_tr_pbmn: f[14].to_string(),
        seln_cntg_csnu: f[15].to_string(),
        shnu_cntg_csnu: f[16].to_string(),
        ntby_cntg_csnu: f[17].to_string(),
        cttr: f[18].to_string(),
        seln_cntg_smtn: f[19].to_string(),
        shnu_cntg_smtn: f[20].to_string(),
        cntg_cls_code: f[21].to_string(),
        shnu_rate: f[22].to_string(),
        prdy_vol_vrss_acml_vol_rate: f[23].to_string(),
        oprc_hour: f[24].to_string(),
        oprc_vrss_prpr_sign: f[25].to_string(),
        oprc_vrss_prpr: f[26].to_string(),
        hgpr_hour: f[27].to_string(),
        hgpr_vrss_prpr_sign: f[28].to_string(),
        hgpr_vrss_prpr: f[29].to_string(),
        lwpr_hour: f[30].to_string(),
        lwpr_vrss_prpr_sign: f[31].to_string(),
        lwpr_vrss_prpr: f[32].to_string(),
        bsop_date: f[33].to_string(),
        new_mkop_cls_code: f[34].to_string(),
        trht_yn: f[35].to_string(),
        askp_rsqn1: f[36].to_string(),
        bidp_rsqn1: f[37].to_string(),
        total_askp_rsqn: f[38].to_string(),
        total_bidp_rsqn: f[39].to_string(),
        vol_tnrt: f[40].to_string(),
        prdy_smns_hour_acml_vol: f[41].to_string(),
        prdy_smns_hour_acml_vol_rate: f[42].to_string(),
        hour_cls_code: f[43].to_string(),
        mrkt_trtm_cls_code: f[44].to_string(),
    })
}
