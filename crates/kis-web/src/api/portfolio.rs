//! 포트폴리오 조회 — 국내 + 해외 잔고를 통일된 DTO 로 합산.

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

use kis_core::api::domestic_stock::order_account::inquire_balance as dome_bal;
use kis_core::api::overseas_stock::order_account::inquire_balance as os_bal;
use kis_core::client::KisClient;

use crate::auth::SessionAuth;
use crate::state::AppState;

use super::ApiTag;

/// 통일된 보유 종목.
#[derive(Object)]
struct Holding {
    market: String,   // "domestic" | "overseas"
    currency: String, // "KRW" | "USD"
    symbol: String,
    name: String,
    quantity: String,
    avg_price: String,
    current_price: String,
    eval_amount: String,
    pnl_amount: String,
    pnl_rate: String,
}

/// 시장별 요약.
#[derive(Object)]
struct Summary {
    currency: String,
    deposit: String,        // 예수금 (국내만)
    total_eval: String,     // 총평가금액
    total_purchase: String, // 총매입금액
    total_pnl: String,      // 총평가손익
    total_pnl_rate: String, // 총손익률(%)
}

#[derive(Object)]
struct Portfolio {
    is_mock: bool,
    domestic: Option<Summary>,
    overseas: Option<Summary>,
    holdings: Vec<Holding>,
}

pub struct PortfolioApi;

#[OpenApi(prefix_path = "/portfolio", tag = "ApiTag::Portfolio")]
impl PortfolioApi {
    /// 내 잔고 — 국내·해외 보유종목 + 시장별 요약.
    #[oai(path = "/balance", method = "get")]
    async fn balance(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
    ) -> Result<Json<Portfolio>> {
        let st = state.0;
        let client = st
            .clients
            .get(&st.db, &st.config.master_key, &auth.0.id)
            .await
            .map_err(internal)?
            .ok_or_else(|| {
                Error::from_string(
                    "KIS 자격증명을 먼저 등록하세요",
                    StatusCode::CONFLICT,
                )
            })?;

        let mut holdings = Vec::new();

        // ── 국내 ──
        let dome = fetch_domestic(&client).await.map_err(upstream)?;
        for h in &dome.holdings {
            if is_zero(&h.hldg_qty) {
                continue;
            }
            holdings.push(Holding {
                market: "domestic".into(),
                currency: "KRW".into(),
                symbol: h.pdno.clone(),
                name: h.prdt_name.clone(),
                quantity: h.hldg_qty.clone(),
                avg_price: h.pchs_avg_pric.clone(),
                current_price: h.prpr.clone(),
                eval_amount: h.evlu_amt.clone(),
                pnl_amount: h.evlu_pfls_amt.clone(),
                pnl_rate: h.evlu_pfls_rt.clone(),
            });
        }
        let domestic_summary = dome.summary.as_ref().map(|s| Summary {
            currency: "KRW".into(),
            deposit: s.dnca_tot_amt.clone(),
            total_eval: s.tot_evlu_amt.clone(),
            total_purchase: s.pchs_amt_smtl_amt.clone(),
            total_pnl: s.evlu_pfls_smtl_amt.clone(),
            total_pnl_rate: pct(&s.evlu_pfls_smtl_amt, &s.pchs_amt_smtl_amt),
        });

        // ── 해외 (NASD/NYSE/AMEX 순회, best-effort) ──
        let mut os_eval = 0.0;
        let mut os_purchase = 0.0;
        let mut os_pnl = 0.0;
        let mut os_any = false;
        for excd in ["NASD", "NYSE", "AMEX"] {
            let Ok(os) = fetch_overseas(&client, excd).await else {
                continue; // 모의계좌·미지원 거래소 등은 건너뜀
            };
            os_any = true;
            for h in &os.holdings {
                if is_zero(&h.ovrs_cblc_qty) {
                    continue;
                }
                let eval = num(&h.ovrs_stck_evlu_amt);
                let purchase = num(&h.frcr_pchs_amt1);
                os_eval += eval;
                os_purchase += purchase;
                os_pnl += eval - purchase;
                holdings.push(Holding {
                    market: "overseas".into(),
                    currency: "USD".into(),
                    symbol: h.ovrs_pdno.clone(),
                    name: h.ovrs_item_name.clone(),
                    quantity: h.ovrs_cblc_qty.clone(),
                    avg_price: h.pchs_avg_pric.clone(),
                    current_price: h.now_pric2.clone(),
                    eval_amount: h.ovrs_stck_evlu_amt.clone(),
                    pnl_amount: fmt2(eval - purchase),
                    pnl_rate: h.evlu_pfls_rt.clone(),
                });
            }
        }
        let overseas_summary = if os_any && os_purchase > 0.0 {
            Some(Summary {
                currency: "USD".into(),
                deposit: String::new(),
                total_eval: fmt2(os_eval),
                total_purchase: fmt2(os_purchase),
                total_pnl: fmt2(os_pnl),
                total_pnl_rate: if os_purchase > 0.0 {
                    fmt2(os_pnl / os_purchase * 100.0)
                } else {
                    String::new()
                },
            })
        } else {
            None
        };

        Ok(Json(Portfolio {
            is_mock: client.is_mock(),
            domestic: domestic_summary,
            overseas: overseas_summary,
            holdings,
        }))
    }
}

async fn fetch_domestic(client: &KisClient) -> anyhow::Result<dome_bal::Response> {
    let req = dome_bal::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        afhr_flpr_yn: "N".into(),
        ofl_yn: "".into(),
        inqr_dvsn: "02".into(),
        unpr_dvsn: "01".into(),
        fund_sttl_icld_yn: "N".into(),
        fncg_amt_auto_rdpt_yn: "N".into(),
        prcs_dvsn: "01".into(),
        ctx_area_fk100: "".into(),
        ctx_area_nk100: "".into(),
    };
    dome_bal::call(client, &req).await
}

async fn fetch_overseas(client: &KisClient, excd: &str) -> anyhow::Result<os_bal::Response> {
    let req = os_bal::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: excd.into(),
        tr_crcy_cd: "USD".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    os_bal::call(client, &req).await
}

fn is_zero(s: &str) -> bool {
    num(s) == 0.0
}

fn num(s: &str) -> f64 {
    s.trim().parse().unwrap_or(0.0)
}

fn fmt2(v: f64) -> String {
    format!("{v:.2}")
}

/// pnl / purchase * 100 → 퍼센트 문자열.
fn pct(pnl: &str, purchase: &str) -> String {
    let p = num(purchase);
    if p == 0.0 {
        return String::new();
    }
    fmt2(num(pnl) / p * 100.0)
}

fn internal<E: std::fmt::Display>(e: E) -> Error {
    tracing::error!("portfolio internal error: {e}");
    Error::from_string("서버 오류", StatusCode::INTERNAL_SERVER_ERROR)
}

/// KIS 상류 API 오류 → 502 + 메시지(자격증명 오류·장 외 시간 등 사용자에게 노출).
fn upstream(e: impl std::fmt::Display) -> Error {
    Error::from_string(format!("KIS 조회 실패: {e}"), StatusCode::BAD_GATEWAY)
}
