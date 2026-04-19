//! Claude Code 스킬 설치 서브커맨드.
//!
//! `~/.claude/skills/kis/SKILL.md` 경로에 임베드된 스킬 정의를 배포한다.
//! 설치 후 Claude Code가 자연어(예: "삼성전자 분석해줘")를 kis-cli 커맨드로 매핑 실행.

use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};

const SKILL_BODY: &str = include_str!("../../skill/SKILL.md");
const SKILL_NAME: &str = "kis";

fn skills_root() -> Result<PathBuf> {
    let home = dirs::home_dir().context("홈 디렉토리를 찾을 수 없습니다")?;
    Ok(home.join(".claude").join("skills"))
}

fn skill_dir() -> Result<PathBuf> {
    Ok(skills_root()?.join(SKILL_NAME))
}

fn skill_file() -> Result<PathBuf> {
    Ok(skill_dir()?.join("SKILL.md"))
}

pub fn run_install(force: bool) -> Result<()> {
    let file = skill_file()?;
    if file.exists() && !force {
        bail!(
            "이미 설치돼 있음: {}\n덮어쓰려면 --force",
            file.display()
        );
    }
    let dir = skill_dir()?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("디렉토리 생성 실패: {}", dir.display()))?;
    fs::write(&file, SKILL_BODY)
        .with_context(|| format!("파일 쓰기 실패: {}", file.display()))?;
    println!("✓ 스킬 설치 완료: {}", file.display());
    println!();
    println!("Claude Code를 재시작하면 자동으로 로드됩니다.");
    println!("테스트: Claude에게 \"삼성전자 현재가 알려줘\" 같은 자연어 요청 → kis-cli 실행됨");
    Ok(())
}

pub fn run_uninstall() -> Result<()> {
    let dir = skill_dir()?;
    if !dir.exists() {
        println!("설치돼 있지 않음: {}", dir.display());
        return Ok(());
    }
    fs::remove_dir_all(&dir)
        .with_context(|| format!("디렉토리 삭제 실패: {}", dir.display()))?;
    println!("✓ 스킬 제거 완료: {}", dir.display());
    Ok(())
}

pub fn run_path() -> Result<()> {
    let file = skill_file()?;
    println!("{}", file.display());
    if !file.exists() {
        println!("(아직 설치되지 않음 — `kis skill install`로 설치)");
    }
    Ok(())
}
