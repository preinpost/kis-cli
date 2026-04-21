//! `kis daytrade signal-watch` — 분봉 주기로 전략 신호를 감시. 주문 없음.
//!
//! 기존 `signal-watch` 와 차이:
//! - 분봉 전용 (1/5/10/30/60m)
//! - cron 대신 **세션 엔진 + tokio sleep** 사용 (미장 DST 자동 처리)
//! - 국내/해외 분봉 API 직접 호출

use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::Local;

use crate::api::domestic_stock::order_account::inquire_balance as inquire_balance_domestic;
use crate::api::overseas_stock::order_account::inquire_balance as inquire_balance_overseas;
use crate::client::KisClient;
use crate::commands::backtest::{self, Params, StrategyKind};
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::symbols::{Market as SymMarket, ResolveMode};

use super::fetch;
use super::period::Period;
use super::session::{self, Market};

pub struct Config {
    pub symbol: String,
    pub strategy: StrategyKind,
    pub period: Period,
    pub usa: bool,
    pub pick: Option<usize>,
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
}

pub async fn run(client: Arc<KisClient>, cfg: Config) -> Result<()> {
    let market = if cfg.usa { Market::Usa } else { Market::Krx };
    let resolve = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, resolve, cfg.pick)?;
    let name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };

    log_info(&format!(
        "daytrade signal-watch 시작: [{}] {} ({}) · {} · 전략 {} (감시 전용)",
        sym.code, name, market.label(), cfg.period.label(), strategy_label(&cfg)
    ));

    let cfg = Arc::new(cfg);
    let code = sym.code.clone();
    let sym_market = sym.market;

    // 최초 1회 tick (세션 중일 때만)
    loop {
        let now = session::now_kst();
        if !session::is_in_session(market, now) {
            let wait = session::time_until_open(market, now);
            let mins = wait.num_minutes().max(1);
            log_info(&format!(
                "세션 밖 — 다음 개장까지 약 {}분 대기",
                mins
            ));
            // 10분 이상 대기면 30분 단위로 깨어나서 재판정 (중간에 로그), 그 외는 전부 대기
            let chunk = if mins > 30 { 30 } else { mins };
            tokio::time::sleep(std::time::Duration::from_secs((chunk * 60) as u64)).await;
            continue;
        }

        if let Err(e) = tick(&client, &cfg, &code, &name, market, sym_market).await {
            log_error(&format!("tick 실패: {e}"));
        }

        // 다음 봉 경계 + 10초 슬랙까지 대기
        let now = session::now_kst();
        let next = session::next_bar_boundary_kst(cfg.period, now, 10);
        let wait = (next - now).to_std().unwrap_or(std::time::Duration::from_secs(60));
        tokio::time::sleep(wait).await;
    }
}

async fn tick(
    client: &KisClient,
    cfg: &Config,
    code: &str,
    name: &str,
    market: Market,
    sym_market: SymMarket,
) -> Result<()> {
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
    let (price_str, unit) = if cfg.usa {
        (format!("{:.4}", last_price), "USD")
    } else {
        (format_number(&format!("{:.0}", last_price)), "원")
    };
    log_info(&format!(
        "  최신봉 {} / 종가 {}{} / 신호 {}",
        format_ts(&last_ts),
        price_str,
        unit,
        signal_label(signal)
    ));

    let held = if cfg.usa {
        holding_overseas(client, code, sym_market).await?
    } else {
        holding_domestic(client, code).await?
    };
    log_info(&format!("  현재 보유: {}주", held));

    match classify(signal, held) {
        Alert::None => log_info("  → 변화 없음"),
        Alert::Entry => log_info("  → ▲ 진입 신호 (미보유 → long 전략)"),
        Alert::Exit => log_info(&format!("  → ▼ 청산 신호 (보유 {}주 → flat 전략)", held)),
    }

    // 마감 임박 힌트 (Phase 2에서 강제청산으로 승격)
    let now = session::now_kst();
    if session::should_force_exit(market, now, 10) {
        log_info("  ⚠ 장 마감 10분 전 — 데이트레이드 EOD 구간");
    }
    Ok(())
}

#[derive(Debug)]
enum Alert { None, Entry, Exit }

fn classify(signal: i8, held: u64) -> Alert {
    if signal > 0 && held == 0 { Alert::Entry }
    else if signal <= 0 && held > 0 { Alert::Exit }
    else { Alert::None }
}

fn build_params(cfg: &Config) -> Params {
    Params {
        strategy: cfg.strategy,
        period: 'D', // 분봉 fetch 이미 끝났음. backtest 내부 분기용이라 여기선 무의미.
        from: None, to: None,
        fee_bps: 5.0, slippage_bps: 0.0,
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

async fn holding_domestic(client: &KisClient, code: &str) -> Result<u64> {
    let req = inquire_balance_domestic::Request {
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
    let r = inquire_balance_domestic::call(client, &req).await?;
    for h in &r.holdings {
        if h.pdno == code {
            return Ok(h.hldg_qty.parse::<u64>().unwrap_or(0));
        }
    }
    Ok(0)
}

async fn holding_overseas(client: &KisClient, code: &str, market: SymMarket) -> Result<u64> {
    let excg = match market {
        SymMarket::Nasdaq => "NASD",
        SymMarket::Nyse => "NYSE",
        SymMarket::Amex => "AMEX",
        _ => "NASD",
    };
    let req = inquire_balance_overseas::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: excg.into(),
        tr_crcy_cd: "USD".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    let r = inquire_balance_overseas::call(client, &req).await?;
    for h in &r.holdings {
        if h.ovrs_pdno.eq_ignore_ascii_case(code) {
            return Ok(h.ovrs_cblc_qty.parse::<u64>().unwrap_or(0));
        }
    }
    Ok(0)
}

fn format_ts(ts: &str) -> String {
    // "YYYYMMDDHHmm" → "YYYY-MM-DD HH:mm"
    if ts.len() == 12 {
        format!("{}-{}-{} {}:{}", &ts[0..4], &ts[4..6], &ts[6..8], &ts[8..10], &ts[10..12])
    } else {
        ts.to_string()
    }
}

fn signal_label(s: i8) -> &'static str {
    match s {
        1 => "+1 (long)",
        -1 => "-1 (short, long-only 에선 flat 취급)",
        _ => "0 (flat)",
    }
}

fn strategy_label(cfg: &Config) -> String {
    match cfg.strategy {
        StrategyKind::MaCross => format!(
            "ma-cross({}/{})",
            cfg.fast.unwrap_or(20),
            cfg.slow.unwrap_or(60)
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
            cfg.bb_period.unwrap_or(20),
            cfg.bb_sigma.unwrap_or(2.0),
        ),
        StrategyKind::Ichimoku => "ichimoku(9/26/52)".into(),
        StrategyKind::Obv => format!("obv({})", cfg.obv_period.unwrap_or(20)),
        StrategyKind::Manual => "manual (signal-watch 부적합)".into(),
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
