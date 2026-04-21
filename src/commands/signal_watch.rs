//! `kis signal-watch` — cron 스케줄에 맞춰 전략 신호를 주기적으로 로그로 남기는 감시 도구.
//!
//! **설계**
//! - `tokio-cron-scheduler` 로 cron 표현식에 따라 주기 실행
//! - 매 실행마다: 캔들 fetch → 최신 신호 → 현재 보유 수량 → **전이 판단** → **로그**
//! - 주문 실행 없음 (의도적). 자동 매매는 별도 커맨드로 분리 예정.
//!
//! **출력 예시**
//! ```
//! [2026-04-19 15:45:03] tick [005930] 삼성전자
//!   최신봉 20260419 / 종가 89,000원 / 신호 +1 (long)
//!   현재 보유: 0주
//!   → ▲ 진입 신호 (BUY 0주 권장)
//! ```

use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::Local;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::api::domestic_stock::order_account::inquire_balance as inquire_balance_domestic;
use crate::api::overseas_stock::order_account::inquire_balance as inquire_balance_overseas;
use crate::client::KisClient;
use crate::commands::backtest::{self, Params, StrategyKind};
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::symbols::{Market, ResolveMode};

pub struct Config {
    pub symbol: String,
    pub strategy: StrategyKind,
    pub cron: String,
    pub period: char,
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
    let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, mode, cfg.pick)?;
    let display_name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };
    log_info(&format!(
        "signal-watch 시작: [{}] {} ({}) · 전략 {} · cron \"{}\" (감시 전용, 주문 없음)",
        sym.code,
        display_name,
        sym.market.as_str(),
        strategy_label(&cfg),
        cfg.cron,
    ));

    let cfg = Arc::new(cfg);
    let sym_code = sym.code.clone();
    let sym_market = sym.market;
    let sym_name = display_name;

    let mut sched = JobScheduler::new().await?;
    let client_cl = client.clone();
    let cfg_cl = cfg.clone();
    let sym_code_cl = sym_code.clone();
    let sym_name_cl = sym_name.clone();

    let job = Job::new_async(cfg.cron.as_str(), move |_uuid, _l| {
        let client = client_cl.clone();
        let cfg = cfg_cl.clone();
        let code = sym_code_cl.clone();
        let name = sym_name_cl.clone();
        Box::pin(async move {
            if let Err(e) = tick(&client, &cfg, &code, &name, sym_market).await {
                log_error(&format!("tick 실패: {e}"));
            }
        })
    })?;
    sched.add(job).await?;
    sched.start().await?;

    log_info("스케줄러 시작됨. 종료: Ctrl+C");

    tokio::signal::ctrl_c().await?;
    log_info("종료 신호 수신, 스케줄러 정리…");
    sched.shutdown().await.ok();
    Ok(())
}

async fn tick(client: &KisClient, cfg: &Config, code: &str, name: &str, market: Market) -> Result<()> {
    log_info(&format!("── tick [{}] {} ──", code, name));

    let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let params = build_params(cfg);
    let series = backtest::fetch_series_range(
        client,
        code,
        mode,
        cfg.period,
        None,
        None,
    )
    .await?;
    if series.closes.len() < 30 {
        return Err(anyhow!("데이터 부족 ({}봉)", series.closes.len()));
    }
    let signal = backtest::latest_signal(&series, &params);
    let last_price = series.closes.last().copied().unwrap_or(f64::NAN);
    let last_date = series.dates.last().cloned().unwrap_or_default();
    let (price_str, unit) = if cfg.usa {
        (format!("{:.4}", last_price), "USD")
    } else {
        (format_number(&format!("{:.0}", last_price)), "원")
    };
    log_info(&format!(
        "  최신봉 {} / 종가 {}{} / 신호 {}",
        last_date,
        price_str,
        unit,
        signal_label(signal)
    ));

    let held = if cfg.usa {
        current_holding_qty_overseas(client, code, market).await?
    } else {
        current_holding_qty_domestic(client, code).await?
    };
    log_info(&format!("  현재 보유: {}주", held));

    match classify(signal, held) {
        Alert::None => log_info("  → 변화 없음"),
        Alert::EntryRecommended => log_info("  → ▲ 진입 신호 (미보유 → long 전략)"),
        Alert::ExitRecommended => log_info(&format!(
            "  → ▼ 청산 신호 (보유 {}주 → flat 전략)", held
        )),
    }
    Ok(())
}

#[derive(Debug)]
enum Alert {
    None,
    EntryRecommended,
    ExitRecommended,
}

fn classify(signal: i8, held: u64) -> Alert {
    if signal > 0 && held == 0 {
        Alert::EntryRecommended
    } else if signal <= 0 && held > 0 {
        Alert::ExitRecommended
    } else {
        Alert::None
    }
}

fn build_params(cfg: &Config) -> Params {
    Params {
        strategy: cfg.strategy,
        period: cfg.period,
        from: None,
        to: None,
        fee_bps: 5.0,
        slippage_bps: 0.0,
        allow_short: false,
        leverage: 1.0,
        stop_loss_pct: None,
        take_profit_pct: None,
        fast: cfg.fast,
        slow: cfg.slow,
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

async fn current_holding_qty_domestic(client: &KisClient, code: &str) -> Result<u64> {
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

async fn current_holding_qty_overseas(client: &KisClient, code: &str, market: Market) -> Result<u64> {
    let excg = match market {
        Market::Nasdaq => "NASD",
        Market::Nyse => "NYSE",
        Market::Amex => "AMEX",
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
