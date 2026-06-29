//! 텔레그램 Bot API HTTP 헬퍼 (sendMessage / editMessageText / setMyCommands).
//!
//! 텔레그램 스트림 데몬이 사용한다(향후 stop-loss 알림에도 재사용 가능하게 분리).

use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde_json::Value;

use kis_core::config::TelegramConfig;

/// 공용 HTTP 클라이언트 — 연결 풀/keep-alive 재사용 + 타임아웃(10s) 부여.
/// 매 요청마다 `Client::new()` 하면 풀을 못 쓰고 매번 TLS 핸드셰이크 →
/// 네트워크가 조금만 흔들려도 `error sending request` 실패가 잦았다.
static HTTP: OnceLock<reqwest::Client> = OnceLock::new();

fn http() -> &'static reqwest::Client {
    HTTP.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

/// 429 응답의 `retry_after` 상한(초). Telegram이 비정상적으로 큰 값(수시간)을
/// 내려줄 때 데몬 전체가 통잠하는 것을 막는다. 9시간 백오프 재발 방지.
const RATE_LIMIT_CAP_SECS: u64 = 60;

pub enum EditOutcome {
    Ok,
    NotModified,
    NotFound,
    /// retry_after (초)
    RateLimited(u64),
}

pub async fn send_message(tg: &TelegramConfig, text: &str) -> Result<i64> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", tg.bot_token);
    let resp = http()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": tg.chat_id,
            "text": text,
            "parse_mode": "HTML",
            "disable_web_page_preview": true,
        }))
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await?;
    if body.get("ok").and_then(Value::as_bool) != Some(true) {
        return Err(anyhow!("sendMessage HTTP {}: {}", status, body));
    }
    body["result"]["message_id"]
        .as_i64()
        .ok_or_else(|| anyhow!("message_id 파싱 실패: {body}"))
}

/// 텔레그램 명령 메뉴(/ 자동완성·Menu 버튼)에 명령 목록을 등록(setMyCommands).
/// 호출하지 않으면 사용자에게 /add 등이 메뉴에 안 보인다. 등록은 봇 전역(기본 scope)에 1회면 충분.
pub async fn register_commands(tg: &TelegramConfig) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/setMyCommands", tg.bot_token);
    let resp = http()
        .post(&url)
        .json(&serde_json::json!({
            "commands": [
                {"command": "add",   "description": "종목 추가 (이름·코드, 국내·미국)"},
                {"command": "rm",    "description": "종목 삭제"},
                {"command": "list",  "description": "현재 관심종목"},
                {"command": "clear", "description": "전체 비우기"},
                {"command": "help",  "description": "도움말"},
            ]
        }))
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await?;
    if body.get("ok").and_then(Value::as_bool) != Some(true) {
        return Err(anyhow!("setMyCommands HTTP {}: {}", status, body));
    }
    Ok(())
}

pub async fn edit_message_text(
    tg: &TelegramConfig,
    message_id: i64,
    text: &str,
) -> Result<EditOutcome> {
    let url = format!("https://api.telegram.org/bot{}/editMessageText", tg.bot_token);
    let resp = http()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": tg.chat_id,
            "message_id": message_id,
            "text": text,
            "parse_mode": "HTML",
            "disable_web_page_preview": true,
        }))
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await?;
    if body.get("ok").and_then(Value::as_bool) == Some(true) {
        return Ok(EditOutcome::Ok);
    }
    let desc = body["description"].as_str().unwrap_or_default();
    if desc.contains("message is not modified") {
        return Ok(EditOutcome::NotModified);
    }
    if desc.contains("message to edit not found") || desc.contains("MESSAGE_ID_INVALID") {
        return Ok(EditOutcome::NotFound);
    }
    if status.as_u16() == 429 {
        let raw = body["parameters"]["retry_after"].as_u64().unwrap_or(1);
        let retry = raw.min(RATE_LIMIT_CAP_SECS);
        return Ok(EditOutcome::RateLimited(retry));
    }
    Err(anyhow!("editMessageText HTTP {}: {}", status, body))
}
