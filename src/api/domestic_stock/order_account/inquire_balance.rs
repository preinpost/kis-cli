//! 주식잔고조회 — GET /uapi/domestic-stock/v1/trading/inquire-balance
//!
//! 스펙: .agent/specs/domestic_stock__order_account__inquire_balance.md
//!
//! output1(보유종목 Array) + output2(계좌요약 Array, 보통 단일).
//! 연속조회용 ctx 필드는 응답 최상위에 있지만 현재 `ApiResponse`가 수용 안 하므로 무시.

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/domestic-stock/v1/trading/inquire-balance";
pub const TR_ID_REAL: &str = "TTTC8434R";
pub const TR_ID_MOCK: &str = "VTTC8434R";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub cano: String,
    pub acnt_prdt_cd: String,
    /// N 기본, Y 시간외단일가, X NXT 정규장
    pub afhr_flpr_yn: String,
    /// 공란(Default)
    pub ofl_yn: String,
    /// 01 대출일별
    pub inqr_dvsn: String,
    /// 01 기본값
    pub unpr_dvsn: String,
    pub fund_sttl_icld_yn: String,
    pub fncg_amt_auto_rdpt_yn: String,
    /// 00 전일매매포함 / 01 전일매매미포함
    pub prcs_dvsn: String,
    pub ctx_area_fk100: String,
    pub ctx_area_nk100: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Holding {
    #[serde(default)]
    pub pdno: String,
    #[serde(default)]
    pub prdt_name: String,
    #[serde(default)]
    pub trad_dvsn_name: String,
    #[serde(default)]
    pub bfdy_buy_qty: String,
    #[serde(default)]
    pub bfdy_sll_qty: String,
    #[serde(default)]
    pub thdt_buyqty: String,
    #[serde(default)]
    pub thdt_sll_qty: String,
    #[serde(default)]
    pub hldg_qty: String,
    #[serde(default)]
    pub ord_psbl_qty: String,
    #[serde(default)]
    pub pchs_avg_pric: String,
    #[serde(default)]
    pub pchs_amt: String,
    #[serde(default)]
    pub prpr: String,
    #[serde(default)]
    pub evlu_amt: String,
    #[serde(default)]
    pub evlu_pfls_amt: String,
    #[serde(default)]
    pub evlu_pfls_rt: String,
    #[serde(default)]
    pub evlu_erng_rt: String,
    #[serde(default)]
    pub loan_dt: String,
    #[serde(default)]
    pub loan_amt: String,
    #[serde(default)]
    pub stln_slng_chgs: String,
    #[serde(default)]
    pub expd_dt: String,
    #[serde(default)]
    pub fltt_rt: String,
    #[serde(default)]
    pub bfdy_cprs_icdc: String,
    #[serde(default)]
    pub item_mgna_rt_name: String,
    #[serde(default)]
    pub grta_rt_name: String,
    #[serde(default)]
    pub sbst_pric: String,
    #[serde(default)]
    pub stck_loan_unpr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    #[serde(default)]
    pub dnca_tot_amt: String,
    #[serde(default)]
    pub nxdy_excc_amt: String,
    #[serde(default)]
    pub prvs_rcdl_excc_amt: String,
    #[serde(default)]
    pub cma_evlu_amt: String,
    #[serde(default)]
    pub bfdy_buy_amt: String,
    #[serde(default)]
    pub thdt_buy_amt: String,
    #[serde(default)]
    pub nxdy_auto_rdpt_amt: String,
    #[serde(default)]
    pub bfdy_sll_amt: String,
    #[serde(default)]
    pub thdt_sll_amt: String,
    #[serde(default)]
    pub d2_auto_rdpt_amt: String,
    #[serde(default)]
    pub bfdy_tlex_amt: String,
    #[serde(default)]
    pub thdt_tlex_amt: String,
    #[serde(default)]
    pub tot_loan_amt: String,
    #[serde(default)]
    pub scts_evlu_amt: String,
    #[serde(default)]
    pub tot_evlu_amt: String,
    #[serde(default)]
    pub nass_amt: String,
    #[serde(default)]
    pub fncg_gld_auto_rdpt_yn: String,
    #[serde(default)]
    pub pchs_amt_smtl_amt: String,
    #[serde(default)]
    pub evlu_amt_smtl_amt: String,
    #[serde(default)]
    pub evlu_pfls_smtl_amt: String,
    #[serde(default)]
    pub tot_stln_slng_chgs: String,
    #[serde(default)]
    pub bfdy_tot_asst_evlu_amt: String,
    #[serde(default)]
    pub asst_icdc_amt: String,
    #[serde(default)]
    pub asst_icdc_erng_rt: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub holdings: Vec<Holding>,
    pub summary: Option<Summary>,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Response> {
    let tr_id = if client.is_mock() { TR_ID_MOCK } else { TR_ID_REAL };
    let params = [
        ("CANO", req.cano.as_str()),
        ("ACNT_PRDT_CD", req.acnt_prdt_cd.as_str()),
        ("AFHR_FLPR_YN", req.afhr_flpr_yn.as_str()),
        ("OFL_YN", req.ofl_yn.as_str()),
        ("INQR_DVSN", req.inqr_dvsn.as_str()),
        ("UNPR_DVSN", req.unpr_dvsn.as_str()),
        ("FUND_STTL_ICLD_YN", req.fund_sttl_icld_yn.as_str()),
        ("FNCG_AMT_AUTO_RDPT_YN", req.fncg_amt_auto_rdpt_yn.as_str()),
        ("PRCS_DVSN", req.prcs_dvsn.as_str()),
        ("CTX_AREA_FK100", req.ctx_area_fk100.as_str()),
        ("CTX_AREA_NK100", req.ctx_area_nk100.as_str()),
    ];
    let resp = client.get(ENDPOINT, tr_id, &params).await?;
    let holdings: Vec<Holding> = resp
        .output1
        .map(serde_json::from_value)
        .transpose()?
        .unwrap_or_default();
    let summary = resp.output2.and_then(|v| {
        serde_json::from_value::<Vec<Summary>>(v)
            .ok()
            .and_then(|mut arr| arr.pop())
    });
    Ok(Response { holdings, summary })
}
