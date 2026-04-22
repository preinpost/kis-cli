//! `kis daytrade run` — KIS 계좌에서 **실제 주문 발주**.
//!
//! 공통 엔진(`engine.rs`) + [`LiveExecutor`](super::live::LiveExecutor) 조합.
//! paper와 동일한 전략·청산 우선순위 + 피라미딩 + SL/TP + EOD 강제 청산.
//!
//! 첫 실주문 전 대화형 확인 (`--yes` 생략 시).

use std::io::{self, Write};
use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::client::KisClient;
use crate::commands::backtest::StrategyKind;
use crate::commands::helpers::resolve_symbol;
use crate::symbols::ResolveMode;

use super::engine::{self, EngineConfig};
use super::live::LiveExecutor;
use super::period::Period;

pub struct Config {
    pub symbol: String,
    pub strategy: StrategyKind,
    pub period: Period,
    pub usa: bool,
    pub pick: Option<usize>,
    pub qty: u64,
    pub fee_bps: f64,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,
    pub stop_loss_atr: Option<f64>,
    pub take_profit_atr: Option<f64>,
    pub atr_period: usize,
    pub budget: f64,
    pub tick_offset: i32,
    pub fill_timeout_secs: u64,
    pub poll_interval_secs: u64,
    pub yes: bool,
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
    let resolve_mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, resolve_mode, cfg.pick)?;
    let name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };

    if !cfg.yes {
        confirm_live_trading(
            &sym.code, &name, cfg.usa, cfg.qty, cfg.budget,
            cfg.stop_loss_pct, cfg.take_profit_pct,
        )?;
    }

    let mut executor = LiveExecutor::new(client.clone(), sym.market);
    executor.tick_offset = cfg.tick_offset;
    executor.fill_timeout_secs = cfg.fill_timeout_secs;
    executor.poll_interval_secs = cfg.poll_interval_secs;

    let engine_cfg = EngineConfig {
        symbol: cfg.symbol,
        strategy: cfg.strategy,
        period: cfg.period,
        usa: cfg.usa,
        pick: cfg.pick,
        qty: cfg.qty,
        budget: cfg.budget,
        fee_bps: cfg.fee_bps,
        stop_loss_pct: cfg.stop_loss_pct,
        take_profit_pct: cfg.take_profit_pct,
        stop_loss_atr: cfg.stop_loss_atr,
        take_profit_atr: cfg.take_profit_atr,
        atr_period: cfg.atr_period,
        fast: cfg.fast,
        slow: cfg.slow,
        rsi_period: cfg.rsi_period,
        rsi_oversold: cfg.rsi_oversold,
        rsi_overbought: cfg.rsi_overbought,
        bb_period: cfg.bb_period,
        bb_sigma: cfg.bb_sigma,
        obv_period: cfg.obv_period,
    };
    engine::run(client, engine_cfg, executor).await
}

fn confirm_live_trading(
    code: &str, name: &str, usa: bool, qty: u64, budget: f64,
    sl: Option<f64>, tp: Option<f64>,
) -> Result<()> {
    let currency = if usa { "USD" } else { "KRW" };
    eprintln!();
    eprintln!("⚠ 실주문 경고");
    eprintln!("  종목:    [{}] {}", code, name);
    eprintln!("  수량:    {}주/회 (피라미딩)", qty);
    eprintln!("  예산:    {:.2} {}", budget, currency);
    eprintln!("  시장:    {}", if usa { "USA" } else { "KRX" });
    eprintln!(
        "  손절/익절: SL {} / TP {}",
        sl.map(|p| format!("-{:.2}%", p)).unwrap_or_else(|| "off".into()),
        tp.map(|p| format!("+{:.2}%", p)).unwrap_or_else(|| "off".into()),
    );
    eprintln!();
    eprint!("실주문을 진행합니까? (y/N): ");
    io::stderr().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let ans = input.trim().to_lowercase();
    if ans != "y" && ans != "yes" {
        return Err(anyhow!("사용자 취소"));
    }
    Ok(())
}
