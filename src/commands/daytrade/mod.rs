//! `kis daytrade` — 분봉 기반 데이트레이드 커맨드 트리.
//!
//! Phase 1: signal-watch (감시 전용)
//! Phase 2+: paper / run / backtest (추후 구현)

pub mod fetch;
pub mod history;
pub mod paper;
pub mod period;
pub mod session;
pub mod signal_watch;
pub mod storage;
