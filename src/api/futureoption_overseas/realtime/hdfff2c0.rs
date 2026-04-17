//! 해외선물옵션 실시간체결내역통보 — WebSocket /tryitout/HDFFF2C0
//!
//! 스펙: .agent/specs/futureoption_overseas__realtime__hdfff2c0.md
//! 모의투자 미지원.

use anyhow::{anyhow, Result};
use serde::Deserialize;

pub const TR_ID: &str = "HDFFF2C0";

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
    pub user_id: String,
    pub acct_no: String,
    pub ord_dt: String,
    pub odno: String,
    pub orgn_ord_dt: String,
    pub orgn_odno: String,
    pub series: String,
    pub rvse_cncl_dvsn_cd: String,
    pub sll_buy_dvsn_cd: String,
    pub cplx_ord_dvsn_cd: String,
    pub prce_tp: String,
    pub fm_excg_rcit_dvsn_cd: String,
    pub ord_qty: String,
    pub fm_lmt_pric: String,
    pub fm_stop_ord_pric: String,
    pub tot_ccld_qty: String,
    pub tot_ccld_uv: String,
    pub ord_remq: String,
    pub fm_ord_grp_dt: String,
    pub ord_grp_stno: String,
    pub ord_dtl_dtime: String,
    pub oprt_dtl_dtime: String,
    pub work_empl: String,
    pub ccld_dt: String,
    pub ccno: String,
    pub api_ccno: String,
    pub ccld_qty: String,
    pub fm_ccld_pric: String,
    pub crcy_cd: String,
    pub trst_fee: String,
    pub ord_mdia_online_yn: String,
    pub fm_ccld_amt: String,
    pub fuop_item_dvsn_cd: String,
}

pub fn parse_frame(data: &str) -> Result<Response> {
    let f: Vec<&str> = data.split('^').collect();
    if f.len() < 33 {
        return Err(anyhow!("필드 수 부족: {} < 33", f.len()));
    }
    let g = |i: usize| f[i].to_string();
    Ok(Response {
        user_id: g(0),
        acct_no: g(1),
        ord_dt: g(2),
        odno: g(3),
        orgn_ord_dt: g(4),
        orgn_odno: g(5),
        series: g(6),
        rvse_cncl_dvsn_cd: g(7),
        sll_buy_dvsn_cd: g(8),
        cplx_ord_dvsn_cd: g(9),
        prce_tp: g(10),
        fm_excg_rcit_dvsn_cd: g(11),
        ord_qty: g(12),
        fm_lmt_pric: g(13),
        fm_stop_ord_pric: g(14),
        tot_ccld_qty: g(15),
        tot_ccld_uv: g(16),
        ord_remq: g(17),
        fm_ord_grp_dt: g(18),
        ord_grp_stno: g(19),
        ord_dtl_dtime: g(20),
        oprt_dtl_dtime: g(21),
        work_empl: g(22),
        ccld_dt: g(23),
        ccno: g(24),
        api_ccno: g(25),
        ccld_qty: g(26),
        fm_ccld_pric: g(27),
        crcy_cd: g(28),
        trst_fee: g(29),
        ord_mdia_online_yn: g(30),
        fm_ccld_amt: g(31),
        fuop_item_dvsn_cd: g(32),
    })
}
