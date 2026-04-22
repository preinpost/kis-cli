//! `kis daytrade paper` — **실전 서버 기반 모의테스트**.
//!
//! 실전 KIS API에서 받은 분봉 데이터로 가상 매매를 돌린다. 실주문은 절대 발생하지 않음.
//! 체결은 전부 가상이지만 시세/신호는 실제 시장 기반.
//!
//! - 매 봉 마감 직후 tick: 분봉 fetch → 신호 계산 → 가상 체결 → SQLite 기록
//! - **청산 우선순위**: EOD 강제 청산 > 손절(SL) > 익절(TP) > 전략 신호 청산
//! - **장 마감 10분 전 보유 포지션 강제 청산** (오버나이트 금지)
//! - 세션 단위로 `session_id` 부여, 프로세스 종료(Ctrl+C) 시 일일 리포트 출력
//!
//! 체결 가격: 최신 봉 종가 × (1 ± `slippage_bps`/10000).
//! 수수료: 매매 한쪽당 `fee_bps` 적용, 청산 시 양쪽 합산 차감.
//!
//! ⚠ 분봉 API는 모의투자 미지원 — 실전 계정에서만 동작.

use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use chrono_tz::Tz;

use crate::client::KisClient;
use crate::commands::backtest::{self, Params, StrategyKind};
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::config;
use crate::symbols::{Market as SymMarket, ResolveMode};

use super::fetch;
use super::period::Period;
use super::session::{self, Market};
use super::storage::{Mode, Side, Storage, TradeInsert};

pub struct Config {
    pub symbol: String,
    pub strategy: StrategyKind,
    pub period: Period,
    pub usa: bool,
    pub pick: Option<usize>,
    pub qty: u64,
    pub fee_bps: f64,
    pub slippage_bps: f64,
    /// 손절 임계 (%). `Some(2.0)` = 진입가 대비 -2% 하회 시 즉시 청산.
    pub stop_loss_pct: Option<f64>,
    /// 익절 임계 (%). `Some(3.0)` = 진입가 대비 +3% 도달 시 즉시 청산.
    pub take_profit_pct: Option<f64>,
    /// 총 예산 한도. 보유 중에도 롱 신호마다 `qty`주씩 추가 매수(피라미딩),
    /// 단 `보유 비용 + 다음 체결 비용 ≤ budget` 조건을 충족할 때만.
    pub budget: f64,
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
}

/// 현재 세션에서의 가상 포지션.
#[derive(Debug, Clone)]
struct Position {
    qty: u64,
    avg_price: f64,
    entry_time: DateTime<Tz>,
}

pub async fn run(client: Arc<KisClient>, cfg: Config) -> Result<()> {
    let market = if cfg.usa { Market::Usa } else { Market::Krx };
    let resolve = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, resolve, cfg.pick)?;
    let name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };

    let storage = Storage::open(&config::daytrade_db_path()?)?;
    let mut session_id = new_session_id(&sym.code, market);

    let sl_label = cfg
        .stop_loss_pct
        .map(|p| format!("{:.2}%", p))
        .unwrap_or_else(|| "off".into());
    let tp_label = cfg
        .take_profit_pct
        .map(|p| format!("{:.2}%", p))
        .unwrap_or_else(|| "off".into());
    log_info(&format!(
        "daytrade paper 시작 (실전 서버 기반 모의테스트): [{}] {} ({}) · {} · qty={} · budget={} · fee={:.1}bps · slip={:.1}bps · SL={} · TP={} · session={}",
        sym.code, name, market.label(), cfg.period.label(),
        cfg.qty, format_price(cfg.budget, cfg.usa),
        cfg.fee_bps, cfg.slippage_bps, sl_label, tp_label, session_id
    ));

    let cfg = Arc::new(cfg);
    let code = sym.code.clone();
    let sym_market = sym.market;
    let mut position: Option<Position> = None;
    // EOD 청산 + 리포트가 이미 한 번 출력됐는지. true면 Ctrl+C 경로에서 중복 출력을 피하고,
    // 다음 tick에서 장이 다시 열렸을 때 session_id 롤오버 트리거로도 사용한다.
    let mut session_reported = false;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                if !session_reported {
                    log_info("종료 신호 수신 — 일일 리포트 출력");
                    print_report(&storage, &session_id, &cfg, market);
                } else {
                    log_info("종료 신호 수신");
                }
                return Ok(());
            }
            _ = sleep_until_next_tick(market, cfg.period) => {}
        }

        if let Err(e) = tick(
            &client, &cfg, &code, &name, market, sym_market,
            &storage, &mut session_id, &mut session_reported, &mut position,
        ).await {
            log_error(&format!("tick 실패: {e}"));
        }
    }
}

async fn sleep_until_next_tick(market: Market, period: Period) {
    let now = session::now_kst();
    if !session::is_in_session(market, now) {
        let wait = session::time_until_open(market, now);
        let mins = wait.num_minutes().max(1);
        log_info(&format!("세션 밖 — 다음 개장까지 약 {}분 대기", mins));
        let chunk = if mins > 30 { 30 } else { mins };
        tokio::time::sleep(std::time::Duration::from_secs((chunk * 60) as u64)).await;
        return;
    }
    let now = session::now_kst();
    let next = session::next_bar_boundary_kst(period, now, 10);
    let wait = (next - now).to_std().unwrap_or(std::time::Duration::from_secs(60));
    tokio::time::sleep(wait).await;
}

#[allow(clippy::too_many_arguments)]
async fn tick(
    client: &KisClient,
    cfg: &Config,
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
    let params = build_params(cfg);
    let signal = backtest::latest_signal(&series, &params);
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
                storage, session_id, code, market, cfg, pos,
                last_price, now, "EOD 강제 청산",
            )?;
        } else {
            log_info("  ⚠ 장 마감 10분 전 — 신규 진입 차단 (EOD)");
        }
        // 세션당 한 번만 리포트 출력, 다음 tick에서 장이 다시 열리면 자동 롤오버
        if !*session_reported {
            print_report(storage, session_id, cfg, market);
            *session_reported = true;
        }
        return Ok(());
    }

    // 2) 손절(SL) / 익절(TP) — 신호보다 우선 --------------------------------
    if let Some(pos) = position.as_ref() {
        if pos.avg_price > 0.0 {
            let change_pct = (last_price / pos.avg_price - 1.0) * 100.0;
            let sl_hit = cfg.stop_loss_pct.is_some_and(|p| change_pct <= -p);
            let tp_hit = cfg.take_profit_pct.is_some_and(|p| change_pct >= p);
            if sl_hit || tp_hit {
                let reason = if sl_hit {
                    format!("손절 ({:+.2}% ≤ -{:.2}%)", change_pct, cfg.stop_loss_pct.unwrap())
                } else {
                    format!("익절 ({:+.2}% ≥ +{:.2}%)", change_pct, cfg.take_profit_pct.unwrap())
                };
                let pos = position.take().unwrap();
                execute_exit(
                    storage, session_id, code, market, cfg, pos,
                    last_price, now, &reason,
                )?;
                return Ok(());
            }
        }
    }

    // 3) 신호 기반 ---------------------------------------------------------
    match (signal, position.as_ref()) {
        (s, None) if s > 0 => {
            if let Some(new_pos) = execute_entry(
                storage, session_id, code, market, cfg, None, last_price, now,
            )? {
                *position = Some(new_pos);
            }
        }
        (s, Some(_)) if s > 0 => {
            let current = position.as_ref().unwrap().clone();
            if let Some(new_pos) = execute_entry(
                storage, session_id, code, market, cfg, Some(&current), last_price, now,
            )? {
                *position = Some(new_pos);
            }
        }
        (s, Some(_)) if s <= 0 => {
            let pos = position.take().unwrap();
            execute_exit(
                storage, session_id, code, market, cfg, pos,
                last_price, now, "신호 청산",
            )?;
        }
        _ => log_info("  → 변화 없음"),
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn execute_entry(
    storage: &Storage,
    session_id: &str,
    code: &str,
    market: Market,
    cfg: &Config,
    current: Option<&Position>,
    base_price: f64,
    now: DateTime<Tz>,
) -> Result<Option<Position>> {
    let fill = base_price * (1.0 + cfg.slippage_bps / 10_000.0);
    let add_cost = cfg.qty as f64 * fill;
    let current_cost = current.map(|p| p.qty as f64 * p.avg_price).unwrap_or(0.0);

    if current_cost + add_cost > cfg.budget {
        let remain = (cfg.budget - current_cost).max(0.0);
        log_info(&format!(
            "  → 진입 보류: 예산 초과 (필요 {}, 남은 예산 {})",
            format_price(add_cost, cfg.usa),
            format_price(remain, cfg.usa),
        ));
        return Ok(None);
    }

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
        qty: cfg.qty,
        price: fill,
        ts: now,
        strategy: &strategy,
        mode: Mode::Paper,
        pnl: None,
        pnl_pct: None,
        reason,
    })?;

    let new_qty = current.map(|p| p.qty).unwrap_or(0) + cfg.qty;
    let new_avg = (current_cost + add_cost) / new_qty as f64;
    let entry_time = current.map(|p| p.entry_time).unwrap_or(now);

    log_info(&format!(
        "  → ▲ {} (가상): {}주 @ {} [슬리피지 +{:.1}bps] · 보유 {}주 @ avg {} · used {} / budget {}",
        label,
        cfg.qty,
        format_price(fill, cfg.usa),
        cfg.slippage_bps,
        new_qty,
        format_price(new_avg, cfg.usa),
        format_price(new_qty as f64 * new_avg, cfg.usa),
        format_price(cfg.budget, cfg.usa),
    ));

    Ok(Some(Position { qty: new_qty, avg_price: new_avg, entry_time }))
}

#[allow(clippy::too_many_arguments)]
fn execute_exit(
    storage: &Storage,
    session_id: &str,
    code: &str,
    market: Market,
    cfg: &Config,
    pos: Position,
    base_price: f64,
    now: DateTime<Tz>,
    reason: &str,
) -> Result<()> {
    let fill = base_price * (1.0 - cfg.slippage_bps / 10_000.0);
    // PnL: (매도가 - 매수가) * qty, 양쪽 수수료 차감
    let gross = (fill - pos.avg_price) * pos.qty as f64;
    let fee = (fill + pos.avg_price) * pos.qty as f64 * (cfg.fee_bps / 10_000.0);
    let pnl = gross - fee;
    let pnl_pct = if pos.avg_price > 0.0 {
        (fill / pos.avg_price - 1.0) * 100.0 - cfg.fee_bps / 100.0
    } else { 0.0 };
    let strategy = strategy_label(cfg);
    storage.insert_trade(&TradeInsert {
        session_id,
        symbol: code,
        market: market.label(),
        side: Side::Sell,
        qty: pos.qty,
        price: fill,
        ts: now,
        strategy: &strategy,
        mode: Mode::Paper,
        pnl: Some(pnl),
        pnl_pct: Some(pnl_pct),
        reason,
    })?;
    let arrow = if pnl >= 0.0 { "▲" } else { "▼" };
    log_info(&format!(
        "  → ▼ 청산 (가상): {}주 @ {} [{}] · PnL {} {} ({:+.2}%)",
        pos.qty,
        format_price(fill, cfg.usa),
        reason,
        arrow,
        format_price(pnl.abs(), cfg.usa),
        pnl_pct,
    ));
    Ok(())
}

fn print_report(storage: &Storage, session_id: &str, cfg: &Config, market: Market) {
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

fn build_params(cfg: &Config) -> Params {
    Params {
        strategy: cfg.strategy,
        period: 'D',
        from: None, to: None,
        fee_bps: cfg.fee_bps,
        slippage_bps: cfg.slippage_bps,
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

fn format_price(v: f64, usa: bool) -> String {
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

fn strategy_label(cfg: &Config) -> String {
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
    }
}

fn log_info(msg: &str) {
    eprintln!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

fn log_error(msg: &str) {
    eprintln!(
        "[{}] ERROR: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    );
}
