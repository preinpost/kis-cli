//! `kis install` / `kis update` — 바이너리 + Claude 스킬 통합 설치/갱신.
//!
//! - `install`: `cargo install --path <REPO>` 로 `~/.cargo/bin/kis` 배포 + SKILL.md 배포
//! - `update`:  `git pull --ff-only` + `cargo install --force` + SKILL.md 덮어쓰기
//!
//! 소스 경로는 빌드 시점의 `CARGO_MANIFEST_DIR` 를 그대로 사용한다
//! (바이너리는 사용자가 직접 빌드한 것이므로 해당 경로가 곧 로컬 리포).

use std::process::Command;

use anyhow::{bail, Context, Result};

use super::skill;

const REPO_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn run_install(force: bool) -> Result<()> {
    println!("▶ 바이너리 설치 (cargo install --path {REPO_DIR})");
    cargo_install(force)?;
    println!();
    println!("▶ Claude 스킬 설치");
    skill::run_install(force)?;
    println!();
    println!("✓ 설치 완료. 새 터미널에서 `kis --version` 으로 확인.");
    Ok(())
}

pub fn run_update(no_pull: bool) -> Result<()> {
    if !no_pull {
        println!("▶ git pull --ff-only (in {REPO_DIR})");
        git_pull()?;
        println!();
    }
    println!("▶ 바이너리 재빌드 + 설치 (cargo install --force --path {REPO_DIR})");
    cargo_install(true)?;
    println!();
    println!("▶ Claude 스킬 갱신");
    skill::run_install(true)?;
    println!();
    println!("✓ 업데이트 완료.");
    Ok(())
}

fn cargo_install(force: bool) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("install").arg("--path").arg(REPO_DIR).arg("--locked");
    if force {
        cmd.arg("--force");
    }
    let status = cmd
        .status()
        .with_context(|| "cargo 실행 실패 — cargo 가 PATH 에 있어야 합니다")?;
    if !status.success() {
        bail!("cargo install 실패 (exit {:?})", status.code());
    }
    Ok(())
}

fn git_pull() -> Result<()> {
    let status = Command::new("git")
        .arg("-C")
        .arg(REPO_DIR)
        .arg("pull")
        .arg("--ff-only")
        .status()
        .with_context(|| "git 실행 실패 — git 이 PATH 에 있어야 합니다")?;
    if !status.success() {
        bail!(
            "git pull 실패 (exit {:?}) — 로컬 변경사항으로 재빌드만 하려면 --no-pull",
            status.code()
        );
    }
    Ok(())
}
