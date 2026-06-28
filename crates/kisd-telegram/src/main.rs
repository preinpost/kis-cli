//! `kisd-telegram` — 텔레그램 실시간 관심종목 스트림 데몬 바이너리 (컨테이너 전용).
//!
//! 기본 동작 = 스트림 데몬 실행. 인자로 종목을 주면 watchlist(telegram-stream.toml)를
//! 그 목록으로 덮어쓰고 시작하며, 생략 시 저장된 목록을 사용한다.
//! 엔진은 [`kis_trade::telegram`]. systemd `--background` 경로는 kis-cli 에만 존재.

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use tokio_util::sync::CancellationToken;

#[derive(Parser)]
#[command(name = "kisd-telegram", about = "텔레그램 실시간 관심종목 스트림 데몬", disable_version_flag = true)]
struct Cli {
    /// 관심 종목 (이름 또는 코드). 공백 구분. 생략 시 저장된 목록(telegram-stream.toml) 사용.
    symbols: Vec<String>,
    /// 갱신 주기 (초). 기본 1
    #[arg(long, default_value_t = 1)]
    interval: u64,
    /// 세션 무시하고 즉시 1회만 전송 후 종료 (포맷 확인용)
    #[arg(long)]
    once: bool,
    /// 텔레그램 명령(/add /rm /list) 수신 끄기 (기본: 켜짐)
    #[arg(long)]
    no_listen: bool,
    /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY 필수. 예: --pick 1
    #[arg(long)]
    pick: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _log_guard = kis_daemon::logging::init_daemon("telegram-stream")?;
    let cli = Cli::parse();

    let cancel = CancellationToken::new();
    kis_daemon::shutdown::spawn_signal_listener(cancel.clone());

    let client = Arc::new(kis_trade::common::client::build_client()?);
    let cfg = kis_trade::telegram::StreamConfig {
        symbols: cli.symbols,
        interval_secs: cli.interval,
        once: cli.once,
        background: false, // systemd 설치는 kis-cli 전용
        listen: !cli.no_listen,
        pick: cli.pick,
    };
    kis_trade::telegram::run(client, cfg, cancel).await
}
