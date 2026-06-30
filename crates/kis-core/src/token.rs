use std::fs;
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use chrono::{Duration, Local, NaiveDateTime};
use reqwest::Client;

use crate::config::{self, Credentials};
use crate::models::{KisAccessToken, KisApprovalKey};

const BASE_URL: &str = "https://openapi.koreainvestment.com:9443";
const EXPIRY_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct TokenManager {
    credentials: Credentials,
    http: Client,
    token: Mutex<Option<KisAccessToken>>,
    ws_token: Mutex<Option<KisApprovalKey>>,
    /// 영속 캐시 계층(L2). 메모리 캐시(L1)는 위 Mutex 가 담당.
    /// CLI/데몬은 FileTokenStore(기본), 웹 멀티유저는 NullTokenStore.
    store: Arc<dyn TokenStore>,
}

impl TokenManager {
    /// 파일 기반 영속 캐시로 생성 (CLI·데몬 기본 동작).
    pub fn new(credentials: Credentials) -> Self {
        Self::with_store(credentials, Arc::new(FileTokenStore))
    }

    /// 영속 캐시 계층을 주입해 생성. 웹 멀티유저는 NullTokenStore 로 디스크 공유를 피한다.
    pub fn with_store(credentials: Credentials, store: Arc<dyn TokenStore>) -> Self {
        Self {
            credentials,
            http: Client::new(),
            token: Mutex::new(None),
            ws_token: Mutex::new(None),
            store,
        }
    }

    // ── REST 토큰 ──

    /// 유효한 액세스 토큰을 반환 (3단계 캐싱)
    pub async fn get_token(&self) -> Result<KisAccessToken> {
        // 1) 메모리 캐시
        {
            let guard = self.token.lock().unwrap();
            if let Some(ref t) = *guard {
                if !Self::is_expired(&t.access_token_token_expired) {
                    return Ok(t.clone());
                }
            }
        }

        // 2) 영속 캐시(L2)
        if let Some(t) = self.store.load_token() {
            if !Self::is_expired(&t.access_token_token_expired) {
                *self.token.lock().unwrap() = Some(t.clone());
                return Ok(t);
            }
        }

        // 3) API 요청
        let token = self.request_new_token().await?;
        *self.token.lock().unwrap() = Some(token.clone());
        Ok(token)
    }

    /// 액세스 토큰 문자열만 반환
    pub async fn get_access_token_string(&self) -> Result<String> {
        let t = self.get_token().await?;
        Ok(t.access_token)
    }

    async fn request_new_token(&self) -> Result<KisAccessToken> {
        let url = format!("{BASE_URL}/oauth2/tokenP");
        let body = serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": self.credentials.app_key,
            "appsecret": self.credentials.app_secret,
        });

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("토큰 요청 실패")?;

        let token: KisAccessToken = resp
            .json()
            .await
            .context("토큰 응답 파싱 실패")?;

        // 영속 캐시에 저장 (store 구현에 따라 파일 또는 no-op)
        self.store.save_token(&token);

        Ok(token)
    }

    // ── WebSocket Approval Key ──

    /// 유효한 WebSocket approval key를 반환 (3단계 캐싱)
    pub async fn get_ws_token(&self) -> Result<KisApprovalKey> {
        // 1) 메모리
        {
            let guard = self.ws_token.lock().unwrap();
            if let Some(ref t) = *guard {
                if !Self::is_expired(&t.approval_key_expired) {
                    return Ok(t.clone());
                }
            }
        }

        // 2) 영속 캐시(L2)
        if let Some(t) = self.store.load_ws_token() {
            if !Self::is_expired(&t.approval_key_expired) {
                *self.ws_token.lock().unwrap() = Some(t.clone());
                return Ok(t);
            }
        }

        // 3) API 요청
        let token = self.request_new_ws_token().await?;
        *self.ws_token.lock().unwrap() = Some(token.clone());
        Ok(token)
    }

    pub async fn get_ws_approval_key_string(&self) -> Result<String> {
        let t = self.get_ws_token().await?;
        Ok(t.approval_key)
    }

    async fn request_new_ws_token(&self) -> Result<KisApprovalKey> {
        let url = format!("{BASE_URL}/oauth2/Approval");
        let body = serde_json::json!({
            "grant_type": "client_credentials",
            "appkey": self.credentials.app_key,
            "secretkey": self.credentials.app_secret,
        });

        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("WebSocket approval key 요청 실패")?;

        let data: serde_json::Value = resp
            .json()
            .await
            .context("WebSocket approval key 응답 파싱 실패")?;

        let approval_key = data["approval_key"]
            .as_str()
            .context("approval_key 필드 없음")?
            .to_string();

        // 24시간 유효
        let expired = (Local::now() + Duration::hours(24))
            .format(EXPIRY_FORMAT)
            .to_string();

        let token = KisApprovalKey {
            approval_key,
            approval_key_expired: expired,
        };

        // 영속 캐시에 저장 (store 구현에 따라 파일 또는 no-op)
        self.store.save_ws_token(&token);

        Ok(token)
    }

    // ── 유틸 ──

    /// 만료 여부 확인 (5분 버퍼)
    fn is_expired(expiry_str: &str) -> bool {
        let Ok(expiry) = NaiveDateTime::parse_from_str(expiry_str, EXPIRY_FORMAT) else {
            return true;
        };
        let now = Local::now().naive_local();
        now >= expiry - Duration::minutes(5)
    }

    /// 토큰 무효화 (REST·WS 메모리 캐시 + 영속 캐시 모두 삭제)
    pub fn invalidate(&self) {
        *self.token.lock().unwrap() = None;
        *self.ws_token.lock().unwrap() = None;
        self.store.clear();
    }
}

/// 토큰 영속화 계층(L2). 메모리 캐시(L1)는 TokenManager 내부 Mutex 가 담당하고,
/// 이 트레잇은 그 위의 "프로세스 재시작에도 살아남는" 저장소를 추상화한다.
///
/// - CLI/데몬: [`FileTokenStore`] — `~/.config/kis-cli/.kis_*token.json` 공유 (기본).
/// - 웹 멀티유저: [`NullTokenStore`] — 디스크에 안 남겨 사용자 간 토큰 누출을 차단.
pub trait TokenStore: Send + Sync {
    fn load_token(&self) -> Option<KisAccessToken>;
    fn save_token(&self, token: &KisAccessToken);
    fn load_ws_token(&self) -> Option<KisApprovalKey>;
    fn save_ws_token(&self, token: &KisApprovalKey);
    fn clear(&self);
}

/// 고정 경로 파일 캐시 — 기존 CLI·데몬 동작과 동일.
pub struct FileTokenStore;

impl TokenStore for FileTokenStore {
    fn load_token(&self) -> Option<KisAccessToken> {
        let path = config::token_path().ok()?;
        let content = fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    fn save_token(&self, token: &KisAccessToken) {
        let Ok(path) = config::token_path() else { return };
        if let Ok(dir) = config::config_dir() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(token) {
            let _ = fs::write(path, json);
        }
    }

    fn load_ws_token(&self) -> Option<KisApprovalKey> {
        let path = config::ws_token_path().ok()?;
        let content = fs::read_to_string(path).ok()?;
        serde_json::from_str(&content).ok()
    }

    fn save_ws_token(&self, token: &KisApprovalKey) {
        let Ok(path) = config::ws_token_path() else { return };
        if let Ok(dir) = config::config_dir() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(token) {
            let _ = fs::write(path, json);
        }
    }

    fn clear(&self) {
        clear_cache_files();
    }
}

/// 영속화하지 않는 캐시 — 토큰을 디스크에 남기지 않는다.
/// 웹 멀티유저에서 사용자별 KisClient 가 한 프로세스에 공존해도 토큰이 섞이지 않도록 보장.
/// (프로세스 내 캐싱은 TokenManager 의 메모리 L1 캐시가 그대로 수행한다.)
pub struct NullTokenStore;

impl TokenStore for NullTokenStore {
    fn load_token(&self) -> Option<KisAccessToken> {
        None
    }
    fn save_token(&self, _token: &KisAccessToken) {}
    fn load_ws_token(&self) -> Option<KisApprovalKey> {
        None
    }
    fn save_ws_token(&self, _token: &KisApprovalKey) {}
    fn clear(&self) {}
}

/// 디스크 상의 토큰 캐시 파일을 모두 삭제 (TokenManager 인스턴스 없이 호출 가능)
pub fn clear_cache_files() {
    if let Ok(path) = config::token_path() {
        let _ = fs::remove_file(path);
    }
    if let Ok(path) = config::ws_token_path() {
        let _ = fs::remove_file(path);
    }
}
