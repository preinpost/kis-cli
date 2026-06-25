//! ELW 실시간예상체결 — WebSocket /tryitout/H0EWANC0
//!
//! 스펙: .agent/specs/domestic_stock__realtime__h0ewanc0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0EWANC0";

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
    pub tmvl_val: String,
    pub prit: String,
    pub prmm_val: String,
    pub gear: String,
    pub prls_qryr_rate: String,
    pub invl_val: String,
    pub prmm_rate: String,
    pub cfp: String,
    pub lvrg_val: String,
    pub delta: String,
    pub gama: String,
    pub vega: String,
    pub theta: String,
    pub rho: String,
    pub hts_ints_vltl: String,
    pub hts_thpr: String,
    pub vol_tnrt: String,
    pub lp_hvol: String,
    pub lp_hldn_rate: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 59 {
        return Err(anyhow!("필드 수 부족: {} < 59", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        mksc_shrn_iscd: g(0),
        stck_cntg_hour: g(1),
        stck_prpr: g(2),
        prdy_vrss_sign: g(3),
        prdy_vrss: g(4),
        prdy_ctrt: g(5),
        wghn_avrg_stck_prc: g(6),
        stck_oprc: g(7),
        stck_hgpr: g(8),
        stck_lwpr: g(9),
        askp1: g(10),
        bidp1: g(11),
        cntg_vol: g(12),
        acml_vol: g(13),
        acml_tr_pbmn: g(14),
        seln_cntg_csnu: g(15),
        shnu_cntg_csnu: g(16),
        ntby_cntg_csnu: g(17),
        cttr: g(18),
        seln_cntg_smtn: g(19),
        shnu_cntg_smtn: g(20),
        cntg_cls_code: g(21),
        shnu_rate: g(22),
        prdy_vol_vrss_acml_vol_rate: g(23),
        oprc_hour: g(24),
        oprc_vrss_prpr_sign: g(25),
        oprc_vrss_prpr: g(26),
        hgpr_hour: g(27),
        hgpr_vrss_prpr_sign: g(28),
        hgpr_vrss_prpr: g(29),
        lwpr_hour: g(30),
        lwpr_vrss_prpr_sign: g(31),
        lwpr_vrss_prpr: g(32),
        bsop_date: g(33),
        new_mkop_cls_code: g(34),
        trht_yn: g(35),
        askp_rsqn1: g(36),
        bidp_rsqn1: g(37),
        total_askp_rsqn: g(38),
        total_bidp_rsqn: g(39),
        tmvl_val: g(40),
        prit: g(41),
        prmm_val: g(42),
        gear: g(43),
        prls_qryr_rate: g(44),
        invl_val: g(45),
        prmm_rate: g(46),
        cfp: g(47),
        lvrg_val: g(48),
        delta: g(49),
        gama: g(50),
        vega: g(51),
        theta: g(52),
        rho: g(53),
        hts_ints_vltl: g(54),
        hts_thpr: g(55),
        vol_tnrt: g(56),
        lp_hvol: g(57),
        lp_hldn_rate: g(58),
    })
}
