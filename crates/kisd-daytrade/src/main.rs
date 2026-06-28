//! `kisd-daytrade` — 데이트레이드 자동매매 데몬 바이너리 (컨테이너 전용).
//!
//! 기본 동작 = `daytrade.toml` 오케스트레이터 실행(전략별 task spawn + 파일 hot-reload).
//! 전략 등록(`daytrade add`)·종목 마스터 sync 는 kis-cli 이미지로 처리한다.
//! 엔진은 [`kis_trade::daytrade::daemon`].

use std::sync::Arc;

use anyhow::Result;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    let _log_guard = kis_daemon::logging::init_daemon("daytrade")?;

    let cancel = CancellationToken::new();
    kis_daemon::shutdown::spawn_signal_listener(cancel.clone());

    let client = Arc::new(kis_trade::common::client::build_client()?);
    kis_trade::daytrade::daemon::run(client, cancel).await
}
