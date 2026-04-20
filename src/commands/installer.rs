//! `kis install` / `kis update` — 바이너리 + Claude 스킬 통합 설치/갱신.
//!
//! - `install`: `cargo install --path <REPO>` 로 `~/.cargo/bin/kis` 배포 + SKILL.md 배포
//! - `update`:  기본은 GitHub Release 에서 현재 플랫폼 바이너리 다운로드 후 현재 바이너리 자리에
//!              atomic rename 으로 교체한다. `--from-source` 를 주면 기존 경로대로
//!              `git pull --ff-only` + `cargo install --force` 로 로컬 소스에서 재빌드한다.
//!
//! 소스 경로는 빌드 시점의 `CARGO_MANIFEST_DIR` 를 그대로 사용한다
//! (소스 빌드는 사용자가 직접 체크아웃한 리포에서만 의미 있음).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;

use super::skill;

const REPO_DIR: &str = env!("CARGO_MANIFEST_DIR");
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_OWNER: &str = "preinpost";
const REPO_NAME: &str = "kis-cli";

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

pub async fn run_update(from_source: bool, no_pull: bool) -> Result<()> {
    if from_source {
        return run_update_from_source(no_pull);
    }
    update_from_release().await
}

fn run_update_from_source(no_pull: bool) -> Result<()> {
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
    println!("✓ 업데이트 완료 (소스 빌드).");
    Ok(())
}

async fn update_from_release() -> Result<()> {
    println!("▶ 최신 릴리스 조회 ({REPO_OWNER}/{REPO_NAME})");
    let latest = fetch_latest_release().await?;
    let latest_ver = latest.tag_name.trim_start_matches('v').to_string();

    if latest_ver == CURRENT_VERSION {
        println!("✓ 이미 최신 버전입니다 (v{CURRENT_VERSION})");
        return Ok(());
    }
    println!("▶ 업데이트: v{CURRENT_VERSION} → v{latest_ver}");

    let triple = detect_triple()?;
    let asset_name = format!("kis-v{latest_ver}-{triple}.tar.gz");
    let asset = latest
        .assets
        .iter()
        .find(|a| a.name == asset_name)
        .ok_or_else(|| {
            anyhow!(
                "릴리스 asset 을 찾을 수 없음: {asset_name}\n사용 가능: {}",
                latest
                    .assets
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    let tmpdir = std::env::temp_dir().join(format!("kis-update-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmpdir);
    fs::create_dir_all(&tmpdir)
        .with_context(|| format!("임시 디렉토리 생성 실패: {}", tmpdir.display()))?;

    let tarball = tmpdir.join(&asset_name);
    println!("▶ 다운로드 ({} bytes)", asset.size);
    download(&asset.browser_download_url, &tarball).await?;

    println!("▶ 추출");
    extract_tarball(&tarball, &tmpdir)?;

    let staging = format!("kis-v{latest_ver}-{triple}");
    let new_bin = tmpdir.join(&staging).join("kis");
    if !new_bin.exists() {
        bail!("추출된 바이너리를 찾을 수 없음: {}", new_bin.display());
    }

    let current_exe = std::env::current_exe().context("현재 바이너리 경로 확인 실패")?;
    println!("▶ 설치: {}", current_exe.display());
    atomic_replace(&new_bin, &current_exe)?;

    let _ = fs::remove_dir_all(&tmpdir);

    println!();
    println!("▶ Claude 스킬 갱신 (새 버전 바이너리 호출)");
    let status = Command::new(&current_exe)
        .args(["skill", "install", "--force"])
        .status()
        .context("새 바이너리 실행 실패")?;
    if !status.success() {
        eprintln!("⚠ 스킬 갱신 실패 — 직접 `kis skill install --force` 실행하세요.");
    }

    println!();
    println!("✓ 업데이트 완료: v{latest_ver}");
    Ok(())
}

#[derive(Deserialize)]
struct ReleaseResp {
    tag_name: String,
    assets: Vec<ReleaseAsset>,
}

#[derive(Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

async fn fetch_latest_release() -> Result<ReleaseResp> {
    let url = format!("https://api.github.com/repos/{REPO_OWNER}/{REPO_NAME}/releases/latest");
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", concat!("kis-cli/", env!("CARGO_PKG_VERSION")))
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .context("GitHub API 호출 실패")?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        bail!("GitHub API 응답 실패: {status} — {body}");
    }
    resp.json::<ReleaseResp>()
        .await
        .context("응답 파싱 실패")
}

async fn download(url: &str, dest: &Path) -> Result<()> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("User-Agent", concat!("kis-cli/", env!("CARGO_PKG_VERSION")))
        .send()
        .await
        .context("다운로드 실패")?;
    let status = resp.status();
    if !status.is_success() {
        bail!("다운로드 응답 실패: {status}");
    }
    let bytes = resp.bytes().await.context("바디 수신 실패")?;
    fs::write(dest, &bytes).with_context(|| format!("파일 쓰기 실패: {}", dest.display()))?;
    Ok(())
}

fn extract_tarball(tarball: &Path, out_dir: &Path) -> Result<()> {
    let status = Command::new("tar")
        .arg("xzf")
        .arg(tarball)
        .arg("-C")
        .arg(out_dir)
        .status()
        .context("tar 실행 실패 — 시스템에 tar 명령어가 있어야 합니다")?;
    if !status.success() {
        bail!("tar 추출 실패 (exit {:?})", status.code());
    }
    Ok(())
}

/// 현재 실행 바이너리 자리에 새 바이너리를 교체한다.
///
/// Unix 에서는 실행 중인 바이너리 경로에 rename 해도 커널이 기존 inode 를
/// 유지하므로 안전하다. 같은 파일시스템에 임시 복사 후 rename 하여 atomic.
fn atomic_replace(new_path: &Path, target: &Path) -> Result<()> {
    let tmp: PathBuf = target.with_extension("new");
    let _ = fs::remove_file(&tmp);
    fs::copy(new_path, &tmp)
        .with_context(|| format!("임시 복사 실패: {}", tmp.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perm = fs::metadata(&tmp)?.permissions();
        perm.set_mode(0o755);
        fs::set_permissions(&tmp, perm)?;
    }
    fs::rename(&tmp, target)
        .with_context(|| format!("설치 실패 (rename): {}", target.display()))?;
    Ok(())
}

fn detect_triple() -> Result<&'static str> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    match (os, arch) {
        ("macos", "aarch64") => Ok("aarch64-apple-darwin"),
        ("linux", "x86_64") => Ok("x86_64-unknown-linux-gnu"),
        _ => bail!(
            "지원하지 않는 플랫폼: {os}/{arch} — `kis update --from-source` 로 소스 빌드하세요."
        ),
    }
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
