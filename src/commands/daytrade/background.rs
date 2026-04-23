//! `daytrade paper|run --background` — systemd service 등록 (Linux 전용).
//!
//! 서비스명: `kis-daytrade-{paper|run}-{strategy}-{code}[-usa]`.
//! ExecStart는 현재 프로세스의 `std::env::args()` 를 재사용하되 `--background` 제거,
//! `run` 모드는 systemd 환경에 stdin이 없으므로 `--yes` 자동 주입.

use anyhow::{anyhow, Result};

const UNIT_DIR: &str = "/etc/systemd/system";
const UNIT_PREFIX: &str = "kis-daytrade-";

pub struct UnitSpec<'a> {
    pub mode: &'a str,        // "paper" or "run"
    pub strategy: &'a str,    // "rsi" / "macd" / ...
    pub code: &'a str,        // 종목코드 (resolve 전 원본 심볼이 아니라 resolved 코드)
    pub display_name: &'a str,
    pub usa: bool,
}

/// 현 프로세스 CLI args 를 기반으로 systemd unit 생성 + enable.
/// Linux 외 OS 에서는 unit 파일 내용만 출력.
pub fn install_unit(spec: &UnitSpec) -> Result<()> {
    let run_user = std::env::var("SUDO_USER")
        .ok()
        .or_else(|| std::env::var("USER").ok())
        .ok_or_else(|| anyhow!("$USER 를 읽을 수 없습니다"))?;

    let exe = std::env::current_exe()?;
    let sub_args = build_exec_args(spec.mode);
    let exec_start = format!(
        "{} {}",
        shell_escape(&exe.to_string_lossy()),
        sub_args.iter().map(|a| shell_escape(a)).collect::<Vec<_>>().join(" "),
    );

    let service_name = format!(
        "kis-daytrade-{}-{}-{}{}",
        spec.mode,
        spec.strategy,
        spec.code.to_lowercase(),
        if spec.usa { "-usa" } else { "" },
    );

    let unit = format!(
        "[Unit]\n\
         Description=kis-cli daytrade {mode} {strategy} — {display} ({code})\n\
         After=network-online.target\n\
         Wants=network-online.target\n\
         \n\
         [Service]\n\
         Type=simple\n\
         User={run_user}\n\
         Group={run_user}\n\
         ExecStart={exec_start}\n\
         Restart=on-failure\n\
         RestartSec=30\n\
         \n\
         [Install]\n\
         WantedBy=multi-user.target\n",
        mode = spec.mode,
        strategy = spec.strategy,
        display = spec.display_name,
        code = spec.code,
    );

    let unit_path = format!("{}/{}.service", UNIT_DIR, service_name);

    if !cfg!(target_os = "linux") {
        eprintln!("─────────────────────────────────────────────");
        eprintln!("⚠ systemd는 Linux 전용입니다. 아래 unit 파일을 VPS에 복사하세요.");
        eprintln!("파일 경로: {}", unit_path);
        eprintln!("─────────────────────────────────────────────");
        eprint!("{}", unit);
        eprintln!("─────────────────────────────────────────────");
        eprintln!("⚠ ExecStart 의 바이너리 경로는 *현재 로컬 경로* 입니다. VPS 에서는 `which kis` 로 교체하세요.");
        eprintln!();
        eprintln!("설치 절차 (VPS 에서, root 또는 sudo):");
        eprintln!("  sudo tee {} > /dev/null <<'EOF'", unit_path);
        eprint!("{}", unit);
        eprintln!("EOF");
        eprintln!("  sudo systemctl daemon-reload");
        eprintln!("  sudo systemctl enable --now {}", service_name);
        eprintln!("  sudo journalctl -u {} -f", service_name);
        eprintln!();
        eprintln!("💡 VPS 에서 직접 실행이 편합니다:");
        eprintln!("  sudo $(which kis) daytrade {} ... --background", spec.mode);
        return Ok(());
    }

    match std::fs::write(&unit_path, &unit) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!(
                "{} 쓰기 권한이 없습니다. 재실행:\n  sudo $(which kis) daytrade {} ... --background",
                unit_path, spec.mode,
            ));
        }
        Err(e) => return Err(anyhow!("{} 쓰기 실패: {e}", unit_path)),
    }
    eprintln!("[background] systemd unit 생성: {}", unit_path);
    run_systemctl(&["daemon-reload"])?;
    run_systemctl(&["enable", "--now", &service_name])?;
    eprintln!("[background] ✓ {}.service 활성화 (실행 유저: {})", service_name, run_user);
    eprintln!();
    eprintln!("ExecStart: {}", exec_start);
    eprintln!("로그: sudo journalctl -u {} -f", service_name);
    eprintln!("상태: sudo systemctl status {}", service_name);
    eprintln!("제거: sudo systemctl disable --now {} && sudo rm {}", service_name, unit_path);
    Ok(())
}

/// `std::env::args()` 에서 첫 원소(실행 파일 경로) 제거, `--background` 제거,
/// `run` 모드는 `--yes` 자동 주입. 결과는 exe 뒤에 붙일 sub-args.
fn build_exec_args(mode: &str) -> Vec<String> {
    let mut sub: Vec<String> = std::env::args()
        .skip(1)
        .filter(|a| a.as_str() != "--background")
        .collect();
    if mode == "run" && !sub.iter().any(|a| a == "--yes") {
        sub.push("--yes".into());
    }
    sub
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

fn shell_escape(s: &str) -> String {
    if s.chars().all(|c| c.is_ascii_alphanumeric() || "/._-".contains(c)) {
        s.to_string()
    } else {
        format!("\"{}\"", s.replace('"', "\\\""))
    }
}

// ─────────────────────────────────────────────────────────────
// `kis daytrade list` / `kis daytrade remove <target>`
// ─────────────────────────────────────────────────────────────

pub fn list_services() -> Result<()> {
    let dir = std::path::Path::new(UNIT_DIR);
    if !dir.exists() {
        println!("(등록된 서비스 없음 — {} 가 없습니다. Linux 전용)", UNIT_DIR);
        return Ok(());
    }
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(UNIT_PREFIX) && n.ends_with(".service"))
                .unwrap_or(false)
        })
        .collect();
    if files.is_empty() {
        println!("(등록된 kis-daytrade 서비스 없음)");
        return Ok(());
    }
    files.sort();

    for path in files {
        let service_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let description = extract_field(&content, "Description=").unwrap_or_default();
        let exec_start = extract_field(&content, "ExecStart=").unwrap_or_default();
        let run_user = extract_field(&content, "User=").unwrap_or_default();
        let active = systemctl_query(&["is-active", &service_name]);
        let enabled = systemctl_query(&["is-enabled", &service_name]);

        println!("● {}.service", service_name);
        if !description.is_empty() {
            println!("    Description: {}", description);
        }
        println!("    Status:      active={} / enabled={} / user={}", active, enabled, run_user);
        if !exec_start.is_empty() {
            println!("    ExecStart:   {}", exec_start);
        }
        println!("    Unit:        {}", path.display());
        println!();
    }
    println!("제거: sudo $(which kis) daytrade remove <target>");
    println!("로그: sudo journalctl -u <service-name> -f");
    Ok(())
}

/// 등록된 모든 `kis-daytrade-*` 서비스를 찾아 `disable --now` 후 unit 파일 삭제.
/// `--yes` 없이 TTY 면 y/N 프롬프트, 비-TTY 면 에러.
pub fn remove_all_services(yes: bool) -> Result<()> {
    let dir = std::path::Path::new(UNIT_DIR);
    if !dir.exists() {
        return Err(anyhow!("{} 가 없습니다 (Linux 전용)", UNIT_DIR));
    }
    let services: Vec<String> = std::fs::read_dir(dir)?
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.starts_with(UNIT_PREFIX) && n.ends_with(".service"))
        .map(|n| n.trim_end_matches(".service").to_string())
        .collect();

    if services.is_empty() {
        eprintln!("(등록된 kis-daytrade 서비스 없음)");
        return Ok(());
    }

    eprintln!("다음 서비스가 모두 제거됩니다 ({} 개):", services.len());
    for s in &services {
        eprintln!("  - {}.service", s);
    }

    if !yes {
        if !is_tty() {
            return Err(anyhow!(
                "비-TTY 환경 — 확인 건너뛰려면 `--yes` 를 추가하세요."
            ));
        }
        eprint!("정말 모두 제거하시겠습니까? [y/N]: ");
        use std::io::Write;
        std::io::stderr().flush().ok();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;
        let ans = line.trim().to_lowercase();
        if ans != "y" && ans != "yes" {
            eprintln!("취소됨.");
            return Ok(());
        }
    }

    let mut ok = 0usize;
    let mut failed: Vec<(String, String)> = Vec::new();
    for service_name in &services {
        let unit_path = format!("{}/{}.service", UNIT_DIR, service_name);
        if let Err(e) = run_systemctl(&["disable", "--now", service_name]) {
            eprintln!("[background] {} disable --now 실패 (무시하고 파일 삭제 시도): {e}", service_name);
        }
        match std::fs::remove_file(&unit_path) {
            Ok(()) => ok += 1,
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                return Err(anyhow!(
                    "{} 삭제 권한이 없습니다. 재실행:\n  sudo $(which kis) daytrade remove --all{}",
                    unit_path,
                    if yes { " --yes" } else { "" },
                ));
            }
            Err(e) => failed.push((service_name.clone(), format!("{e}"))),
        }
    }

    run_systemctl(&["daemon-reload"])?;
    eprintln!("[background] ✓ {} 개 제거 완료", ok);
    for (name, err) in &failed {
        eprintln!("[background] ✗ {} 실패: {}", name, err);
    }
    if !failed.is_empty() {
        return Err(anyhow!("{} 개 서비스 제거 실패", failed.len()));
    }
    Ok(())
}

fn is_tty() -> bool {
    unsafe extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(0) == 1 }
}

pub fn remove_service(target: &str) -> Result<()> {
    let service_name = resolve_service_name(target)?;
    let unit_path = format!("{}/{}.service", UNIT_DIR, service_name);

    if !std::path::Path::new(&unit_path).exists() {
        return Err(anyhow!(
            "서비스 파일이 없습니다: {}\n`kis daytrade list` 로 등록된 서비스를 확인하세요.",
            unit_path
        ));
    }

    if let Err(e) = run_systemctl(&["disable", "--now", &service_name]) {
        eprintln!("[background] disable --now 실패 (무시하고 파일 삭제 시도): {e}");
    }

    match std::fs::remove_file(&unit_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!(
                "{} 삭제 권한이 없습니다. 재실행:\n  sudo $(which kis) daytrade remove {}",
                unit_path, target
            ));
        }
        Err(e) => return Err(anyhow!("{} 삭제 실패: {e}", unit_path)),
    }

    run_systemctl(&["daemon-reload"])?;
    eprintln!("[background] ✓ {}.service 제거됨 ({} 삭제)", service_name, unit_path);
    Ok(())
}

/// target 해석:
/// - 전체 서비스명 (`kis-daytrade-paper-rsi-tsla-usa`, `.service` 접미사 무관) → 그대로
/// - 부분 문자열 (예: `tsla`) → `UNIT_PREFIX` 로 시작하는 unit 중 부분 일치. 유일하면 OK, 여러 개면 에러.
fn resolve_service_name(target: &str) -> Result<String> {
    let stripped = target.strip_suffix(".service").unwrap_or(target);
    if stripped.starts_with(UNIT_PREFIX) {
        return Ok(stripped.to_string());
    }
    let needle = stripped.to_lowercase();
    let dir = std::path::Path::new(UNIT_DIR);
    if !dir.exists() {
        return Err(anyhow!("{} 가 없습니다 (Linux 전용)", UNIT_DIR));
    }
    let matches: Vec<String> = std::fs::read_dir(dir)?
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| n.starts_with(UNIT_PREFIX) && n.ends_with(".service"))
        .filter(|n| n.contains(&needle))
        .map(|n| n.trim_end_matches(".service").to_string())
        .collect();
    match matches.len() {
        0 => Err(anyhow!(
            "'{}' 에 매칭되는 daytrade 서비스 없음. `kis daytrade list` 로 확인하세요.",
            target
        )),
        1 => Ok(matches.into_iter().next().unwrap()),
        _ => Err(anyhow!(
            "여러 서비스가 매칭됨: {:?}\n전체 서비스명을 지정해주세요 (예: `kis daytrade remove {}`)",
            matches, matches[0]
        )),
    }
}

fn extract_field(content: &str, prefix: &str) -> Option<String> {
    content
        .lines()
        .find(|l| l.starts_with(prefix))
        .map(|l| l[prefix.len()..].trim().to_string())
}

fn systemctl_query(args: &[&str]) -> String {
    std::process::Command::new("systemctl")
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}
