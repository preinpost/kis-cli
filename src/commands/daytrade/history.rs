//! `kis daytrade history` — SQLite에 쌓인 체결 기록 조회.
//!
//! 라우팅:
//! - `--session <id>` → 해당 세션의 체결 내역
//! - `--symbol` | `--today` | `--days N` → 필터 체결 내역 (최신 DESC)
//! - 아무 것도 없으면 → 최근 N개 세션 요약 (기본 10)
//! - `--json` → 어느 경로든 JSON 덤프

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Asia::Seoul;
use chrono_tz::Tz;

use crate::commands::helpers::format_number;
use crate::config;

use super::storage::{SessionRow, Storage, TradeRow};

pub struct Opts {
    pub session: Option<String>,
    pub symbol: Option<String>,
    pub today: bool,
    pub days: Option<u32>,
    pub limit: usize,
    pub json: bool,
}

pub fn run(opts: Opts) -> Result<()> {
    let storage = Storage::open(&config::daytrade_db_path()?)?;

    if let Some(sid) = opts.session.as_deref() {
        let trades = storage.trades_for_session(sid)?;
        if opts.json {
            println!("{}", serde_json::to_string_pretty(&trades)?);
        } else {
            print_session_detail(sid, &trades);
        }
        return Ok(());
    }

    let has_filter = opts.symbol.is_some() || opts.today || opts.days.is_some();
    if has_filter {
        let since = if opts.today {
            Some(today_midnight_kst_as_utc())
        } else {
            opts.days.map(|d| Utc::now() - Duration::days(d as i64))
        };
        let trades = storage.trades_filtered(opts.symbol.as_deref(), since)?;
        if opts.json {
            println!("{}", serde_json::to_string_pretty(&trades)?);
        } else {
            print_filtered_trades(&trades, &opts);
        }
        return Ok(());
    }

    let sessions = storage.recent_sessions(opts.limit)?;
    if opts.json {
        println!("{}", serde_json::to_string_pretty(&sessions)?);
    } else {
        print_sessions_table(&sessions);
    }
    Ok(())
}

// ─── 출력 ────────────────────────────────────────────────────────────────

fn print_sessions_table(sessions: &[SessionRow]) {
    if sessions.is_empty() {
        println!("기록된 세션이 없습니다. (DB: {})", db_display());
        return;
    }
    println!(
        "{:<32}  {:<4}  {:<8}  {:<14}  {:>6}  {:>8}  {:>14}  {:>8}  기간(KST)",
        "session_id", "mkt", "symbol", "strategy", "trades", "win/sell", "total_pnl", "avg %",
    );
    println!("{}", "─".repeat(130));
    for s in sessions {
        let win_sell = format!("{}/{}", s.wins, s.sells);
        let range = format_session_range(s.start_ts, s.end_ts);
        println!(
            "{:<32}  {:<4}  {:<8}  {:<14}  {:>6}  {:>8}  {:>14}  {:>+8.2}  {}",
            truncate(&s.session_id, 32),
            s.market,
            s.symbol,
            truncate(&s.strategy, 14),
            s.trades,
            win_sell,
            format_pnl(s.total_pnl, &s.market),
            s.avg_pnl_pct,
            range,
        );
    }
    println!("{}", "─".repeat(130));
    print_sessions_totals(sessions);
    println!();
    println!("상세: kis daytrade history --session <session_id>");
}

fn print_sessions_totals(sessions: &[SessionRow]) {
    use std::collections::BTreeMap;
    let mut by_market: BTreeMap<&str, (usize, usize, usize, f64, f64, usize)> = BTreeMap::new();
    for s in sessions {
        let e = by_market.entry(s.market.as_str()).or_insert((0, 0, 0, 0.0, 0.0, 0));
        e.0 += s.trades;
        e.1 += s.sells;
        e.2 += s.wins;
        e.3 += s.total_pnl;
        if s.sells > 0 {
            e.4 += s.avg_pnl_pct;
            e.5 += 1;
        }
    }
    for (market, (trades, sells, wins, total_pnl, avg_sum, avg_cnt)) in by_market {
        let win_rate = if sells > 0 {
            format!("{:.1}%", wins as f64 / sells as f64 * 100.0)
        } else {
            "-".into()
        };
        let avg = if avg_cnt > 0 {
            format!("{:+.2}%", avg_sum / avg_cnt as f64)
        } else {
            "-".into()
        };
        println!(
            "합계 ({}): {} sessions, {} trades, {}/{} win/sell ({}), {}, avg {}",
            market,
            sessions.iter().filter(|s| s.market == market).count(),
            trades,
            wins,
            sells,
            win_rate,
            format_pnl(total_pnl, market),
            avg,
        );
    }
}

fn print_session_detail(session_id: &str, trades: &[TradeRow]) {
    if trades.is_empty() {
        println!("세션을 찾을 수 없습니다: {}", session_id);
        return;
    }
    println!("session: {}", session_id);
    println!("종목: {} ({})", trades[0].symbol, trades[0].market);
    println!("전략: {}", trades[0].strategy);
    println!();
    print_trades_header();
    for t in trades {
        print_trade_row(t);
    }
    println!("{}", "─".repeat(110));
    print_trades_totals(trades);
}

fn print_trades_totals(trades: &[TradeRow]) {
    let market = &trades[0].market;
    let mut sells = 0usize;
    let mut wins = 0usize;
    let mut total_pnl = 0.0f64;
    let mut pct_sum = 0.0f64;
    let mut pct_cnt = 0usize;
    for t in trades {
        if t.side == "SELL" {
            sells += 1;
            if let Some(pnl) = t.pnl {
                total_pnl += pnl;
                if pnl > 0.0 {
                    wins += 1;
                }
            }
            if let Some(pct) = t.pnl_pct {
                pct_sum += pct;
                pct_cnt += 1;
            }
        }
    }
    let win_rate = if sells > 0 {
        format!("{:.1}%", wins as f64 / sells as f64 * 100.0)
    } else {
        "-".into()
    };
    let avg = if pct_cnt > 0 {
        format!("{:+.2}%", pct_sum / pct_cnt as f64)
    } else {
        "-".into()
    };
    println!(
        "합계: {} trades, {}/{} win/sell ({}), {}, avg {}",
        trades.len(),
        wins,
        sells,
        win_rate,
        format_pnl(total_pnl, market),
        avg,
    );
}

fn print_filtered_trades(trades: &[TradeRow], opts: &Opts) {
    if trades.is_empty() {
        println!("조건에 맞는 체결이 없습니다.");
        return;
    }
    if let Some(s) = &opts.symbol {
        print!("[symbol={}]", s);
    }
    if opts.today {
        print!("[today]");
    } else if let Some(d) = opts.days {
        print!("[{}d]", d);
    }
    println!(" total={}", trades.len());
    println!();
    print_trades_header();
    for t in trades {
        print_trade_row(t);
    }
}

fn print_trades_header() {
    println!(
        "{:<19}  {:<8}  {:<4}  {:>4}  {:>12}  {:>14}  {:>8}  {}",
        "ts(KST)", "symbol", "side", "qty", "price", "pnl", "pct", "reason",
    );
    println!("{}", "─".repeat(110));
}

fn print_trade_row(t: &TradeRow) {
    let ts_kst: DateTime<Tz> = t.ts.with_timezone(&Seoul);
    let pnl_str = t
        .pnl
        .map(|v| format_pnl(v, &t.market))
        .unwrap_or_else(|| "-".into());
    let pct_str = t
        .pnl_pct
        .map(|v| format!("{:+.2}%", v))
        .unwrap_or_else(|| "-".into());
    println!(
        "{}  {:<8}  {:<4}  {:>4}  {:>12}  {:>14}  {:>8}  {}",
        ts_kst.format("%Y-%m-%d %H:%M:%S"),
        t.symbol,
        t.side,
        t.qty,
        format_price(t.price, &t.market),
        pnl_str,
        pct_str,
        t.reason,
    );
}

// ─── 포맷 헬퍼 ───────────────────────────────────────────────────────────

fn format_price(v: f64, market: &str) -> String {
    if market == "USA" {
        format!("{:.4} USD", v)
    } else {
        format!("{}원", format_number(&format!("{:.0}", v)))
    }
}

fn format_pnl(v: f64, market: &str) -> String {
    if market == "USA" {
        format!("{:+.4} USD", v)
    } else {
        let sign = if v > 0.0 { "+" } else { "" };
        format!("{}{}원", sign, format_number(&format!("{:.0}", v)))
    }
}

fn format_session_range(start: DateTime<Utc>, end: DateTime<Utc>) -> String {
    let s = start.with_timezone(&Seoul);
    let e = end.with_timezone(&Seoul);
    if s.date_naive() == e.date_naive() {
        format!(
            "{} {}–{}",
            s.format("%Y-%m-%d"),
            s.format("%H:%M"),
            e.format("%H:%M")
        )
    } else {
        format!(
            "{} → {}",
            s.format("%m-%d %H:%M"),
            e.format("%m-%d %H:%M")
        )
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max.saturating_sub(1)).collect::<String>() + "…"
    }
}

fn today_midnight_kst_as_utc() -> DateTime<Utc> {
    use chrono::TimeZone;
    let kst = Utc::now().with_timezone(&Seoul);
    let midnight_naive = kst.date_naive().and_hms_opt(0, 0, 0).unwrap();
    Seoul
        .from_local_datetime(&midnight_naive)
        .single()
        .expect("KST midnight always valid")
        .with_timezone(&Utc)
}

fn db_display() -> String {
    config::daytrade_db_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "<unknown>".into())
}
