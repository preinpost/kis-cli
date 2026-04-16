//! ELW 신규상장종목 — GET /uapi/elw/v1/quotations/newly-listed
//!
//! 스펙: .agent/specs/domestic_stock__elw__newly_listed.md
//!
//! 모의투자 미지원.

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};

use crate::client::KisClient;

pub const ENDPOINT: &str = "/uapi/elw/v1/quotations/newly-listed";
pub const TR_ID: &str = "FHKEW154800C0";

#[derive(Debug, Clone, Serialize)]
pub struct Request {
    /// W
    pub fid_cond_mrkt_div_code: String,
    /// Unique key 11548
    pub fid_cond_scr_div_code: String,
    /// 02 전체, 00 콜, 01 풋
    pub fid_div_cls_code: String,
    /// 000000 전체 또는 기초자산 종목코드
    pub fid_unas_input_iscd: String,
    /// 00003/00017/00005 등 발행사
    pub fid_input_iscd_2: String,
    /// YYYYMMDD
    pub fid_input_date_1: String,
    /// 0 전체, 1 일반, 2 조기종료
    pub fid_blnc_cls_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub stck_lstn_date: String,
    #[serde(default)]
    pub elw_kor_isnm: String,
    #[serde(default)]
    pub elw_shrn_iscd: String,
    #[serde(default)]
    pub unas_isnm: String,
    #[serde(default)]
    pub pblc_co_name: String,
    #[serde(default)]
    pub lstn_stcn: String,
    #[serde(default)]
    pub acpr: String,
    #[serde(default)]
    pub stck_last_tr_date: String,
    #[serde(default)]
    pub elw_ko_barrier: String,
}

pub async fn call(client: &KisClient, req: &Request) -> Result<Vec<Row>> {
    if client.is_mock() {
        bail!("ELW 신규상장종목은 모의투자 미지원 API입니다");
    }
    let params = [
        ("FID_COND_MRKT_DIV_CODE", req.fid_cond_mrkt_div_code.as_str()),
        ("FID_COND_SCR_DIV_CODE", req.fid_cond_scr_div_code.as_str()),
        ("FID_DIV_CLS_CODE", req.fid_div_cls_code.as_str()),
        ("FID_UNAS_INPUT_ISCD", req.fid_unas_input_iscd.as_str()),
        ("FID_INPUT_ISCD_2", req.fid_input_iscd_2.as_str()),
        ("FID_INPUT_DATE_1", req.fid_input_date_1.as_str()),
        ("FID_BLNC_CLS_CODE", req.fid_blnc_cls_code.as_str()),
    ];
    let resp = client.get(ENDPOINT, TR_ID, &params).await?;
    let output = resp.output.ok_or_else(|| anyhow!("응답에 output 없음"))?;
    let rows: Vec<Row> = serde_json::from_value(output)?;
    Ok(rows)
}
