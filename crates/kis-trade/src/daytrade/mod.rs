//! 데이트레이드 엔진 — 분봉 기반 자동매매.
//!
//! - `daemon`       : toml 오케스트레이터 (컨테이너 데몬 진입점, cancel 주입)
//! - `engine`       : 공통 매매 엔진(Executor trait, tick/진입/청산, 신호 계산)
//! - `live`/`paper` : 실거래 / 모의(슬리피지) Executor
//! - `run`/`signal_watch` : 인터랙티브 단일 전략 실행 (foreground)
//! - `fetch`        : 분봉 조회·집계
//! - `dconfig`      : daytrade.toml 스키마·로드
//! - `store`        : 거래 기록 SQLite 저장소
//!
//! 시장 세션·봉 주기는 [`crate::common::session`] / [`crate::common::period`] 공용.

pub mod daemon;
pub mod dconfig;
pub mod engine;
pub mod fetch;
pub mod live;
pub mod paper;
pub mod run;
pub mod signal_watch;
pub mod store;
