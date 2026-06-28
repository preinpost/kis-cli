//! `kis daytrade` — CLI 표층(이력 출력 + systemd/toml 관리)만 kis-cli 에 남고,
//! 매매 엔진은 kis-trade::daytrade 로 이동했다.
//!
//! - `history`   : 거래 이력 조회·출력 (store 질의 + 표/JSON)
//! - `lifecycle` : `daytrade add/rm/list/start/stop/...` systemd·toml 관리

pub mod history;
pub mod lifecycle;

// 엔진은 kis-trade::daytrade 로, 봉 주기는 kis-trade::common 으로 이동 —
// 기존 경로(`commands::daytrade::*`)를 유지하기 위한 재노출(kis-cli 가 실제로 참조하는 것만).
pub use kis_trade::common::period;
pub use kis_trade::daytrade::{daemon, dconfig, engine, paper, run, signal_watch};
// 기존 `storage` 경로 호환 (개명: storage → store).
pub use kis_trade::daytrade::store as storage;
