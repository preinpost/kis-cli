//! 환경변수 기반 서버 설정.

use anyhow::{Context, Result};

#[derive(Clone)]
pub struct Config {
    /// 바인드 주소 (`KIS_WEB_BIND`, 기본 127.0.0.1:8088).
    pub bind: String,
    /// SQLite 파일 경로 (`KIS_WEB_DB_PATH`, 기본 $XDG_CONFIG_HOME/kis-cli/kis-web.db).
    pub db_path: String,
    /// 종목 마스터 SQLite (`KIS_WEB_SYMBOLS_DB`, 기본 $XDG_CONFIG_HOME/kis-cli/symbols.db).
    /// kis-cli `symbols sync` 가 만든 DB를 읽기 전용으로 재사용.
    pub symbols_db_path: String,
    /// KIS 자격증명 봉투암호화 마스터키 (`KIS_WEB_MASTER_KEY`, 32바이트 hex 또는 임의 문자열→SHA-256).
    pub master_key: [u8; 32],
    /// 세션 쿠키에 Secure 속성 부여 여부 (`KIS_WEB_SECURE_COOKIE`, 기본 false → 로컬 http 개발).
    /// 퍼블릭(HTTPS) 배포 시 반드시 true.
    pub secure_cookie: bool,
    /// 신규 회원가입 허용 여부 (`KIS_WEB_ALLOW_REGISTER`, 기본 true).
    /// 첫 사용자 가입 후 닫고 싶으면 false (그땐 관리자가 초대).
    pub allow_register: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let bind = std::env::var("KIS_WEB_BIND").unwrap_or_else(|_| "127.0.0.1:8088".to_string());

        let db_path = std::env::var("KIS_WEB_DB_PATH").unwrap_or_else(|_| {
            let dir = dirs_config_dir();
            format!("{dir}/kis-web.db")
        });

        let symbols_db_path = std::env::var("KIS_WEB_SYMBOLS_DB").unwrap_or_else(|_| {
            let dir = dirs_config_dir();
            format!("{dir}/symbols.db")
        });

        let master_key = derive_master_key()?;

        let secure_cookie = env_bool("KIS_WEB_SECURE_COOKIE", false);
        let allow_register = env_bool("KIS_WEB_ALLOW_REGISTER", true);

        Ok(Self {
            bind,
            db_path,
            symbols_db_path,
            master_key,
            secure_cookie,
            allow_register,
        })
    }
}

/// `$XDG_CONFIG_HOME/kis-cli` 또는 `~/.config/kis-cli` (kis-core 와 동일 위치).
fn dirs_config_dir() -> String {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return format!("{xdg}/kis-cli");
    }
    if let Ok(home) = std::env::var("HOME") {
        return format!("{home}/.config/kis-cli");
    }
    ".".to_string()
}

/// 마스터키: 정확히 64자 hex(= 32바이트). 미설정/형식오류 시 에러.
/// 패스프레이즈→키 도출 같은 약한 방식 대신 고엔트로피 키를 강제한다.
///   생성: `openssl rand -hex 32`
fn derive_master_key() -> Result<[u8; 32]> {
    let raw = std::env::var("KIS_WEB_MASTER_KEY")
        .context("KIS_WEB_MASTER_KEY 미설정 — KIS 자격증명 암호화에 필요. 생성: openssl rand -hex 32")?;
    let raw = raw.trim();
    let bytes = hex::decode(raw).map_err(|_| {
        anyhow::anyhow!("KIS_WEB_MASTER_KEY 는 64자 hex 여야 함(openssl rand -hex 32)")
    })?;
    let key: [u8; 32] = bytes.try_into().map_err(|_| {
        anyhow::anyhow!("KIS_WEB_MASTER_KEY 는 정확히 32바이트(64 hex)여야 함")
    })?;
    Ok(key)
}

fn env_bool(key: &str, default: bool) -> bool {
    match std::env::var(key) {
        Ok(v) => matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
        Err(_) => default,
    }
}
