//! 데몬용 tracing subscriber 초기화.
//!
//! `daytrade daemon` / `signal-watch` / `stop-loss` 진입점에서 호출:
//! ```ignore
//! let _guard = logging::init_daemon("daytrade")?;
//! ```
//! 반환된 `WorkerGuard`는 함수 끝까지 살려둬야 비동기 appender 가 flush 된다.
//!
//! 출력 위치 — Linux 우선:
//! 1. `/var/log/kis-cli/<name>.log` (디렉터리가 쓰기 가능하면)
//! 2. 그 외 (macOS / 권한 없음): `~/.local/state/kis-cli/logs/<name>.log`
//!
//! 파일은 일별 롤링 (`<name>.log.YYYY-MM-DD`). stderr 에도 동시 출력해서 foreground 디버깅 가능.

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

const SYSTEM_LOG_DIR: &str = "/var/log/kis-cli";

pub struct DaemonLogging {
    pub _guard: WorkerGuard,
    pub log_dir: PathBuf,
    pub file_name: String,
}

pub fn init_daemon(name: &str) -> Result<DaemonLogging> {
    let (dir, file_name) = resolve_log_path(name)?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("로그 디렉터리 생성 실패: {}", dir.display()))?;

    let appender = tracing_appender::rolling::daily(&dir, &file_name);
    let (file_writer, guard) = tracing_appender::non_blocking(appender);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(false);

    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_target(false);

    let result = tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(stderr_layer)
        .try_init();

    if let Err(e) = result {
        return Err(anyhow!("tracing subscriber 초기화 실패: {e}"));
    }

    Ok(DaemonLogging {
        _guard: guard,
        log_dir: dir,
        file_name,
    })
}

/// 데몬 외부 (예: `kis daytrade logs`) 에서 현재 활성 로그파일 경로를 알아낼 때 사용.
/// Linux: `/var/log/kis-cli/<name>.log` 가 쓰기 가능하면 그 경로, 아니면 user fallback.
pub fn current_log_path(name: &str) -> Result<PathBuf> {
    let (dir, file_name) = resolve_log_path(name)?;
    Ok(dir.join(file_name))
}

/// foreground 단발성 커맨드 (`kis daytrade paper` / `run` / `signal-watch`) 용.
/// 파일 appender 없이 stderr 만 사용해 사용자 터미널에 흐른다.
/// 이미 subscriber 가 등록돼 있으면 조용히 무시 (다중 호출 안전).
pub fn init_foreground() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(true)
                .with_target(false),
        )
        .try_init();
}

fn resolve_log_path(name: &str) -> Result<(PathBuf, String)> {
    let file_name = format!("{name}.log");

    if cfg!(target_os = "linux") {
        let sys = PathBuf::from(SYSTEM_LOG_DIR);
        if is_writable_dir(&sys) {
            return Ok((sys, file_name));
        }
    }

    // fallback: ~/.local/state/kis-cli/logs/
    let base = dirs::state_dir()
        .or_else(dirs::data_local_dir)
        .or_else(dirs::home_dir)
        .ok_or_else(|| anyhow!("사용자 state/data/home 디렉터리 확인 실패"))?;
    let dir = base.join("kis-cli").join("logs");
    Ok((dir, file_name))
}

fn is_writable_dir(p: &std::path::Path) -> bool {
    if !p.is_dir() {
        return false;
    }
    // 쓰기 권한 체크 — 파일 하나 생성 시도. 실패 시 false.
    let probe = p.join(".kis-cli-write-probe");
    match fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&probe)
    {
        Ok(_) => {
            let _ = fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}
