//! `kisd-stop-loss` — 자동 손절 데몬 바이너리 (컨테이너 전용).
//!
//! 기본 동작 = 데몬 실행. 잔고를 감시해 임계치 손실 도달 시 매도(`--execute` 시 실주문).
//! 엔진은 [`kis_trade::stop_loss`]. 로깅·SIGTERM 그레이스풀 종료를 여기서 배선한다.

use anyhow::Result;
use clap::Parser;
use tokio_util::sync::CancellationToken;

#[derive(Parser)]
#[command(name = "kisd-stop-loss", about = "자동 손절 데몬", disable_version_flag = true)]
struct Cli {
    /// 손절 임계치 (%). 이 값보다 손실이 크면 매도. 기본 -5.0
    #[arg(long, default_value_t = -5.0, allow_hyphen_values = true)]
    threshold: f64,
    /// 확인 주기 (초). 기본 30
    #[arg(long, default_value_t = 30)]
    interval: u64,
    /// 감시 대상 (쉼표 구분, 코드 또는 종목명 일부). 미지정 시 전체 잔고
    #[arg(long)]
    symbols: Option<String>,
    /// 실제 매도 집행. 없으면 dry-run (로그만)
    #[arg(long)]
    execute: bool,
    /// 해외주식 지정가 스프레드 (%). 기본 1.0
    #[arg(long, default_value_t = 1.0)]
    usa_spread: f64,
    /// WebSocket 실시간 감시 (폴링 대신 tick 단위)
    #[arg(long)]
    ws: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _log_guard = kis_daemon::logging::init_daemon("stop-loss")?;
    let cli = Cli::parse();

    let cancel = CancellationToken::new();
    kis_daemon::shutdown::spawn_signal_listener(cancel.clone());

    let client = kis_trade::common::client::build_client()?;
    let symbols = cli.symbols.map(|s| {
        s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect::<Vec<_>>()
    });
    let cfg = kis_trade::stop_loss::Config {
        threshold_pct: cli.threshold,
        interval_secs: cli.interval,
        symbols,
        execute: cli.execute,
        usa_spread_pct: cli.usa_spread,
        use_ws: cli.ws,
    };
    kis_trade::stop_loss::run(&client, cfg, cancel).await
}
