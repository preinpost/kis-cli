//! 포트폴리오 조회 — 국내 + 해외 잔고를 통일된 DTO 로 합산 + 통합 포지션(KIS+수동) 관리.

use std::collections::HashMap;

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

use kis_core::api::domestic_stock::order_account::inquire_balance as dome_bal;
use kis_core::api::overseas_stock::order_account::inquire_balance as os_bal;
use kis_core::client::KisClient;

use crate::auth::SessionAuth;
use crate::positions::{positions_from_trades, Position, TradeRow};
use crate::state::AppState;

use super::{quotes, ApiTag};

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

/// 통합 포지션 (KIS 자동 + 수동 매매일지 파생) + 관리 메타.
#[derive(Object)]
struct PositionDto {
    source: String, // "KIS" | broker(수동)
    symbol: String,
    name: String,
    market: String,
    currency: String,
    quantity: f64,
    avg_cost: f64,
    current_price: Option<f64>,
    eval_amount: f64,
    unrealized_pnl: Option<f64>,
    realized_pnl: f64,    // 수동(매매일지)만
    weight: f64,          // 같은 통화 내 비중(%)
    memo: Option<String>,
    target_price: Option<f64>,
    stop_price: Option<f64>,
    target_weight: Option<f64>,
}

#[derive(Object)]
struct Positions {
    has_kis: bool,
    positions: Vec<PositionDto>,
}

/// 종목 관리 메타 입력.
#[derive(Object)]
struct MetaInput {
    memo: Option<String>,
    target_price: Option<f64>,
    stop_price: Option<f64>,
    target_weight: Option<f64>,
}

#[derive(sqlx::FromRow)]
struct MetaRow {
    symbol: String,
    memo: Option<String>,
    target_price: Option<f64>,
    stop_price: Option<f64>,
    target_weight: Option<f64>,
}

#[derive(sqlx::FromRow)]
struct TradeCalcRow {
    traded_at: String,
    symbol: String,
    name: Option<String>,
    market: Option<String>,
    broker: Option<String>,
    side: String,
    quantity: f64,
    price: f64,
    fee: f64,
    currency: String,
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

    /// 통합 포지션 — KIS 자동 보유 + 수동(매매일지 파생). 종목별 메타·비중·실현손익 포함.
    /// KIS 자격증명이 없어도 수동 포지션은 동작(현재가는 미표시).
    #[oai(path = "/positions", method = "get")]
    async fn positions(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
    ) -> Result<Json<Positions>> {
        let st = state.0;
        let uid = &auth.0.id;

        // 1) 메타
        let metas: Vec<MetaRow> = sqlx::query_as(
            "SELECT symbol, memo, target_price, stop_price, target_weight FROM holding_meta WHERE user_id = ?",
        )
        .bind(uid)
        .fetch_all(&st.db)
        .await
        .map_err(internal)?;
        let meta_map: HashMap<String, MetaRow> =
            metas.into_iter().map(|m| (m.symbol.clone(), m)).collect();

        // 2) 수동 포지션 (매매일지 파생, 잔여수량>0)
        let trades: Vec<TradeCalcRow> = sqlx::query_as(
            "SELECT traded_at, symbol, name, market, broker, side, quantity, price, fee, currency \
             FROM trades WHERE user_id = ?",
        )
        .bind(uid)
        .fetch_all(&st.db)
        .await
        .map_err(internal)?;
        let calc: Vec<TradeRow> = trades.into_iter().map(calc_row).collect();
        let manual: Vec<Position> = positions_from_trades(&calc)
            .into_iter()
            .filter(|p| p.quantity > 1e-9)
            .collect();

        // 3) KIS 클라이언트 (선택)
        let client = st
            .clients
            .get(&st.db, &st.config.master_key, uid)
            .await
            .ok()
            .flatten();
        let has_kis = client.is_some();

        let mut out: Vec<PositionDto> = Vec::new();

        // 4) KIS 자동 보유
        if let Some(ref client) = client {
            if let Ok(dome) = fetch_domestic(client).await {
                for h in &dome.holdings {
                    if is_zero(&h.hldg_qty) {
                        continue;
                    }
                    let price = num(&h.prpr);
                    out.push(merge_meta(
                        PositionDto {
                            source: "KIS".into(),
                            symbol: h.pdno.clone(),
                            name: h.prdt_name.clone(),
                            market: "domestic".into(),
                            currency: "KRW".into(),
                            quantity: num(&h.hldg_qty),
                            avg_cost: num(&h.pchs_avg_pric),
                            current_price: Some(price),
                            eval_amount: num(&h.evlu_amt),
                            unrealized_pnl: Some(num(&h.evlu_pfls_amt)),
                            realized_pnl: 0.0,
                            weight: 0.0,
                            memo: None, target_price: None, stop_price: None, target_weight: None,
                        },
                        &meta_map,
                    ));
                }
            }
            for excd in ["NASD", "NYSE", "AMEX"] {
                let Ok(os) = fetch_overseas(client, excd).await else { continue };
                for h in &os.holdings {
                    if is_zero(&h.ovrs_cblc_qty) {
                        continue;
                    }
                    let eval = num(&h.ovrs_stck_evlu_amt);
                    let purchase = num(&h.frcr_pchs_amt1);
                    out.push(merge_meta(
                        PositionDto {
                            source: "KIS".into(),
                            symbol: h.ovrs_pdno.clone(),
                            name: h.ovrs_item_name.clone(),
                            market: "overseas".into(),
                            currency: "USD".into(),
                            quantity: num(&h.ovrs_cblc_qty),
                            avg_cost: num(&h.pchs_avg_pric),
                            current_price: Some(num(&h.now_pric2)),
                            eval_amount: eval,
                            unrealized_pnl: Some(eval - purchase),
                            realized_pnl: 0.0,
                            weight: 0.0,
                            memo: None, target_price: None, stop_price: None, target_weight: None,
                        },
                        &meta_map,
                    ));
                }
            }
        }

        // 5) 수동 포지션 (+ 현재가 best-effort)
        for p in &manual {
            let cur = if let Some(ref client) = client {
                quotes::current_price(client, st.config.symbols_db_path.clone(), &p.symbol)
                    .await
                    .map(|(price, _)| price)
            } else {
                None
            };
            let eval = p.quantity * cur.unwrap_or(p.avg_cost);
            let unreal = cur.map(|c| (c - p.avg_cost) * p.quantity);
            out.push(merge_meta(
                PositionDto {
                    source: p.broker.clone().unwrap_or_else(|| "수동".into()),
                    symbol: p.symbol.clone(),
                    name: p.name.clone().unwrap_or_else(|| p.symbol.clone()),
                    market: p.market.clone().unwrap_or_else(|| "other".into()),
                    currency: p.currency.clone(),
                    quantity: p.quantity,
                    avg_cost: p.avg_cost,
                    current_price: cur,
                    eval_amount: eval,
                    unrealized_pnl: unreal,
                    realized_pnl: p.realized_pnl,
                    weight: 0.0,
                    memo: None, target_price: None, stop_price: None, target_weight: None,
                },
                &meta_map,
            ));
        }

        // 6) 통화별 비중
        let mut total: HashMap<String, f64> = HashMap::new();
        for p in &out {
            *total.entry(p.currency.clone()).or_insert(0.0) += p.eval_amount;
        }
        for p in &mut out {
            let t = total.get(&p.currency).copied().unwrap_or(0.0);
            p.weight = if t > 0.0 { p.eval_amount / t * 100.0 } else { 0.0 };
        }

        Ok(Json(Positions { has_kis, positions: out }))
    }

    /// 종목 관리 메타(메모·목표가·손절가·목표비중) 저장.
    #[oai(path = "/meta/:symbol", method = "put")]
    async fn put_meta(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        symbol: Path<String>,
        body: Json<MetaInput>,
    ) -> Result<Json<super::OkDto>> {
        let st = state.0;
        let m = body.0;
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO holding_meta (user_id, symbol, memo, target_price, stop_price, target_weight, updated_at) \
             VALUES (?,?,?,?,?,?,?) \
             ON CONFLICT(user_id, symbol) DO UPDATE SET \
               memo=excluded.memo, target_price=excluded.target_price, stop_price=excluded.stop_price, \
               target_weight=excluded.target_weight, updated_at=excluded.updated_at",
        )
        .bind(&auth.0.id)
        .bind(symbol.0.trim())
        .bind(m.memo.as_deref())
        .bind(m.target_price)
        .bind(m.stop_price)
        .bind(m.target_weight)
        .bind(&now)
        .execute(&st.db)
        .await
        .map_err(internal)?;
        Ok(Json(super::OkDto { ok: true }))
    }
}

fn merge_meta(mut p: PositionDto, metas: &HashMap<String, MetaRow>) -> PositionDto {
    if let Some(m) = metas.get(&p.symbol) {
        p.memo = m.memo.clone();
        p.target_price = m.target_price;
        p.stop_price = m.stop_price;
        p.target_weight = m.target_weight;
    }
    p
}

fn calc_row(r: TradeCalcRow) -> TradeRow {
    TradeRow {
        traded_at: r.traded_at,
        symbol: r.symbol,
        name: r.name,
        market: r.market,
        broker: r.broker,
        side: r.side,
        quantity: r.quantity,
        price: r.price,
        fee: r.fee,
        currency: r.currency,
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
