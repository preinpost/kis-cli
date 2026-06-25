use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub credentials: Credentials,
    #[serde(default)]
    pub is_mock: bool,
    #[serde(default)]
    pub telegram: Option<TelegramConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
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
///
/// `sudo` 실행 시엔 HOME 이 `/root` 로 바뀌므로, `SUDO_USER` 가 있으면
/// 원래 유저의 홈 디렉토리 기준으로 리다이렉트한다. (VPS 에서 systemd unit 설치할 때
/// `sudo $(which kis) ...` 형태로 실행되는 케이스 커버)
pub fn config_dir() -> Result<PathBuf> {
    if let Some(dir) = sudo_user_config_dir() {
        return Ok(dir);
    }
    let base = dirs::config_dir().context("설정 디렉토리를 찾을 수 없습니다")?;
    Ok(base.join("kis-cli"))
}

fn sudo_user_config_dir() -> Option<PathBuf> {
    let sudo_user = std::env::var("SUDO_USER").ok()?;
    if sudo_user.is_empty() || sudo_user == "root" {
        return None;
    }
    let home = lookup_home(&sudo_user)?;
    Some(home.join(".config").join("kis-cli"))
}

fn lookup_home(user: &str) -> Option<PathBuf> {
    // /etc/passwd 에서 직접 조회 (libc 없이 NSS 우회)
    if let Ok(passwd) = fs::read_to_string("/etc/passwd") {
        for line in passwd.lines() {
            let fields: Vec<&str> = line.split(':').collect();
            if fields.first().copied() == Some(user) {
                if let Some(home) = fields.get(5) {
                    return Some(PathBuf::from(home));
                }
            }
        }
    }
    // fallback: 관례적 경로
    if cfg!(target_os = "macos") {
        Some(PathBuf::from("/Users").join(user))
    } else {
        Some(PathBuf::from("/home").join(user))
    }
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

/// 데이트레이드 매매 기록 SQLite 경로
pub fn daytrade_db_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("daytrade.db"))
}

/// 데이트레이드 데몬 설정 파일 경로 (`daytrade.toml`)
pub fn daytrade_config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("daytrade.toml"))
}

/// 텔레그램 스트림 관심종목 파일 경로 (`telegram-stream.toml`)
pub fn telegram_stream_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("telegram-stream.toml"))
}

/// 설정 파일 로딩
///
/// 우선순위: `config.toml` 파일 → 환경변수 오버레이.
/// 파일이 없어도 `KIS_*` 환경변수만으로 구성할 수 있다 (컨테이너/12-factor 배포).
/// 파일이 있으면 기존과 동일하게 동작하고, 환경변수가 주어진 필드만 덮어쓴다.
pub fn load_config() -> Result<AppConfig> {
    let path = config_path()?;

    // 1) 파일이 있으면 로드, 없으면(NotFound) env 로만 구성 시도.
    let mut config = match fs::read_to_string(&path) {
        Ok(content) => toml::from_str(&content)
            .with_context(|| format!("설정 파일 파싱 오류: {}", path.display()))?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => AppConfig {
            credentials: Credentials {
                app_key: String::new(),
                app_secret: String::new(),
                account_number: String::new(),
            },
            is_mock: false,
            telegram: None,
        },
        Err(e) => {
            return Err(e)
                .with_context(|| format!("설정 파일을 읽을 수 없습니다: {}", path.display()));
        }
    };

    // 2) 환경변수 오버레이 — 값이 있는 필드만 덮어쓴다.
    apply_env_overrides(&mut config);

    // 3) 필수 자격증명 검증.
    if config.credentials.app_key.is_empty()
        || config.credentials.app_secret.is_empty()
        || config.credentials.account_number.is_empty()
    {
        anyhow::bail!(
            "자격증명이 없습니다. `{}` 를 만들거나(`kis config init`) \
             환경변수 KIS_APP_KEY / KIS_APP_SECRET / KIS_ACCOUNT_NUMBER 를 설정하세요.",
            path.display()
        );
    }

    Ok(config)
}

/// `KIS_*` 환경변수를 설정에 덮어쓴다. 컨테이너 배포 시 `.env`/`environment:` 로 시크릿 주입.
fn apply_env_overrides(config: &mut AppConfig) {
    fn env_nonempty(key: &str) -> Option<String> {
        std::env::var(key).ok().filter(|v| !v.is_empty())
    }

    if let Some(v) = env_nonempty("KIS_APP_KEY") {
        config.credentials.app_key = v;
    }
    if let Some(v) = env_nonempty("KIS_APP_SECRET") {
        config.credentials.app_secret = v;
    }
    if let Some(v) = env_nonempty("KIS_ACCOUNT_NUMBER") {
        config.credentials.account_number = v;
    }
    if let Some(v) = env_nonempty("KIS_IS_MOCK") {
        if let Some(b) = parse_bool(&v) {
            config.is_mock = b;
        }
    }
    // 텔레그램: bot_token + chat_id 가 모두 env 로 주어지면 구성/덮어쓰기.
    if let (Some(bot_token), Some(chat_id)) = (
        env_nonempty("KIS_TELEGRAM_BOT_TOKEN"),
        env_nonempty("KIS_TELEGRAM_CHAT_ID"),
    ) {
        config.telegram = Some(TelegramConfig { bot_token, chat_id });
    }
}

fn parse_bool(s: &str) -> Option<bool> {
    match s.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "on" => Some(true),
        "false" | "0" | "no" | "n" | "off" => Some(false),
        _ => None,
    }
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
