use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub credentials: Credentials,
    #[serde(default)]
    pub is_mock: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub app_key: String,
    pub app_secret: String,
    pub account_number: String,
}

impl Credentials {
    /// 계좌번호 앞 8자리 (CANO)
    pub fn cano(&self) -> &str {
        self.account_number.split('-').next().unwrap_or("")
    }

    /// 계좌상품코드 2자리 (ACNT_PRDT_CD)
    pub fn product_code(&self) -> &str {
        self.account_number
            .split('-')
            .nth(1)
            .unwrap_or("01")
    }
}

/// 설정 디렉토리 경로: ~/.config/kis-cli/
pub fn config_dir() -> Result<PathBuf> {
    let base = dirs::config_dir().context("설정 디렉토리를 찾을 수 없습니다")?;
    Ok(base.join("kis-cli"))
}

/// 설정 파일 경로: ~/.config/kis-cli/config.toml
pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

/// 토큰 캐시 파일 경로
pub fn token_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(".kis_token.json"))
}

/// WebSocket 토큰 캐시 파일 경로
pub fn ws_token_path() -> Result<PathBuf> {
    Ok(config_dir()?.join(".kis_ws_token.json"))
}

/// 종목 마스터 DB 경로
pub fn symbols_db_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("symbols.db"))
}

/// 자동 손절 데몬 상태 파일 경로
pub fn stoploss_status_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("stoploss.status.json"))
}

/// 설정 파일 로딩
pub fn load_config() -> Result<AppConfig> {
    let path = config_path()?;
    let content = fs::read_to_string(&path)
        .with_context(|| format!("설정 파일을 읽을 수 없습니다: {}\n`kis config init`으로 초기화하세요.", path.display()))?;
    let config: AppConfig = toml::from_str(&content)
        .with_context(|| format!("설정 파일 파싱 오류: {}", path.display()))?;
    Ok(config)
}

/// 설정 파일 저장
pub fn save_config(config: &AppConfig) -> Result<()> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    let path = dir.join("config.toml");
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}
