//! `daytrade add/rm/list/start/stop/status/legacy-clean` — toml 편집과 단일 데몬 라이프사이클.
//!
//! 데몬 자체 로직은 [`super::daemon`]. 여기는 CLI 표층 + systemd unit 관리만.

use std::path::Path;

use anyhow::{anyhow, Context, Result};

use super::dconfig::{
    self, duplicate_summary, min_distinguishing_prefix, new_id, short_id, ChildStrategyEntry,
    Combinator as DcCombinator, DaytradeConfig, ExecMode, StrategyEntry,
};
use super::engine::{Combinator as EngineCombinator, CompositeConfig};
use super::paper;
use super::run as run_mod;
use crate::commands::helpers::resolve_symbol;
use crate::symbols::ResolveMode;

const DAEMON_UNIT_NAME: &str = "kis-daytrade";
const DAEMON_UNIT_PATH: &str = "/etc/systemd/system/kis-daytrade.service";
const LEGACY_UNIT_PREFIX: &str = "kis-daytrade-";

// ─────────────────────────────────────────────────────────────────────
// add: paper / run config → resolve symbol → toml append
// ─────────────────────────────────────────────────────────────────────

pub fn add_paper(cfg: paper::Config) -> Result<()> {
    let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, mode, cfg.pick)?;
    let display_name = if !sym.name_kr.is_empty() {
        sym.name_kr.clone()
    } else if !sym.name_en.is_empty() {
        sym.name_en.clone()
    } else {
        sym.code.clone()
    };
    let (combinator, children) = composite_to_toml(&cfg.composite);
    let entry = StrategyEntry {
        id: new_id(),
        mode: ExecMode::Paper,
        kind: cfg.strategy,
        code: sym.code.clone(),
        market: sym.market.as_str().to_string(),
        display_name,
        period: cfg.period.label(),
        qty: cfg.qty,
        budget: cfg.budget,
        fee_bps: cfg.fee_bps,
        slippage_bps: cfg.slippage_bps,
        stop_loss_pct: cfg.stop_loss_pct,
        take_profit_pct: cfg.take_profit_pct,
        stop_loss_atr: cfg.stop_loss_atr,
        take_profit_atr: cfg.take_profit_atr,
        atr_period: cfg.atr_period,
        fast: cfg.fast,
        slow: cfg.slow,
        rsi_period: cfg.rsi_period,
        rsi_oversold: cfg.rsi_oversold,
        rsi_overbought: cfg.rsi_overbought,
        bb_period: cfg.bb_period,
        bb_sigma: cfg.bb_sigma,
        obv_period: cfg.obv_period,
        combinator,
        children,
        tick_offset: 0,
        fill_timeout_secs: 30,
        poll_interval_secs: 2,
    };
    append_entry(entry)
}

pub fn add_run(cfg: run_mod::Config) -> Result<()> {
    let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, mode, cfg.pick)?;
    let display_name = if !sym.name_kr.is_empty() {
        sym.name_kr.clone()
    } else if !sym.name_en.is_empty() {
        sym.name_en.clone()
    } else {
        sym.code.clone()
    };
    let (combinator, children) = composite_to_toml(&cfg.composite);
    let entry = StrategyEntry {
        id: new_id(),
        mode: ExecMode::Run,
        kind: cfg.strategy,
        code: sym.code.clone(),
        market: sym.market.as_str().to_string(),
        display_name,
        period: cfg.period.label(),
        qty: cfg.qty,
        budget: cfg.budget,
        fee_bps: cfg.fee_bps,
        slippage_bps: 0.0,
        stop_loss_pct: cfg.stop_loss_pct,
        take_profit_pct: cfg.take_profit_pct,
        stop_loss_atr: cfg.stop_loss_atr,
        take_profit_atr: cfg.take_profit_atr,
        atr_period: cfg.atr_period,
        fast: cfg.fast,
        slow: cfg.slow,
        rsi_period: cfg.rsi_period,
        rsi_oversold: cfg.rsi_oversold,
        rsi_overbought: cfg.rsi_overbought,
        bb_period: cfg.bb_period,
        bb_sigma: cfg.bb_sigma,
        obv_period: cfg.obv_period,
        combinator,
        children,
        tick_offset: cfg.tick_offset,
        fill_timeout_secs: cfg.fill_timeout_secs,
        poll_interval_secs: cfg.poll_interval_secs,
    };
    append_entry(entry)
}

fn composite_to_toml(c: &Option<CompositeConfig>) -> (Option<DcCombinator>, Vec<ChildStrategyEntry>) {
    match c {
        None => (None, Vec::new()),
        Some(cc) => {
            let combinator = match cc.combinator {
                EngineCombinator::And => Some(DcCombinator::And),
                EngineCombinator::Or => Some(DcCombinator::Or),
            };
            let children = cc
                .children
                .iter()
                .map(|child| ChildStrategyEntry {
                    kind: child.strategy,
                    fast: child.fast,
                    slow: child.slow,
                    rsi_period: child.rsi_period,
                    rsi_oversold: child.rsi_oversold,
                    rsi_overbought: child.rsi_overbought,
                    bb_period: child.bb_period,
                    bb_sigma: child.bb_sigma,
                    obv_period: child.obv_period,
                })
                .collect();
            (combinator, children)
        }
    }
}

fn append_entry(entry: StrategyEntry) -> Result<()> {
    let mut cfg = DaytradeConfig::load()?;
    let dups = duplicate_summary(&cfg, &entry);
    cfg.add(entry.clone());
    cfg.save()?;
    println!(
        "✓ 추가됨 — id={} mode={} kind={} {} ({}) [{}]",
        short_id(&entry.id),
        entry.mode.as_str(),
        entry.kind.as_str(),
        entry.code,
        entry.display_name,
        entry.market,
    );
    println!("  파일: {}", dconfig::config_path()?.display());
    if !dups.is_empty() {
        println!(
            "  ⚠ 동일 (mode/kind/code) strategy 가 이미 있음 — id: {}",
            dups.join(", ")
        );
        println!("    파라미터만 다른 변형이라면 의도된 것일 수 있음. 중복이면 `kis daytrade rm <id>` 로 제거.");
    }
    println!();
    println!("데몬이 실행 중이면 자동 반영됩니다. 처음이면: `sudo $(which kis) daytrade start`");
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// rm: toml에서 id 일치 항목 제거
// ─────────────────────────────────────────────────────────────────────

pub fn remove(id_or_substring: &str) -> Result<()> {
    let mut cfg = DaytradeConfig::load()?;
    let removed = cfg.remove(id_or_substring)?;
    cfg.save()?;
    println!(
        "✓ 제거됨 — id={} mode={} kind={} {} ({}) [{}]",
        short_id(&removed.id),
        removed.mode.as_str(),
        removed.kind.as_str(),
        removed.code,
        removed.display_name,
        removed.market,
    );
    println!("  데몬이 실행 중이면 해당 strategy task가 곧 종료됩니다.");
    Ok(())
}

pub fn remove_all(yes: bool) -> Result<()> {
    let mut cfg = DaytradeConfig::load()?;
    if cfg.strategies.is_empty() {
        println!("(daytrade.toml 에 등록된 strategy 없음 — 제거할 게 없음)");
        return Ok(());
    }
    let n = cfg.strategies.len();
    eprintln!("아래 {}개 strategy가 daytrade.toml 에서 제거됩니다:", n);
    for s in &cfg.strategies {
        eprintln!(
            "  - {} {} {} {} ({})",
            short_id(&s.id),
            s.mode.as_str(),
            s.kind.as_str(),
            s.code,
            s.display_name,
        );
    }
    if !yes {
        if !is_tty() {
            return Err(anyhow!("비-TTY 환경 — `--yes` 필수"));
        }
        eprint!("진행하시겠습니까? [y/N]: ");
        use std::io::Write;
        std::io::stderr().flush().ok();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        if !matches!(line.trim().to_lowercase().as_str(), "y" | "yes") {
            eprintln!("취소됨.");
            return Ok(());
        }
    }
    cfg.strategies.clear();
    cfg.save()?;
    println!("✓ {}개 strategy 제거됨. 데몬이 실행 중이면 모든 task가 곧 종료됩니다.", n);
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// list: toml 항목 표시 + (있으면) legacy 서비스 경고
// ─────────────────────────────────────────────────────────────────────

pub fn list() -> Result<()> {
    let cfg = DaytradeConfig::load()?;
    if cfg.strategies.is_empty() {
        println!("(daytrade.toml 에 등록된 strategy 없음)");
        println!("  추가: `kis daytrade add paper rsi <symbol> --qty 1 --budget 1000000 ...`");
    } else {
        println!("등록된 strategy ({}):", cfg.strategies.len());
        let ids: Vec<&str> = cfg.strategies.iter().map(|s| s.id.as_str()).collect();
        let id_len = min_distinguishing_prefix(&ids);
        let id_w = id_len.max(8);
        println!(
            "  {:<id_w$} {:<6} {:<10} {:<8} {:<7} {:<5} {:<13} {:<7} {}",
            "id", "mode", "kind", "code", "market", "qty", "budget", "period", "name",
            id_w = id_w,
        );
        for s in &cfg.strategies {
            let id_short: String = s.id.chars().take(id_len).collect();
            println!(
                "  {:<id_w$} {:<6} {:<10} {:<8} {:<7} {:<5} {:<13} {:<7} {}",
                id_short,
                s.mode.as_str(),
                s.kind.as_str(),
                s.code,
                s.market,
                s.qty,
                fmt_money(s.budget),
                s.period,
                s.display_name,
            );
        }
        println!();
        println!("  파일: {}", dconfig::config_path()?.display());
    }

    let legacy = list_legacy_units();
    if !legacy.is_empty() {
        println!();
        println!(
            "⚠ 기존 per-strategy systemd 서비스 {}개 감지 — 단일 데몬 모델로 마이그레이션 권장:",
            legacy.len()
        );
        for s in &legacy {
            println!("    - {}.service", s);
        }
        println!("  정리: `sudo $(which kis) daytrade legacy-clean`");
    }
    Ok(())
}

fn fmt_money(v: f64) -> String {
    if v >= 1_000_000.0 {
        format!("{:.2}m", v / 1_000_000.0)
    } else if v >= 1_000.0 {
        format!("{:.1}k", v / 1_000.0)
    } else {
        format!("{:.0}", v)
    }
}

// ─────────────────────────────────────────────────────────────────────
// start / stop / status — 단일 unit (`kis-daytrade.service`) 관리
// ─────────────────────────────────────────────────────────────────────

pub fn start() -> Result<()> {
    if !cfg!(target_os = "linux") {
        return print_unit_for_manual_install();
    }

    let exe = std::env::current_exe().context("현재 바이너리 경로 조회 실패")?;
    let run_user = std::env::var("SUDO_USER")
        .ok()
        .or_else(|| std::env::var("USER").ok())
        .ok_or_else(|| anyhow!("$USER 를 읽을 수 없습니다"))?;

    let unit = render_unit(&exe.to_string_lossy(), &run_user);
    write_unit(DAEMON_UNIT_PATH, &unit)?;
    eprintln!("[start] systemd unit 작성: {}", DAEMON_UNIT_PATH);
    if let Err(e) = ensure_log_dir(&run_user) {
        eprintln!("[start] ⚠ /var/log/kis-cli 준비 실패 ({e}) — 데몬은 사용자 폴백 경로로 전환합니다");
    }
    run_systemctl(&["daemon-reload"])?;
    run_systemctl(&["enable", "--now", DAEMON_UNIT_NAME])?;
    eprintln!("[start] ✓ {}.service 활성화 (실행 유저: {})", DAEMON_UNIT_NAME, run_user);
    eprintln!();
    eprintln!("로그:   kis daytrade logs -f");
    eprintln!("상태:   kis daytrade status");
    eprintln!("중지:   sudo $(which kis) daytrade stop");

    let legacy = list_legacy_units();
    if !legacy.is_empty() {
        eprintln!();
        eprintln!(
            "⚠ 기존 per-strategy 서비스 {}개 감지 — `daytrade legacy-clean` 으로 정리하세요.",
            legacy.len()
        );
    }
    Ok(())
}

pub fn restart() -> Result<()> {
    if !cfg!(target_os = "linux") {
        return Err(anyhow!("systemd는 Linux 전용입니다."));
    }
    if !Path::new(DAEMON_UNIT_PATH).exists() {
        return Err(anyhow!(
            "{} 가 설치돼 있지 않습니다 — `sudo $(which kis) daytrade start` 로 먼저 시작하세요",
            DAEMON_UNIT_PATH
        ));
    }
    run_systemctl(&["restart", DAEMON_UNIT_NAME])?;
    eprintln!("[restart] ✓ {}.service 재시작", DAEMON_UNIT_NAME);
    eprintln!("로그:   kis daytrade logs -f");
    Ok(())
}

pub fn stop() -> Result<()> {
    if !cfg!(target_os = "linux") {
        return Err(anyhow!("systemd는 Linux 전용입니다."));
    }
    if !Path::new(DAEMON_UNIT_PATH).exists() {
        eprintln!("(이미 제거됨 — {} 가 없습니다)", DAEMON_UNIT_PATH);
        return Ok(());
    }
    // disable --now 가 stop + disable 동시 수행
    let _ = run_systemctl(&["disable", "--now", DAEMON_UNIT_NAME]);
    match std::fs::remove_file(DAEMON_UNIT_PATH) {
        Ok(()) => eprintln!("[stop] unit 파일 삭제: {}", DAEMON_UNIT_PATH),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!(
                "{} 삭제 권한 없음 — `sudo $(which kis) daytrade stop` 으로 재시도",
                DAEMON_UNIT_PATH
            ));
        }
        Err(e) => return Err(anyhow!("{} 삭제 실패: {e}", DAEMON_UNIT_PATH)),
    }
    let _ = run_systemctl(&["daemon-reload"]);
    eprintln!("[stop] ✓ {}.service 제거 완료", DAEMON_UNIT_NAME);
    Ok(())
}

pub fn status() -> Result<()> {
    let exists_unit = Path::new(DAEMON_UNIT_PATH).exists();
    let cfg = DaytradeConfig::load()?;

    println!("=== daytrade 데몬 상태 ===");
    println!("  unit:       {}", if exists_unit { DAEMON_UNIT_PATH } else { "(설치 안됨)" });
    if cfg!(target_os = "linux") && exists_unit {
        let active = systemctl_query(&["is-active", DAEMON_UNIT_NAME]);
        let enabled = systemctl_query(&["is-enabled", DAEMON_UNIT_NAME]);
        println!("  active:     {}", active);
        println!("  enabled:    {}", enabled);
        println!();
        println!("  → 로그:   kis daytrade logs -f");
    } else if !cfg!(target_os = "linux") {
        println!("  ※ Linux 외 OS — systemd 미사용. `kis daytrade daemon` 직접 실행.");
    }

    println!();
    println!("=== 등록된 strategy ({}) ===", cfg.strategies.len());
    if cfg.strategies.is_empty() {
        println!("  (없음 — `kis daytrade add ...` 로 추가)");
    } else {
        let ids: Vec<&str> = cfg.strategies.iter().map(|s| s.id.as_str()).collect();
        let id_len = min_distinguishing_prefix(&ids);
        for s in &cfg.strategies {
            let id_short: String = s.id.chars().take(id_len).collect();
            println!(
                "  {} {} {} {} ({}) qty={} budget={}",
                id_short,
                s.mode.as_str(),
                s.kind.as_str(),
                s.code,
                s.display_name,
                s.qty,
                fmt_money(s.budget),
            );
        }
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// logs — 데몬 로그파일 출력 (`tail -n N` / `tail -F`)
// ─────────────────────────────────────────────────────────────────────

pub fn logs(follow: bool, lines: usize, path_only: bool) -> Result<()> {
    let log_path = crate::logging::current_log_path("daytrade")?;

    if path_only {
        println!("{}", log_path.display());
        return Ok(());
    }

    if !log_path.exists() {
        eprintln!("(로그 파일 없음: {})", log_path.display());
        eprintln!("  데몬을 먼저 시작하세요: sudo $(which kis) daytrade start");
        eprintln!("  포그라운드 디버그:       kis daytrade daemon");
        return Ok(());
    }

    // tail이 일별 롤링도 잘 따라가도록 -F (대문자: file rename 추적). macOS/Linux 둘 다 지원.
    let mut cmd = std::process::Command::new("tail");
    if follow {
        cmd.arg("-F").arg("-n").arg(lines.to_string());
    } else {
        cmd.arg("-n").arg(lines.to_string());
    }
    cmd.arg(&log_path);

    let status = cmd
        .status()
        .map_err(|e| anyhow!("tail 실행 실패: {e} (경로: {})", log_path.display()))?;
    if !status.success() && !follow {
        return Err(anyhow!("tail 비정상 종료 (exit {:?})", status.code()));
    }
    Ok(())
}

/// `daytrade start` (sudo) 가 호출 — `/var/log/kis-cli/` 디렉터리 생성 + 데몬유저 chown.
/// 권한 부족이나 이미 존재하면 silent 통과.
fn ensure_log_dir(run_user: &str) -> Result<()> {
    if !cfg!(target_os = "linux") {
        return Ok(());
    }
    let dir = Path::new("/var/log/kis-cli");
    if !dir.exists() {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("{} 생성 실패", dir.display()))?;
    }
    // chown — `chown <user>:<user> /var/log/kis-cli`. 실패해도 fatal은 아니지만 보고는 한다.
    let status = std::process::Command::new("chown")
        .arg(format!("{user}:{user}", user = run_user))
        .arg(dir)
        .status()
        .map_err(|e| anyhow!("chown 실행 실패: {e}"))?;
    if !status.success() {
        return Err(anyhow!("chown {} 실패 (exit {:?})", dir.display(), status.code()));
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// legacy-clean: kis-daytrade-*-* (단일 데몬 unit 제외) 일괄 제거
// ─────────────────────────────────────────────────────────────────────

pub fn legacy_clean(yes: bool) -> Result<()> {
    if !cfg!(target_os = "linux") {
        return Err(anyhow!("systemd는 Linux 전용입니다."));
    }
    let units = list_legacy_units();
    if units.is_empty() {
        eprintln!("(정리할 legacy 서비스 없음)");
        return Ok(());
    }
    eprintln!("아래 {} 개 legacy 서비스가 disable + 삭제됩니다:", units.len());
    for u in &units {
        eprintln!("  - {}.service", u);
    }
    if !yes {
        if !is_tty() {
            return Err(anyhow!("비-TTY 환경 — `--yes` 필수"));
        }
        eprint!("진행하시겠습니까? [y/N]: ");
        use std::io::Write;
        std::io::stderr().flush().ok();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        if !matches!(line.trim().to_lowercase().as_str(), "y" | "yes") {
            eprintln!("취소됨.");
            return Ok(());
        }
    }
    for u in &units {
        let _ = run_systemctl(&["disable", "--now", u]);
        let path = format!("/etc/systemd/system/{}.service", u);
        if let Err(e) = std::fs::remove_file(&path) {
            eprintln!("  ! {} 삭제 실패: {e}", path);
        } else {
            eprintln!("  ✓ {}.service 제거", u);
        }
    }
    let _ = run_systemctl(&["daemon-reload"]);
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// 내부 유틸
// ─────────────────────────────────────────────────────────────────────

fn render_unit(exe: &str, run_user: &str) -> String {
    format!(
        "[Unit]\n\
         Description=kis-cli daytrade daemon (단일 프로세스, daytrade.toml 기반)\n\
         After=network-online.target\n\
         Wants=network-online.target\n\
         \n\
         [Service]\n\
         Type=simple\n\
         User={user}\n\
         Group={user}\n\
         ExecStart={exe} daytrade daemon\n\
         Restart=on-failure\n\
         RestartSec=10\n\
         KillSignal=SIGTERM\n\
         TimeoutStopSec=30\n\
         \n\
         [Install]\n\
         WantedBy=multi-user.target\n",
        user = run_user,
        exe = shell_escape(exe),
    )
}

fn write_unit(path: &str, content: &str) -> Result<()> {
    match std::fs::write(path, content) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Err(anyhow!(
            "{} 쓰기 권한 없음 — `sudo $(which kis) daytrade start` 로 재시도",
            path
        )),
        Err(e) => Err(anyhow!("{} 쓰기 실패: {e}", path)),
    }
}

fn print_unit_for_manual_install() -> Result<()> {
    let exe = std::env::current_exe()?;
    let user = std::env::var("USER").unwrap_or_else(|_| "kis".into());
    let unit = render_unit(&exe.to_string_lossy(), &user);
    eprintln!("─────────────────────────────────────────────");
    eprintln!("⚠ systemd는 Linux 전용입니다. 아래 unit 을 VPS에 복사하세요:");
    eprintln!("─────────────────────────────────────────────");
    eprint!("{}", unit);
    eprintln!("─────────────────────────────────────────────");
    eprintln!("VPS 설치:");
    eprintln!("  sudo tee {} > /dev/null <<'EOF'", DAEMON_UNIT_PATH);
    eprint!("{}", unit);
    eprintln!("EOF");
    eprintln!("  sudo systemctl daemon-reload");
    eprintln!("  sudo systemctl enable --now {}", DAEMON_UNIT_NAME);
    Ok(())
}

fn run_systemctl(args: &[&str]) -> Result<()> {
    let status = std::process::Command::new("systemctl")
        .args(args)
        .status()
        .map_err(|e| anyhow!("systemctl 실행 실패: {e}"))?;
    if !status.success() {
        return Err(anyhow!("systemctl {:?} 실패 (exit {:?})", args, status.code()));
    }
    Ok(())
}

fn systemctl_query(args: &[&str]) -> String {
    let out = std::process::Command::new("systemctl").args(args).output();
    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => "?".into(),
    }
}

fn shell_escape(s: &str) -> String {
    if s.chars().all(|c| c.is_ascii_alphanumeric() || "/._-".contains(c)) {
        s.to_string()
    } else {
        format!("\"{}\"", s.replace('"', "\\\""))
    }
}

fn is_tty() -> bool {
    unsafe extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(0) == 1 }
}

/// `/etc/systemd/system/kis-daytrade-*.service` 중 단일 데몬 unit 제외.
fn list_legacy_units() -> Vec<String> {
    let dir = Path::new("/etc/systemd/system");
    if !dir.exists() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return out,
    };
    for entry in entries.flatten() {
        let name = match entry.file_name().into_string() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if !name.starts_with(LEGACY_UNIT_PREFIX) || !name.ends_with(".service") {
            continue;
        }
        // 단일 데몬 unit(`kis-daytrade.service`) 은 제외
        if name == "kis-daytrade.service" {
            continue;
        }
        out.push(name.trim_end_matches(".service").to_string());
    }
    out.sort();
    out
}

