//! `kis daytrade daemon` — toml 기반 단일 프로세스 오케스트레이터.
//!
//! 시작 시 `~/.config/kis-cli/daytrade.toml` 로드 → 각 strategy를 `tokio::spawn` 으로 격리 실행.
//! `notify-debouncer-mini` 로 toml 파일 watch (500ms debounce) → 변경 시 diff:
//! - 신규 id → spawn
//! - 삭제된 id → cancel
//! - 변경된 id → 다음 진입부터 적용 (즉시 교체는 사용자가 rm + add)
//!
//! KisClient·토큰·TPS 레이트리밋은 데몬 1개만 보유하여 중앙 관리.

use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use chrono::Local;
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebouncedEvent};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::dconfig::{self, DaytradeConfig, ExecMode, StrategyEntry};
use super::engine::{self, Combinator, CompositeChild, CompositeConfig, EngineConfig};
use super::live::LiveExecutor;
use super::paper::PaperExecutor;
use super::period::Period;
use crate::client::KisClient;
use crate::commands::backtest::StrategyKind;
use crate::symbols::{self, ResolveMode, ResolvedSymbol};

struct RunningTask {
    cancel: CancellationToken,
    handle: JoinHandle<()>,
}

pub async fn run(client: Arc<KisClient>) -> Result<()> {
    let cfg_path = dconfig::config_path()?;
    log_info(&format!("daemon 시작 — 설정 파일: {}", cfg_path.display()));

    let global_cancel = CancellationToken::new();
    spawn_signal_listener(global_cancel.clone());

    let mut tasks: HashMap<String, RunningTask> = HashMap::new();

    let initial = DaytradeConfig::load().unwrap_or_else(|e| {
        log_error(&format!("초기 toml 로드 실패: {e}"));
        DaytradeConfig::default()
    });
    apply_diff(&client, &mut tasks, &[], &initial.strategies, &global_cancel);

    let (tx, mut rx) = mpsc::unbounded_channel::<()>();
    let _watcher_guard = spawn_watcher(&cfg_path, tx)?;

    let mut current = initial.strategies.clone();

    loop {
        tokio::select! {
            _ = global_cancel.cancelled() => {
                log_info("종료 신호 — 모든 strategy 정리");
                break;
            }
            evt = rx.recv() => {
                if evt.is_none() {
                    log_info("watcher 종료됨");
                    break;
                }
                match DaytradeConfig::load() {
                    Ok(new_cfg) => {
                        let next = new_cfg.strategies;
                        apply_diff(&client, &mut tasks, &current, &next, &global_cancel);
                        current = next;
                    }
                    Err(e) => log_error(&format!("toml 재로드 실패 (변경 무시): {e}")),
                }
            }
        }
    }

    for (id, task) in tasks.drain() {
        task.cancel.cancel();
        if let Err(e) = task.handle.await {
            log_error(&format!("task {} join 실패: {e}", short(&id)));
        }
    }
    log_info("daemon 정상 종료");
    Ok(())
}

fn spawn_signal_listener(cancel: CancellationToken) {
    tokio::spawn(async move {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut sigterm = match signal(SignalKind::terminate()) {
                Ok(s) => s,
                Err(e) => {
                    log_error(&format!("SIGTERM 핸들러 등록 실패: {e}"));
                    return;
                }
            };
            tokio::select! {
                _ = sigterm.recv() => log_info("SIGTERM 수신"),
                _ = tokio::signal::ctrl_c() => log_info("SIGINT 수신"),
            }
        }
        #[cfg(not(unix))]
        {
            let _ = tokio::signal::ctrl_c().await;
            log_info("Ctrl-C 수신");
        }
        cancel.cancel();
    });
}

fn spawn_watcher(
    path: &Path,
    tx: mpsc::UnboundedSender<()>,
) -> Result<notify_debouncer_mini::Debouncer<notify_debouncer_mini::notify::RecommendedWatcher>>
{
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("daytrade.toml 의 부모 디렉토리 없음: {}", path.display()))?
        .to_path_buf();
    if !parent.exists() {
        std::fs::create_dir_all(&parent)
            .with_context(|| format!("config dir 생성 실패: {}", parent.display()))?;
    }
    // 부모 디렉토리를 watch (파일 자체가 다시 생성될 수도 있음 — 에디터의 atomic write 패턴 대응).
    let path_filter = path.to_path_buf();
    let mut debouncer = new_debouncer(
        Duration::from_millis(500),
        move |res: Result<Vec<DebouncedEvent>, notify_debouncer_mini::notify::Error>| match res {
            Ok(events) => {
                if events.iter().any(|e| e.path == path_filter) {
                    let _ = tx.send(());
                }
            }
            Err(e) => log_error(&format!("watcher 에러: {e}")),
        },
    )
    .context("file watcher 초기화 실패")?;
    debouncer
        .watcher()
        .watch(&parent, RecursiveMode::NonRecursive)
        .with_context(|| format!("디렉토리 watch 실패: {}", parent.display()))?;
    log_info(&format!("watcher 등록 — {}", parent.display()));
    Ok(debouncer)
}

fn apply_diff(
    client: &Arc<KisClient>,
    tasks: &mut HashMap<String, RunningTask>,
    old: &[StrategyEntry],
    new: &[StrategyEntry],
    global_cancel: &CancellationToken,
) {
    let new_ids: std::collections::HashSet<&str> = new.iter().map(|s| s.id.as_str()).collect();
    let old_ids: std::collections::HashSet<&str> = old.iter().map(|s| s.id.as_str()).collect();

    // 삭제된 id → cancel + drop handle
    let to_remove: Vec<String> = tasks
        .keys()
        .filter(|id| !new_ids.contains(id.as_str()))
        .cloned()
        .collect();
    for id in to_remove {
        log_info(&format!("strategy 제거: {}", short(&id)));
        if let Some(task) = tasks.remove(&id) {
            task.cancel.cancel();
            // join 은 background — 즉시 차단되지 않게.
            tokio::spawn(async move {
                let _ = task.handle.await;
            });
        }
    }

    // 신규 id → spawn (변경된 id 는 sliently 무시 — 사용자가 rm+add 로 명시)
    for entry in new {
        if tasks.contains_key(&entry.id) {
            continue;
        }
        if !old_ids.contains(entry.id.as_str()) {
            log_info(&format!(
                "strategy 추가: {} {} {} {} ({})",
                short(&entry.id),
                entry.mode.as_str(),
                entry.kind.as_str(),
                entry.code,
                entry.display_name,
            ));
        }
        match spawn_strategy(client.clone(), entry, global_cancel) {
            Ok(task) => {
                tasks.insert(entry.id.clone(), task);
            }
            Err(e) => log_error(&format!(
                "strategy spawn 실패 ({}): {e}",
                short(&entry.id)
            )),
        }
    }
}

fn spawn_strategy(
    client: Arc<KisClient>,
    entry: &StrategyEntry,
    global_cancel: &CancellationToken,
) -> Result<RunningTask> {
    let entry = entry.clone();
    let task_cancel = global_cancel.child_token();
    let cancel_for_task = task_cancel.clone();
    let id_short = short(&entry.id);

    let engine_cfg = build_engine_config(&entry)?;
    let usa = entry.is_usa();
    let mode = entry.mode;

    // sym_market을 LiveExecutor 가 필요로 함 — pre_resolved 에서 꺼냄.
    let sym_market = engine_cfg
        .pre_resolved
        .as_ref()
        .map(|s| s.market)
        .ok_or_else(|| anyhow!("pre_resolved 없음 (내부 오류)"))?;

    let entry_for_task = entry.clone();
    let handle = tokio::spawn(async move {
        let label = format!(
            "[{}] {} {}",
            id_short,
            entry_for_task.mode.as_str(),
            entry_for_task.code
        );
        let result = match mode {
            ExecMode::Paper => {
                let executor = PaperExecutor {
                    slippage_bps: entry_for_task.slippage_bps,
                };
                engine::run(client, engine_cfg, executor, cancel_for_task).await
            }
            ExecMode::Run => {
                let _ = usa;
                let mut executor = LiveExecutor::new(client.clone(), sym_market);
                executor.tick_offset = entry_for_task.tick_offset;
                executor.fill_timeout_secs = entry_for_task.fill_timeout_secs;
                executor.poll_interval_secs = entry_for_task.poll_interval_secs;
                engine::run(client, engine_cfg, executor, cancel_for_task).await
            }
        };
        match result {
            Ok(()) => log_info(&format!("{} 정상 종료", label)),
            Err(e) => log_error(&format!("{} 종료: {e}", label)),
        }
    });

    Ok(RunningTask {
        cancel: task_cancel,
        handle,
    })
}

fn build_composite(entry: &StrategyEntry) -> Result<Option<CompositeConfig>> {
    if !entry.kind.is_composite() {
        return Ok(None);
    }
    if entry.children.is_empty() {
        return Err(anyhow!(
            "kind=composite 인데 children 비어있음 (id={})",
            short(&entry.id)
        ));
    }
    let combinator = match entry.combinator {
        Some(dconfig::Combinator::And) => Combinator::And,
        Some(dconfig::Combinator::Or) => Combinator::Or,
        None => Combinator::And, // 디폴트
    };
    let children = entry
        .children
        .iter()
        .map(|c| {
            if c.kind.is_composite() {
                Err(anyhow!("nested composite 미지원"))
            } else {
                Ok(CompositeChild {
                    strategy: c.kind,
                    fast: c.fast,
                    slow: c.slow,
                    rsi_period: c.rsi_period,
                    rsi_oversold: c.rsi_oversold,
                    rsi_overbought: c.rsi_overbought,
                    bb_period: c.bb_period,
                    bb_sigma: c.bb_sigma,
                    obv_period: c.obv_period,
                })
            }
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Some(CompositeConfig {
        combinator,
        children,
    }))
}

fn build_engine_config(entry: &StrategyEntry) -> Result<EngineConfig> {
    let market = symbols::Market::from_str(&entry.market)
        .ok_or_else(|| anyhow!("알 수 없는 market: '{}'", entry.market))?;
    let pre_resolved = ResolvedSymbol {
        code: entry.code.clone(),
        market,
        name_kr: entry.display_name.clone(),
        name_en: String::new(),
    };
    let usa = entry.is_usa();
    let period = Period::from_str(&entry.period)
        .with_context(|| format!("period 파싱 실패: '{}'", entry.period))?;

    // run 모드 안전 디폴트 (기존 unpack_daytrade_run 과 동일):
    let (sl_pct, tp_pct) = match entry.mode {
        ExecMode::Run => (
            Some(entry.stop_loss_pct.unwrap_or(2.0)),
            Some(entry.take_profit_pct.unwrap_or(5.0)),
        ),
        ExecMode::Paper => (entry.stop_loss_pct, entry.take_profit_pct),
    };

    let composite = build_composite(entry)?;

    Ok(EngineConfig {
        symbol: entry.code.clone(),
        pre_resolved: Some(pre_resolved),
        strategy: entry.kind,
        composite,
        period,
        usa,
        pick: None,
        qty: entry.qty,
        budget: entry.budget,
        fee_bps: entry.fee_bps,
        stop_loss_pct: sl_pct,
        take_profit_pct: tp_pct,
        stop_loss_atr: entry.stop_loss_atr,
        take_profit_atr: entry.take_profit_atr,
        atr_period: entry.atr_period,
        fast: entry.fast,
        slow: entry.slow,
        rsi_period: entry.rsi_period,
        rsi_oversold: entry.rsi_oversold,
        rsi_overbought: entry.rsi_overbought,
        bb_period: entry.bb_period,
        bb_sigma: entry.bb_sigma,
        obv_period: entry.obv_period,
    })
}

fn short(id: &str) -> String {
    id.chars().take(8).collect()
}

// ResolveMode 의 의미 없는 사용 막아주는 헬퍼 (compile error 회피용 import 보존)
#[allow(dead_code)]
fn _resolve_mode_keep_alive() -> ResolveMode {
    ResolveMode::Domestic
}

// StrategyKind import keep-alive
#[allow(dead_code)]
fn _strategy_kind_keep_alive() -> StrategyKind {
    StrategyKind::Rsi
}

fn log_info(msg: &str) {
    eprintln!(
        "[{}] daemon: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    );
}

fn log_error(msg: &str) {
    eprintln!(
        "[{}] daemon ERROR: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    );
}
