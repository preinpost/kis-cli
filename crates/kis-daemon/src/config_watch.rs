//! 설정 파일 핫리로드 감시 — `notify-debouncer-mini` 기반.
//!
//! 대상 파일의 **부모 디렉토리**를 watch 한다(에디터의 atomic write = rename 패턴 대응 —
//! 파일이 통째로 교체돼도 감지). 변경 감지 시 `tx` 로 `()` 신호만 보내고, 실제 재로드는
//! 호출자가 수행한다. 500ms 디바운스로 연속 쓰기를 1건으로 합친다.
//!
//! 반환된 `Debouncer` 가 drop 되면 watch 가 해제되므로, 호출자는 살아있는 동안 보관해야 한다.

use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebouncedEvent};
use tokio::sync::mpsc;
use tracing::{error, info};

/// `path` 파일의 변경을 감시한다. 변경 시 `tx.send(())`.
/// 부모 디렉토리가 없으면 생성한다.
pub fn spawn_watcher(
    path: &Path,
    tx: mpsc::UnboundedSender<()>,
) -> Result<notify_debouncer_mini::Debouncer<notify_debouncer_mini::notify::RecommendedWatcher>> {
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("watch 대상의 부모 디렉토리 없음: {}", path.display()))?
        .to_path_buf();
    if !parent.exists() {
        std::fs::create_dir_all(&parent)
            .with_context(|| format!("config dir 생성 실패: {}", parent.display()))?;
    }
    let path_filter = path.to_path_buf();
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        move |res: Result<Vec<DebouncedEvent>, notify_debouncer_mini::notify::Error>| match res {
            Ok(events) => {
                if events.iter().any(|e| e.path == path_filter) {
                    let _ = tx.send(());
                }
            }
            Err(e) => error!("watcher 에러: {e}"),
        },
    )
    .context("file watcher 초기화 실패")?;
    debouncer
        .watcher()
        .watch(&parent, RecursiveMode::NonRecursive)
        .with_context(|| format!("디렉토리 watch 실패: {}", parent.display()))?;
    info!("watcher 등록 — {}", parent.display());
    Ok(debouncer)
}
