//! 인증: 비밀번호 해시(argon2id) + 서버측 세션 + poem-openapi SecurityScheme.

pub mod password;
pub mod session;

use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::SecurityScheme;

use crate::state::AppState;

/// 인증된 사용자 (응답·핸들러에서 사용).
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub is_admin: bool,
}

/// DB users 행.
#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub is_admin: i64,
}

impl From<UserRow> for User {
    fn from(r: UserRow) -> Self {
        User {
            id: r.id,
            username: r.username,
            display_name: r.display_name,
            is_admin: r.is_admin != 0,
        }
    }
}

/// 세션 쿠키 기반 인증. 보호된 엔드포인트는 `auth: SessionAuth` 파라미터를 받는다.
/// 쿠키가 없거나 세션이 무효면 401.
#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "kisweb_session",
    key_in = "cookie",
    checker = "session_checker"
)]
pub struct SessionAuth(pub User);

async fn session_checker(req: &Request, api_key: ApiKey) -> Option<User> {
    let state = req.data::<AppState>()?;
    session::lookup(&state.db, &api_key.key).await.ok().flatten()
}
