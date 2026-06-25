//! 주식옵션 실시간체결가 — WebSocket /tryitout/H0ZOCNT0
//!
//! 스펙: .agent/specs/futureoption_domestic__realtime__h0zocnt0.md
//! 실시간 WebSocket API. 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "H0ZOCNT0";

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
    pub optn_shrn_iscd: String,
    pub bsop_hour: String,
    pub optn_prpr: String,
    pub prdy_vrss_sign: String,
    pub optn_prdy_vrss: String,
    pub prdy_ctrt: String,
    pub optn_oprc: String,
    pub optn_hgpr: String,
    pub optn_lwpr: String,
    pub last_cnqn: String,
    pub acml_vol: String,
    pub acml_tr_pbmn: String,
    pub hts_thpr: String,
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
    pub prmm_val: String,
    pub invl_val: String,
    pub tmvl_val: String,
    pub delta: String,
    pub gama: String,
    pub vega: String,
    pub theta: String,
    pub rho: String,
    pub hts_ints_vltl: String,
    pub esdg: String,
    pub otst_stpl_rgbf_qty_icdc: String,
    pub thpr_basis: String,
    pub unas_hist_vltl: String,
    pub cttr: String,
    pub dprt: String,
    pub mrkt_basis: String,
    pub optn_askp1: String,
    pub optn_bidp1: String,
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
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 53 {
        return Err(anyhow!("필드 수 부족: {} < 53", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        optn_shrn_iscd: g(0),
        bsop_hour: g(1),
        optn_prpr: g(2),
        prdy_vrss_sign: g(3),
        optn_prdy_vrss: g(4),
        prdy_ctrt: g(5),
        optn_oprc: g(6),
        optn_hgpr: g(7),
        optn_lwpr: g(8),
        last_cnqn: g(9),
        acml_vol: g(10),
        acml_tr_pbmn: g(11),
        hts_thpr: g(12),
        hts_otst_stpl_qty: g(13),
        otst_stpl_qty_icdc: g(14),
        oprc_hour: g(15),
        oprc_vrss_prpr_sign: g(16),
        oprc_vrss_nmix_prpr: g(17),
        hgpr_hour: g(18),
        hgpr_vrss_prpr_sign: g(19),
        hgpr_vrss_nmix_prpr: g(20),
        lwpr_hour: g(21),
        lwpr_vrss_prpr_sign: g(22),
        lwpr_vrss_nmix_prpr: g(23),
        shnu_rate: g(24),
        prmm_val: g(25),
        invl_val: g(26),
        tmvl_val: g(27),
        delta: g(28),
        gama: g(29),
        vega: g(30),
        theta: g(31),
        rho: g(32),
        hts_ints_vltl: g(33),
        esdg: g(34),
        otst_stpl_rgbf_qty_icdc: g(35),
        thpr_basis: g(36),
        unas_hist_vltl: g(37),
        cttr: g(38),
        dprt: g(39),
        mrkt_basis: g(40),
        optn_askp1: g(41),
        optn_bidp1: g(42),
        askp_rsqn1: g(43),
        bidp_rsqn1: g(44),
        seln_cntg_csnu: g(45),
        shnu_cntg_csnu: g(46),
        ntby_cntg_csnu: g(47),
        seln_cntg_smtn: g(48),
        shnu_cntg_smtn: g(49),
        total_askp_rsqn: g(50),
        total_bidp_rsqn: g(51),
        prdy_vol_vrss_acml_vol_rate: g(52),
    })
}
