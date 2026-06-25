//! KRX야간선물 실시간종목체결 — WebSocket /tryitout/H0MFCNT0
//!
//! 스펙: .agent/specs/futureoption_domestic__realtime__h0mfcnt0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0MFCNT0";

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
    pub futs_shrn_iscd: String,
    pub bsop_hour: String,
    pub futs_prdy_vrss: String,
    pub prdy_vrss_sign: String,
    pub futs_prdy_ctrt: String,
    pub futs_prpr: String,
    pub futs_oprc: String,
    pub futs_hgpr: String,
    pub futs_lwpr: String,
    pub last_cnqn: String,
    pub acml_vol: String,
    pub acml_tr_pbmn: String,
    pub hts_thpr: String,
    pub mrkt_basis: String,
    pub dprt: String,
    pub nmsc_fctn_stpl_prc: String,
    pub fmsc_fctn_stpl_prc: String,
    pub spead_prc: String,
    pub hts_otst_stpl_qty: String,
    pub otst_stpl_qty_icdc: String,
    pub oprc_hour: String,
    pub oprc_vrss_prpr_sign: String,
    pub oprc_vrss_nmix_prpr: String,
    pub hgpr_hour: String,
    pub hgpr_vrss_prpr_sign: String,
    pub hgpr_vrss_nmix_prpr: String,
    pub lwpr_hour: String,
    pub lwpr_vrss_prpr_sign: String,
    pub lwpr_vrss_nmix_prpr: String,
    pub shnu_rate: String,
    pub cttr: String,
    pub esdg: String,
    pub otst_stpl_rgbf_qty_icdc: String,
    pub thpr_basis: String,
    pub futs_askp1: String,
    pub futs_bidp1: String,
    pub askp_rsqn1: String,
    pub bidp_rsqn1: String,
    pub seln_cntg_csnu: String,
    pub shnu_cntg_csnu: String,
    pub ntby_cntg_csnu: String,
    pub seln_cntg_smtn: String,
    pub shnu_cntg_smtn: String,
    pub total_askp_rsqn: String,
    pub total_bidp_rsqn: String,
    pub prdy_vol_vrss_acml_vol_rate: String,
    pub dynm_mxpr: String,
    pub dynm_llam: String,
    pub dynm_prc_limt_yn: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 49 {
        return Err(anyhow!("필드 수 부족: {} < 49", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        futs_shrn_iscd: g(0),
        bsop_hour: g(1),
        futs_prdy_vrss: g(2),
        prdy_vrss_sign: g(3),
        futs_prdy_ctrt: g(4),
        futs_prpr: g(5),
        futs_oprc: g(6),
        futs_hgpr: g(7),
        futs_lwpr: g(8),
        last_cnqn: g(9),
        acml_vol: g(10),
        acml_tr_pbmn: g(11),
        hts_thpr: g(12),
        mrkt_basis: g(13),
        dprt: g(14),
        nmsc_fctn_stpl_prc: g(15),
        fmsc_fctn_stpl_prc: g(16),
        spead_prc: g(17),
        hts_otst_stpl_qty: g(18),
        otst_stpl_qty_icdc: g(19),
        oprc_hour: g(20),
        oprc_vrss_prpr_sign: g(21),
        oprc_vrss_nmix_prpr: g(22),
        hgpr_hour: g(23),
        hgpr_vrss_prpr_sign: g(24),
        hgpr_vrss_nmix_prpr: g(25),
        lwpr_hour: g(26),
        lwpr_vrss_prpr_sign: g(27),
        lwpr_vrss_nmix_prpr: g(28),
        shnu_rate: g(29),
        cttr: g(30),
        esdg: g(31),
        otst_stpl_rgbf_qty_icdc: g(32),
        thpr_basis: g(33),
        futs_askp1: g(34),
        futs_bidp1: g(35),
        askp_rsqn1: g(36),
        bidp_rsqn1: g(37),
        seln_cntg_csnu: g(38),
        shnu_cntg_csnu: g(39),
        ntby_cntg_csnu: g(40),
        seln_cntg_smtn: g(41),
        shnu_cntg_smtn: g(42),
        total_askp_rsqn: g(43),
        total_bidp_rsqn: g(44),
        prdy_vol_vrss_acml_vol_rate: g(45),
        dynm_mxpr: g(46),
        dynm_llam: g(47),
        dynm_prc_limt_yn: g(48),
    })
}
