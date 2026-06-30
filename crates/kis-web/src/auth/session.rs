//! 서버측 세션. 원문 토큰은 쿠키에만, DB엔 SHA-256 해시(id)만 저장한다.

use anyhow::Result;
use chrono::{Duration, Utc};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use super::{User, UserRow};
use crate::config::Config;

pub const COOKIE_NAME: &str = "kisweb_session";
const TTL_DAYS: i64 = 7;

/// 원문 토큰의 SHA-256 hex (DB의 sessions.id).
fn token_hash(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex::encode(digest)
}

/// 새 세션 생성 → 쿠키에 넣을 원문 토큰 반환.
pub async fn create(pool: &SqlitePool, user_id: &str) -> Result<String> {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let token = hex::encode(bytes);
    let id = token_hash(&token);

    let now = Utc::now();
    let expires = now + Duration::days(TTL_DAYS);

    sqlx::query("INSERT INTO sessions (id, user_id, created_at, expires_at) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(user_id)
        .bind(now.to_rfc3339())
        .bind(expires.to_rfc3339())
        .execute(pool)
        .await?;

    Ok(token)
}

/// 토큰으로 세션을 조회 → 유효하면 User. 만료 세션은 삭제하고 None.
pub async fn lookup(pool: &SqlitePool, token: &str) -> Result<Option<User>> {
    let id = token_hash(token);

    let row: Option<(String, String)> =
        sqlx::query_as("SELECT user_id, expires_at FROM sessions WHERE id = ?")
            .bind(&id)
            .fetch_optional(pool)
            .await?;

    let Some((user_id, expires_at)) = row else {
        return Ok(None);
    };

    // 만료 검사
    if let Ok(exp) = chrono::DateTime::parse_from_rfc3339(&expires_at) {
        if Utc::now() >= exp {
            let _ = sqlx::query("DELETE FROM sessions WHERE id = ?")
                .bind(&id)
                .execute(pool)
                .await;
            return Ok(None);
        }
    }

    let user: Option<UserRow> = sqlx::query_as(
        "SELECT id, username, display_name, password_hash, is_admin FROM users WHERE id = ?",
    )
    .bind(&user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user.map(User::from))
}

/// 토큰의 세션 삭제(로그아웃).
pub async fn delete(pool: &SqlitePool, token: &str) -> Result<()> {
    let id = token_hash(token);
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(&id)
        .execute(pool)
        .await?;
    Ok(())
}

/// 로그인 시 Set-Cookie 헤더 값.
pub fn build_cookie(token: &str, config: &Config) -> String {
    let max_age = TTL_DAYS * 24 * 3600;
    let mut c = format!(
        "{COOKIE_NAME}={token}; HttpOnly; SameSite=Strict; Path=/; Max-Age={max_age}"
    );
    if config.secure_cookie {
        c.push_str("; Secure");
    }
    c
}

/// 로그아웃 시 쿠키 삭제용 Set-Cookie 헤더 값.
pub fn clear_cookie(config: &Config) -> String {
    let mut c = format!("{COOKIE_NAME}=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0");
    if config.secure_cookie {
        c.push_str("; Secure");
    }
    c
}
