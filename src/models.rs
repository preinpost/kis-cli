use serde::{Deserialize, Serialize};

// ── 토큰 ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KisAccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub access_token_token_expired: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KisApprovalKey {
    pub approval_key: String,
    pub approval_key_expired: String,
}

// ── REST API 응답 ──

/// API 공통 응답 래퍼
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub rt_cd: String,
    pub msg_cd: String,
    pub msg1: String,
    pub output: Option<serde_json::Value>,
    pub output1: Option<serde_json::Value>,
    pub output2: Option<serde_json::Value>,
}

/// 국내주식 현재가 응답
#[derive(Debug, Deserialize)]
pub struct DomesticStockPrice {
    /// 대표시장한글명
    #[serde(default)]
    pub rprs_mrkt_kor_name: String,
    /// 현재가
    #[serde(default)]
    pub stck_prpr: String,
    /// 전일대비
    #[serde(default)]
    pub prdy_vrss: String,
    /// 전일대비부호 (1:상한,2:상승,3:보합,4:하한,5:하락)
    #[serde(default)]
    pub prdy_vrss_sign: String,
    /// 전일대비율
    #[serde(default)]
    pub prdy_ctrt: String,
    /// 시가
    #[serde(default)]
    pub stck_oprc: String,
    /// 고가
    #[serde(default)]
    pub stck_hgpr: String,
    /// 저가
    #[serde(default)]
    pub stck_lwpr: String,
    /// 누적거래량
    #[serde(default)]
    pub acml_vol: String,
    /// 누적거래대금
    #[serde(default)]
    pub acml_tr_pbmn: String,
    /// HTS시가총액
    #[serde(default)]
    pub hts_avls: String,
    /// PER
    #[serde(default)]
    pub per: String,
    /// PBR
    #[serde(default)]
    pub pbr: String,
    /// 52주최고가
    #[serde(default)]
    pub w52_hgpr: String,
    /// 52주최저가
    #[serde(default)]
    pub w52_lwpr: String,
}

/// 계좌잔고 - 보유종목
#[derive(Debug, Deserialize)]
pub struct AccountHolding {
    /// 종목코드
    #[serde(default)]
    pub pdno: String,
    /// 종목명
    #[serde(default)]
    pub prdt_name: String,
    /// 보유수량
    #[serde(default)]
    pub hldg_qty: String,
    /// 매입평균가
    #[serde(default)]
    pub pchs_avg_pric: String,
    /// 매입금액
    #[serde(default)]
    pub pchs_amt: String,
    /// 현재가
    #[serde(default)]
    pub prpr: String,
    /// 평가금액
    #[serde(default)]
    pub evlu_amt: String,
    /// 평가손익금액
    #[serde(default)]
    pub evlu_pfls_amt: String,
    /// 평가손익률
    #[serde(default)]
    pub evlu_pfls_rt: String,
}

/// 계좌잔고 - 요약
#[derive(Debug, Deserialize)]
pub struct AccountSummary {
    /// 예수금총액
    #[serde(default)]
    pub dnca_tot_amt: String,
    /// 총평가금액
    #[serde(default)]
    pub tot_evlu_amt: String,
    /// 매입금액합계
    #[serde(default)]
    pub pchs_amt_smtl_amt: String,
    /// 평가금액합계
    #[serde(default)]
    pub evlu_amt_smtl_amt: String,
    /// 평가손익합계금액
    #[serde(default)]
    pub evlu_pfls_smtl_amt: String,
}
