//! `kis daytrade paper` — **실전 서버 기반 모의테스트**.
//!
//! 실전 KIS API에서 받은 분봉 데이터로 가상 매매를 돌린다. 실주문은 절대 발생하지 않음.
//! 체결은 전부 가상이지만 시세/신호는 실제 시장 기반.
//!
//! 메인 루프·청산 우선순위·피라미딩·세션 관리는 [`super::engine`] 공통 엔진을 사용하고,
//! 이 모듈은 `PaperExecutor` (slippage 기반 가상 체결) 만 제공한다.
//!
//! 체결 가격: 최신 봉 종가 × (1 ± `slippage_bps`/10000).
//! 수수료: 매매 한쪽당 `fee_bps` 적용, 청산 시 양쪽 합산 차감.
//!
//! ⚠ 분봉 API는 모의투자 미지원 — 실전 계정에서만 동작.

use std::sync::Arc;

use anyhow::Result;

use crate::client::KisClient;
use crate::commands::backtest::StrategyKind;

use super::engine::{self, EngineConfig, Executor, Fill};
use super::period::Period;
use super::session::Market;
use super::storage::Mode;

pub struct Config {
    pub symbol: String,
    pub strategy: StrategyKind,
    pub period: Period,
    pub usa: bool,
    pub pick: Option<usize>,
    pub qty: u64,
    pub fee_bps: f64,
    pub slippage_bps: f64,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,
    pub stop_loss_atr: Option<f64>,
    pub take_profit_atr: Option<f64>,
    pub atr_period: usize,
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

struct PaperExecutor {
    slippage_bps: f64,
}

impl Executor for PaperExecutor {
    fn mode(&self) -> Mode { Mode::Paper }
    fn start_prefix(&self) -> &'static str {
        "daytrade paper 시작 (실전 서버 기반 모의테스트)"
    }
    fn extra_start_info(&self) -> String {
        format!(" · slip={:.1}bps", self.slippage_bps)
    }
    async fn buy(&self, _code: &str, _market: Market, qty: u64, ref_price: f64) -> Result<Fill> {
        Ok(Fill { qty, price: ref_price * (1.0 + self.slippage_bps / 10_000.0) })
    }
    async fn sell(&self, _code: &str, _market: Market, qty: u64, ref_price: f64) -> Result<Fill> {
        Ok(Fill { qty, price: ref_price * (1.0 - self.slippage_bps / 10_000.0) })
    }
}

pub async fn run(client: Arc<KisClient>, cfg: Config) -> Result<()> {
    let executor = PaperExecutor { slippage_bps: cfg.slippage_bps };
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
