//! 그레이스풀 종료 — SIGTERM(unix) / Ctrl-C 를 받아 `CancellationToken` 을 취소한다.
//!
//! 컨테이너(`docker compose stop` → SIGTERM) 와 foreground(Ctrl-C) 양쪽에서 동일하게 동작.

use tokio_util::sync::CancellationToken;
use tracing::{error, info};

/// 백그라운드 태스크를 띄워 종료 신호를 감시하고, 신호 수신 시 `cancel` 을 취소한다.
/// 메인 루프가 `cancel.cancelled()` 를 select 하다 빠져나오는 패턴에 쓴다.
/// (brief stream / daytrade daemon 처럼 자체 select 루프를 가진 데몬용)
pub fn spawn_signal_listener(cancel: CancellationToken) {
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut sigterm = match signal(SignalKind::terminate()) {
                Ok(s) => s,
                Err(e) => {
                    error!("SIGTERM 핸들러 등록 실패: {e}");
                    return;
                }
            };
            tokio::select! {
                _ = sigterm.recv() => info!("SIGTERM 수신"),
                _ = tokio::signal::ctrl_c() => info!("SIGINT 수신"),
            }
        }
        #[cfg(not(unix))]
        {
            let _ = tokio::signal::ctrl_c().await;
            info!("Ctrl-C 수신");
        }
        cancel.cancel();
    });
}

/// 종료 신호(SIGTERM / Ctrl-C)가 올 때까지 대기 후 반환한다.
/// 자체 스케줄러를 await 하다 종료 신호에서 정리하는 데몬(signal-watch 등)용.
pub async fn wait_for_shutdown() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        match signal(SignalKind::terminate()) {
            Ok(mut sigterm) => {
                tokio::select! {
                    _ = sigterm.recv() => info!("SIGTERM 수신"),
                    _ = tokio::signal::ctrl_c() => info!("SIGINT 수신"),
                }
            }
            Err(e) => {
                error!("SIGTERM 핸들러 등록 실패: {e}");
                let _ = tokio::signal::ctrl_c().await;
                info!("Ctrl-C 수신");
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
        info!("Ctrl-C 수신");
    }
}
