//! `daytrade` 공통 엔진 — `paper` / `run` 공유.
//!
//! - 매 봉 마감 직후 tick: 분봉 fetch → 신호 계산 → 체결 → SQLite 기록
//! - **청산 우선순위**: EOD 강제 청산 > 손절(SL) > 익절(TP) > 전략 신호
//! - **장 마감 10분 전 보유 포지션 강제 청산** (오버나이트 금지)
//! - 세션 단위로 `session_id` 부여, 프로세스 종료(Ctrl+C) 시 일일 리포트 출력
//! - 체결은 [`Executor`] trait로 추상화 — `PaperExecutor` (가상) / `LiveExecutor` (실주문).

use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use chrono_tz::Tz;
use tokio_util::sync::CancellationToken;

use crate::analysis::indicators;
use crate::client::KisClient;
use crate::commands::backtest::{self, Params, StrategyKind};
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::config;
use crate::symbols::{Market as SymMarket, ResolveMode, ResolvedSymbol};

use super::fetch;
use super::period::Period;
use super::session::{self, Market};
use super::storage::{Mode, Side, Storage, TradeInsert};

/// 복합 전략 결합 방식.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
    /// 모든 child가 +1 → +1 (보수적). 어느 하나가 ≤0 이면 청산.
    And,
    /// 하나라도 +1 → +1. 모두 ≤0 이면 청산.
    Or,
}

/// composite child 의 신호 파라미터 — `Params` 의 strategy-specific 필드만 추림.
#[derive(Debug, Clone)]
pub struct CompositeChild {
    pub strategy: StrategyKind,
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct CompositeConfig {
    pub combinator: Combinator,
    pub children: Vec<CompositeChild>,
}

/// 엔진 공통 설정. `paper::Config` / `run::Config` 에서 공통 필드만 추출.
/// 체결 방식별 파라미터(slippage/order_type 등)는 `Executor` 구현체에 둔다.
pub struct EngineConfig {
    pub symbol: String,
    /// 사전 해석된 심볼. 있으면 `symbol`/`pick` 무시하고 그대로 사용 — 데몬 경로에서 재해석 회피.
    pub pre_resolved: Option<ResolvedSymbol>,
    pub strategy: StrategyKind,
    /// `strategy == Composite` 일 때만 사용. None 이면 단일 전략으로 동작.
    pub composite: Option<CompositeConfig>,
    pub period: Period,
    pub usa: bool,
    pub pick: Option<usize>,
    pub qty: u64,
    pub budget: f64,
    pub fee_bps: f64,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,
    /// ATR 배수 손절. `Some(2.0)` = 진입가 - ATR(period) × 2.0 하회 시 청산.
    /// `stop_loss_pct` 와 둘 다 지정되면 더 타이트한(높은) 손절선 사용.
    pub stop_loss_atr: Option<f64>,
    /// ATR 배수 익절. `Some(3.0)` = 진입가 + ATR × 3.0 도달 시 청산.
    pub take_profit_atr: Option<f64>,
    /// ATR 계산 봉 수 (기본 14).
    pub atr_period: usize,
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
}

/// 체결 결과.
pub struct Fill {
    pub qty: u64,
    pub price: f64,
}

/// 체결 추상화. paper는 slippage 기반 가상 체결, live는 실제 주문 발주.
pub trait Executor: Send + Sync {
    fn mode(&self) -> Mode;
    /// 시작 로그 접두사 — 예: `"daytrade paper 시작 (실전 서버 기반 모의테스트)"`.
    fn start_prefix(&self) -> &'static str;
    /// 시작 로그에 붙일 실행기별 부가 정보 — 예: `" · slip=5.0bps"`.
    fn extra_start_info(&self) -> String;
    #[allow(async_fn_in_trait)]
    async fn buy(&self, code: &str, market: Market, qty: u64, ref_price: f64) -> Result<Fill>;
    #[allow(async_fn_in_trait)]
    async fn sell(&self, code: &str, market: Market, qty: u64, ref_price: f64) -> Result<Fill>;
    /// 실제 계좌 잔고로부터 포지션 재구성 (live 전용, paper는 `None`).
    ///
    /// 호출 시점:
    /// - 세션 시작 시 (이전 보유 로드)
    /// - 매수/매도 체결 직후 (부분체결·수수료 반영)
    ///
    /// `None` 반환 시 엔진 자체 계산을 그대로 사용한다.
    #[allow(async_fn_in_trait)]
    async fn sync_position(&self, code: &str, market: Market) -> Result<Option<Position>> {
        let _ = (code, market);
        Ok(None)
    }
}

/// 현재 세션의 포지션.
#[derive(Debug, Clone)]
pub struct Position {
    pub qty: u64,
    pub avg_price: f64,
    pub entry_time: DateTime<Tz>,
}

pub async fn run<E: Executor>(
    client: Arc<KisClient>,
    cfg: EngineConfig,
    executor: E,
    cancel: CancellationToken,
) -> Result<()> {
    let market = if cfg.usa { Market::Usa } else { Market::Krx };
    let sym = if let Some(pre) = cfg.pre_resolved.clone() {
        pre
    } else {
        let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
        resolve_symbol(&cfg.symbol, mode, cfg.pick)?
    };
    let name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };

    let storage = Storage::open(&config::daytrade_db_path()?)?;
    let mut session_id = new_session_id(&sym.code, market);

    let sl_label = format_sl_tp_label(cfg.stop_loss_pct, cfg.stop_loss_atr, cfg.atr_period);
    let tp_label = format_sl_tp_label(cfg.take_profit_pct, cfg.take_profit_atr, cfg.atr_period);
    log_info(&format!(
        "{}: [{}] {} ({}) · {} · qty={} · budget={} · fee={:.1}bps{} · SL={} · TP={} · session={}",
        executor.start_prefix(),
        sym.code, name, market.label(), cfg.period.label(),
        cfg.qty, format_price(cfg.budget, cfg.usa),
        cfg.fee_bps, executor.extra_start_info(),
        sl_label, tp_label, session_id
    ));

    let cfg = Arc::new(cfg);
    let code = sym.code.clone();
    let sym_market = sym.market;

    // 세션 시작 — live는 실제 계좌 잔고에서 기존 보유를 로드, paper는 None.
    let mut position: Option<Position> = executor.sync_position(&code, market).await?;
    if let Some(p) = position.as_ref() {
        log_info(&format!(
            "  초기 보유 로드: {}주 @ {} (계좌 기준)",
            p.qty, format_price(p.avg_price, cfg.usa)
        ));
    }
    // EOD 청산 + 리포트가 이미 한 번 출력됐는지. true면 Ctrl+C 경로에서 중복 출력을 피하고,
    // 다음 tick에서 장이 다시 열렸을 때 session_id 롤오버 트리거로도 사용한다.
    let mut session_reported = false;

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                if !session_reported {
                    log_info("종료 신호 수신 — 일일 리포트 출력");
                    print_report(&storage, &session_id, &cfg, market);
                } else {
                    log_info("종료 신호 수신");
                }
                return Ok(());
            }
            _ = sleep_until_next_tick(market, cfg.period, &code) => {}
        }

        if let Err(e) = tick(
            &client, &cfg, &executor, &code, &name, market, sym_market,
            &storage, &mut session_id, &mut session_reported, &mut position,
        ).await {
            log_error(&format!("tick 실패: {e}"));
        }
    }
}

async fn sleep_until_next_tick(market: Market, period: Period, code: &str) {
    let now = session::now_kst();
    if !session::is_in_session(market, now) {
        let wait = session::time_until_open(market, now);
        let mins = wait.num_minutes().max(1);
        log_info(&format!("세션 밖 — 다음 개장까지 약 {}분 대기", mins));
        let chunk = if mins > 30 { 30 } else { mins };
        tokio::time::sleep(std::time::Duration::from_secs((chunk * 60) as u64)).await;
        tokio::time::sleep(std::time::Duration::from_millis(code_jitter_ms(code))).await;
        return;
    }
    let now = session::now_kst();
    let next = session::next_bar_boundary_kst(period, now, 10);
    let wait = (next - now).to_std().unwrap_or(std::time::Duration::from_secs(60));
    tokio::time::sleep(wait).await;
    tokio::time::sleep(std::time::Duration::from_millis(code_jitter_ms(code))).await;
}

/// 여러 daytrade 서비스가 같은 tick 경계에서 동시에 깨어나 KIS TPS를 초과하는 걸 막기 위한
/// 종목코드 기반 결정적 지터 (0~2999ms).
fn code_jitter_ms(code: &str) -> u64 {
    code.bytes().map(|b| b as u64).sum::<u64>() % 3000
}

#[allow(clippy::too_many_arguments)]
async fn tick<E: Executor>(
    client: &KisClient,
    cfg: &EngineConfig,
    executor: &E,
    code: &str,
    name: &str,
    market: Market,
    sym_market: SymMarket,
    storage: &Storage,
    session_id: &mut String,
    session_reported: &mut bool,
    position: &mut Option<Position>,
) -> Result<()> {
    // 이전 세션이 EOD로 끝나 리포트가 출력됐는데 장이 다시 열렸다면 — 새 세션 롤오버.
    let now = session::now_kst();
    if *session_reported && session::is_in_session(market, now) {
        *session_id = new_session_id(code, market);
        *session_reported = false;
        *position = None; // EOD로 이미 청산됐어야 하지만 안전상 리셋
        log_info(&format!("── 새 세션 시작 session={} ──", session_id));
    }

    log_info(&format!("── tick [{}] {} ──", code, name));

    let series = if cfg.usa {
        fetch::fetch_overseas(client, code, sym_market, cfg.period).await?
    } else {
        fetch::fetch_domestic(client, code, cfg.period).await?
    };
    if series.closes.len() < 30 {
        return Err(anyhow!("데이터 부족 ({}봉)", series.closes.len()));
    }
    let signal = compute_signal(cfg, &series);
    let last_price = series.closes.last().copied().unwrap_or(f64::NAN);
    let last_ts = series.dates.last().cloned().unwrap_or_default();
    log_info(&format!(
        "  최신봉 {} / 종가 {} / 신호 {}",
        format_ts(&last_ts),
        format_price(last_price, cfg.usa),
        signal_label(signal)
    ));

    if let Some(pos) = position {
        let used = pos.qty as f64 * pos.avg_price;
        log_info(&format!(
            "  보유: {}주 @ {} (진입 {}) · used {} / budget {}",
            pos.qty,
            format_price(pos.avg_price, cfg.usa),
            pos.entry_time.format("%m-%d %H:%M"),
            format_price(used, cfg.usa),
            format_price(cfg.budget, cfg.usa),
        ));
    } else {
        log_info("  보유 없음");
    }

    let force_exit = session::should_force_exit(market, now, 10);

    // 1) EOD 강제 청산 (우선순위 최상) ---------------------------------------
    if force_exit {
        if let Some(pos) = position.take() {
            execute_exit(
                storage, executor, session_id, code, market, cfg, pos,
                last_price, now, "EOD 강제 청산",
            ).await?;
        } else {
            log_info("  ⚠ 장 마감 10분 전 — 신규 진입 차단 (EOD)");
        }
        if !*session_reported {
            print_report(storage, session_id, cfg, market);
            *session_reported = true;
        }
        return Ok(());
    }

    // 2) 손절(SL) / 익절(TP) — 신호보다 우선 --------------------------------
    if let Some(pos) = position.as_ref() {
        if pos.avg_price > 0.0 {
            // 현재 ATR (마지막 봉) — Wilder smoothing
            let cur_atr = indicators::atr(&series.high, &series.low, &series.closes, cfg.atr_period)
                .last().copied().unwrap_or(f64::NAN);

            // SL 후보: pct, atr 중 더 타이트한(=가격이 더 높은 = 손실폭이 작은) 쪽
            let sl_price_pct = cfg.stop_loss_pct.map(|p| pos.avg_price * (1.0 - p / 100.0));
            let sl_price_atr = cfg.stop_loss_atr.and_then(|m| {
                if cur_atr.is_finite() { Some(pos.avg_price - cur_atr * m) } else { None }
            });
            let sl_price = match (sl_price_pct, sl_price_atr) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (a, b) => a.or(b),
            };
            let sl_hit = sl_price.is_some_and(|p| last_price <= p);

            // TP 후보: pct, atr 중 더 타이트한(=가격이 더 낮은 = 빨리 익절) 쪽
            let tp_price_pct = cfg.take_profit_pct.map(|p| pos.avg_price * (1.0 + p / 100.0));
            let tp_price_atr = cfg.take_profit_atr.and_then(|m| {
                if cur_atr.is_finite() { Some(pos.avg_price + cur_atr * m) } else { None }
            });
            let tp_price = match (tp_price_pct, tp_price_atr) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, b) => a.or(b),
            };
            let tp_hit = tp_price.is_some_and(|p| last_price >= p);

            if sl_hit || tp_hit {
                let change_pct = (last_price / pos.avg_price - 1.0) * 100.0;
                let reason = if sl_hit {
                    format!("손절 ({:+.2}%, trigger {})", change_pct, format_price(sl_price.unwrap(), cfg.usa))
                } else {
                    format!("익절 ({:+.2}%, trigger {})", change_pct, format_price(tp_price.unwrap(), cfg.usa))
                };
                let pos = position.take().unwrap();
                execute_exit(
                    storage, executor, session_id, code, market, cfg, pos,
                    last_price, now, &reason,
                ).await?;
                return Ok(());
            }
        }
    }

    // 3) 신호 기반 ---------------------------------------------------------
    match (signal, position.as_ref()) {
        (s, None) if s > 0 => {
            if let Some(new_pos) = execute_entry(
                storage, executor, session_id, code, market, cfg, None, last_price, now,
            ).await? {
                *position = Some(new_pos);
            }
        }
        (s, Some(_)) if s > 0 => {
            let current = position.as_ref().unwrap().clone();
            if let Some(new_pos) = execute_entry(
                storage, executor, session_id, code, market, cfg, Some(&current), last_price, now,
            ).await? {
                *position = Some(new_pos);
            }
        }
        (s, Some(_)) if s <= 0 => {
            let pos = position.take().unwrap();
            execute_exit(
                storage, executor, session_id, code, market, cfg, pos,
                last_price, now, "신호 청산",
            ).await?;
        }
        _ => log_info("  → 변화 없음"),
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn execute_entry<E: Executor>(
    storage: &Storage,
    executor: &E,
    session_id: &str,
    code: &str,
    market: Market,
    cfg: &EngineConfig,
    current: Option<&Position>,
    base_price: f64,
    now: DateTime<Tz>,
) -> Result<Option<Position>> {
    // 예산 체크는 체결 전에 — 실주문 헛발사 방지
    let current_cost = current.map(|p| p.qty as f64 * p.avg_price).unwrap_or(0.0);
    let estimated_cost = cfg.qty as f64 * base_price;
    if current_cost + estimated_cost > cfg.budget {
        let remain = (cfg.budget - current_cost).max(0.0);
        log_info(&format!(
            "  → 진입 보류: 예산 초과 (필요 ~{}, 남은 예산 {})",
            format_price(estimated_cost, cfg.usa),
            format_price(remain, cfg.usa),
        ));
        return Ok(None);
    }

    let fill = executor.buy(code, market, cfg.qty, base_price).await?;
    let add_cost = fill.qty as f64 * fill.price;

    let (reason, label) = if current.is_some() {
        ("피라미딩 매수", "추가 매수")
    } else {
        ("신호 진입", "진입")
    };
    let strategy = strategy_label(cfg);
    storage.insert_trade(&TradeInsert {
        session_id,
        symbol: code,
        market: market.label(),
        side: Side::Buy,
        qty: fill.qty,
        price: fill.price,
        ts: now,
        strategy: &strategy,
        mode: executor.mode(),
        pnl: None,
        pnl_pct: None,
        reason,
    })?;

    let entry_time = current.map(|p| p.entry_time).unwrap_or(now);

    // 기본: 엔진 자체 계산
    let computed_qty = current.map(|p| p.qty).unwrap_or(0) + fill.qty;
    let computed_avg = (current_cost + add_cost) / computed_qty as f64;
    let mut new_pos = Position { qty: computed_qty, avg_price: computed_avg, entry_time };

    // live 전용: 실제 계좌 잔고로 덮어쓰기 (수수료·부분체결·환율 반영)
    if let Some(synced) = executor.sync_position(code, market).await? {
        if synced.qty != computed_qty {
            log_info(&format!(
                "  ⚠ 포지션 불일치: 엔진 기대 {}주, 계좌 실제 {}주 — 계좌 기준으로 동기화",
                computed_qty, synced.qty
            ));
        }
        new_pos = Position { qty: synced.qty, avg_price: synced.avg_price, entry_time };
    }

    log_info(&format!(
        "  → ▲ {}: {}주 @ {} · 보유 {}주 @ avg {} · used {} / budget {}",
        label,
        fill.qty,
        format_price(fill.price, cfg.usa),
        new_pos.qty,
        format_price(new_pos.avg_price, cfg.usa),
        format_price(new_pos.qty as f64 * new_pos.avg_price, cfg.usa),
        format_price(cfg.budget, cfg.usa),
    ));

    Ok(Some(new_pos))
}

#[allow(clippy::too_many_arguments)]
async fn execute_exit<E: Executor>(
    storage: &Storage,
    executor: &E,
    session_id: &str,
    code: &str,
    market: Market,
    cfg: &EngineConfig,
    pos: Position,
    base_price: f64,
    now: DateTime<Tz>,
    reason: &str,
) -> Result<()> {
    let fill = executor.sell(code, market, pos.qty, base_price).await?;
    // PnL: (매도가 - 평단가) * qty, 양쪽 수수료 차감
    let gross = (fill.price - pos.avg_price) * fill.qty as f64;
    let fee = (fill.price + pos.avg_price) * fill.qty as f64 * (cfg.fee_bps / 10_000.0);
    let pnl = gross - fee;
    let pnl_pct = if pos.avg_price > 0.0 {
        (fill.price / pos.avg_price - 1.0) * 100.0 - cfg.fee_bps / 100.0
    } else { 0.0 };
    let strategy = strategy_label(cfg);
    storage.insert_trade(&TradeInsert {
        session_id,
        symbol: code,
        market: market.label(),
        side: Side::Sell,
        qty: fill.qty,
        price: fill.price,
        ts: now,
        strategy: &strategy,
        mode: executor.mode(),
        pnl: Some(pnl),
        pnl_pct: Some(pnl_pct),
        reason,
    })?;
    let arrow = if pnl >= 0.0 { "▲" } else { "▼" };
    log_info(&format!(
        "  → ▼ 청산: {}주 @ {} [{}] · PnL {} {} ({:+.2}%)",
        fill.qty,
        format_price(fill.price, cfg.usa),
        reason,
        arrow,
        format_price(pnl.abs(), cfg.usa),
        pnl_pct,
    ));

    // live: 전량 청산 검증. 잔량이 남아있으면 경고.
    if let Some(remaining) = executor.sync_position(code, market).await? {
        log_info(&format!(
            "  ⚠ 청산 후 잔량 {}주 감지 — 부분체결/미체결 가능성, 수동 확인 필요",
            remaining.qty
        ));
    }
    Ok(())
}

fn print_report(storage: &Storage, session_id: &str, cfg: &EngineConfig, market: Market) {
    match storage.session_summary(session_id) {
        Ok(s) => {
            let win_rate = if s.sells > 0 {
                s.wins as f64 / s.sells as f64 * 100.0
            } else { 0.0 };
            eprintln!();
            eprintln!("═══ 일일 리포트 ({}, session={}) ═══", market.label(), session_id);
            eprintln!("  체결 건수     : {} (매수 {}, 매도 {})", s.trades, s.trades.saturating_sub(s.sells), s.sells);
            eprintln!("  승률          : {}/{} = {:.1}%", s.wins, s.sells, win_rate);
            eprintln!("  총 PnL        : {}", format_price(s.total_pnl, cfg.usa));
            eprintln!("  평균 PnL %    : {:+.2}%", s.avg_pnl_pct);
            eprintln!("  DB            : {}", config::daytrade_db_path().map(|p| p.display().to_string()).unwrap_or_default());
        }
        Err(e) => {
            log_error(&format!("리포트 집계 실패: {e}"));
        }
    }
}

fn new_session_id(code: &str, market: Market) -> String {
    let now = session::now_kst();
    format!("{}_{}_{}", market.label(), code, now.format("%Y%m%d_%H%M%S"))
}

/// 단일 또는 복합 전략에 따라 분기. composite는 child별 신호를 backtest::latest_signal 로 받아 결합.
fn compute_signal(cfg: &EngineConfig, series: &crate::commands::analyze::Series) -> i8 {
    if let (StrategyKind::Composite, Some(comp)) = (cfg.strategy, &cfg.composite) {
        if comp.children.is_empty() {
            return 0;
        }
        let signals: Vec<i8> = comp
            .children
            .iter()
            .map(|c| backtest::latest_signal(series, &child_params(c, cfg.fee_bps)))
            .collect();
        match comp.combinator {
            Combinator::And => and_signal(&signals),
            Combinator::Or => or_signal(&signals),
        }
    } else {
        let params = build_params(cfg);
        backtest::latest_signal(series, &params)
    }
}

fn and_signal(signals: &[i8]) -> i8 {
    if signals.iter().all(|&s| s > 0) {
        1
    } else {
        0
    }
}

fn or_signal(signals: &[i8]) -> i8 {
    if signals.iter().any(|&s| s > 0) {
        1
    } else {
        0
    }
}

fn child_params(c: &CompositeChild, fee_bps: f64) -> Params {
    Params {
        strategy: c.strategy,
        period: 'D',
        from: None,
        to: None,
        fee_bps,
        slippage_bps: 0.0,
        allow_short: false,
        leverage: 1.0,
        stop_loss_pct: None,
        take_profit_pct: None,
        fast: c.fast,
        slow: c.slow,
        rsi_period: c.rsi_period,
        rsi_oversold: c.rsi_oversold,
        rsi_overbought: c.rsi_overbought,
        bb_period: c.bb_period,
        bb_sigma: c.bb_sigma,
        obv_period: c.obv_period,
        manual_entry_date: None,
        manual_exit_date: None,
        manual_direction: None,
    }
}

fn build_params(cfg: &EngineConfig) -> Params {
    Params {
        strategy: cfg.strategy,
        period: 'D',
        from: None, to: None,
        fee_bps: cfg.fee_bps,
        slippage_bps: 0.0,
        allow_short: false, leverage: 1.0,
        stop_loss_pct: None, take_profit_pct: None,
        fast: cfg.fast, slow: cfg.slow,
        rsi_period: cfg.rsi_period,
        rsi_oversold: cfg.rsi_oversold,
        rsi_overbought: cfg.rsi_overbought,
        bb_period: cfg.bb_period,
        bb_sigma: cfg.bb_sigma,
        obv_period: cfg.obv_period,
        manual_entry_date: None,
        manual_exit_date: None,
        manual_direction: None,
    }
}

fn format_sl_tp_label(pct: Option<f64>, atr_mult: Option<f64>, atr_period: usize) -> String {
    match (pct, atr_mult) {
        (None, None) => "off".into(),
        (Some(p), None) => format!("{:.2}%", p),
        (None, Some(m)) => format!("ATR({})×{:.1}", atr_period, m),
        (Some(p), Some(m)) => format!("{:.2}% ∧ ATR({})×{:.1}", p, atr_period, m),
    }
}

pub(crate) fn format_price(v: f64, usa: bool) -> String {
    if usa {
        format!("{:.4} USD", v)
    } else {
        format!("{}원", format_number(&format!("{:.0}", v)))
    }
}

fn format_ts(ts: &str) -> String {
    if ts.len() == 12 {
        format!("{}-{}-{} {}:{}", &ts[0..4], &ts[4..6], &ts[6..8], &ts[8..10], &ts[10..12])
    } else {
        ts.to_string()
    }
}

fn signal_label(s: i8) -> &'static str {
    match s {
        1 => "+1 (long)",
        -1 => "-1 (flat, long-only)",
        _ => "0 (flat)",
    }
}

fn strategy_label(cfg: &EngineConfig) -> String {
    match cfg.strategy {
        StrategyKind::MaCross => format!(
            "ma-cross({}/{})",
            cfg.fast.unwrap_or(20), cfg.slow.unwrap_or(60)
        ),
        StrategyKind::Rsi => format!(
            "rsi({}, {:.0}/{:.0})",
            cfg.rsi_period.unwrap_or(14),
            cfg.rsi_oversold.unwrap_or(30.0),
            cfg.rsi_overbought.unwrap_or(70.0),
        ),
        StrategyKind::Macd => "macd(12/26/9)".into(),
        StrategyKind::Bollinger => format!(
            "bollinger({}, {}σ)",
            cfg.bb_period.unwrap_or(20), cfg.bb_sigma.unwrap_or(2.0),
        ),
        StrategyKind::Ichimoku => "ichimoku(9/26/52)".into(),
        StrategyKind::Obv => format!("obv({})", cfg.obv_period.unwrap_or(20)),
        StrategyKind::Manual => "manual".into(),
        StrategyKind::Composite => match &cfg.composite {
            Some(c) => {
                let parts: Vec<String> = c.children.iter().map(child_label).collect();
                let sep = if matches!(c.combinator, Combinator::And) { " ∧ " } else { " ∨ " };
                format!("composite[{}]", parts.join(sep))
            }
            None => "composite[?]".into(),
        },
    }
}

fn child_label(c: &CompositeChild) -> String {
    match c.strategy {
        StrategyKind::MaCross => format!(
            "ma-cross({}/{})",
            c.fast.unwrap_or(20),
            c.slow.unwrap_or(60),
        ),
        StrategyKind::Rsi => format!(
            "rsi({}, {:.0}/{:.0})",
            c.rsi_period.unwrap_or(14),
            c.rsi_oversold.unwrap_or(30.0),
            c.rsi_overbought.unwrap_or(70.0),
        ),
        StrategyKind::Macd => "macd(12/26/9)".into(),
        StrategyKind::Bollinger => format!(
            "bollinger({}, {}σ)",
            c.bb_period.unwrap_or(20),
            c.bb_sigma.unwrap_or(2.0),
        ),
        StrategyKind::Ichimoku => "ichimoku(9/26/52)".into(),
        StrategyKind::Obv => format!("obv({})", c.obv_period.unwrap_or(20)),
        StrategyKind::Manual | StrategyKind::Composite => "?".into(),
    }
}

pub(crate) fn log_info(msg: &str) {
    eprintln!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

pub(crate) fn log_error(msg: &str) {
    eprintln!("[{}] ERROR: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}
