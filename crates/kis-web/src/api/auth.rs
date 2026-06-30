//! 인증 API: 회원가입 / 로그인 / 로그아웃 / 내 정보.

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::param::Cookie;
use poem_openapi::payload::{Json, Response};
use poem_openapi::{Object, OpenApi};
use uuid::Uuid;

use crate::auth::{password, session, SessionAuth, User, UserRow};
use crate::state::AppState;

#[derive(Object)]
struct RegisterReq {
    username: String,
    display_name: String,
    password: String,
}

#[derive(Object)]
struct LoginReq {
    username: String,
    password: String,
}

#[derive(Object)]
struct UserDto {
    id: String,
    username: String,
    display_name: String,
    is_admin: bool,
}

impl From<User> for UserDto {
    fn from(u: User) -> Self {
        UserDto {
            id: u.id,
            username: u.username,
            display_name: u.display_name,
            is_admin: u.is_admin,
        }
    }
}

pub struct AuthApi;

#[OpenApi(prefix_path = "/auth", tag = "ApiTag::Auth")]
impl AuthApi {
    /// 회원가입. 첫 사용자는 관리자로 생성된다. 이후 가입은 KIS_WEB_ALLOW_REGISTER 로 제어.
    #[oai(path = "/register", method = "post")]
    async fn register(
        &self,
        state: Data<&AppState>,
        body: Json<RegisterReq>,
    ) -> Result<Response<Json<UserDto>>> {
        let st = state.0;
        let req = body.0;

        let username = req.username.trim().to_string();
        if username.is_empty() {
            return Err(Error::from_string("사용자 이름을 입력하세요", StatusCode::BAD_REQUEST));
        }
        if req.password.len() < 8 {
            return Err(Error::from_string(
                "비밀번호는 8자 이상이어야 합니다",
                StatusCode::BAD_REQUEST,
            ));
        }

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&st.db)
            .await
            .map_err(internal)?;
        let is_first = count.0 == 0;

        if !is_first && !st.config.allow_register {
            return Err(Error::from_string(
                "회원가입이 비활성화되어 있습니다",
                StatusCode::FORBIDDEN,
            ));
        }

        // 중복 사용자명 검사
        let exists: Option<(String,)> = sqlx::query_as("SELECT id FROM users WHERE username = ?")
            .bind(&username)
            .fetch_optional(&st.db)
            .await
            .map_err(internal)?;
        if exists.is_some() {
            return Err(Error::from_string(
                "이미 사용 중인 사용자 이름입니다",
                StatusCode::CONFLICT,
            ));
        }

        let display_name = {
            let d = req.display_name.trim();
            if d.is_empty() { username.clone() } else { d.to_string() }
        };
        let password_hash = password::hash(&req.password).map_err(internal)?;
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO users (id, username, display_name, password_hash, is_admin, created_at) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&username)
        .bind(&display_name)
        .bind(&password_hash)
        .bind(if is_first { 1 } else { 0 })
        .bind(&now)
        .execute(&st.db)
        .await
        .map_err(internal)?;

        let user = User {
            id: id.clone(),
            username,
            display_name,
            is_admin: is_first,
        };
        issue_session(st, user).await
    }

    /// 로그인. 성공 시 세션 쿠키를 발급한다.
    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        state: Data<&AppState>,
        body: Json<LoginReq>,
    ) -> Result<Response<Json<UserDto>>> {
        let st = state.0;
        let req = body.0;

        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, username, display_name, password_hash, is_admin FROM users WHERE username = ?",
        )
        .bind(req.username.trim())
        .fetch_optional(&st.db)
        .await
        .map_err(internal)?;

        // 사용자 없음/비번 불일치 모두 동일 메시지 (사용자 존재 여부 노출 방지)
        let unauthorized =
            || Error::from_string("사용자 이름 또는 비밀번호가 올바르지 않습니다", StatusCode::UNAUTHORIZED);

        let Some(row) = row else {
            // 타이밍 평탄화를 위해 더미 검증 후 실패
            password::verify(&req.password, DUMMY_HASH);
            return Err(unauthorized());
        };

        if !password::verify(&req.password, &row.password_hash) {
            return Err(unauthorized());
        }

        issue_session(st, User::from(row)).await
    }

    /// 로그아웃. 세션을 삭제하고 쿠키를 비운다.
    #[oai(path = "/logout", method = "post")]
    async fn logout(
        &self,
        state: Data<&AppState>,
        #[oai(name = "kisweb_session")] token: Cookie<Option<String>>,
    ) -> Result<Response<Json<super::OkDto>>> {
        let st = state.0;
        if let Some(tok) = token.0 {
            let _ = session::delete(&st.db, &tok).await;
        }
        Ok(Response::new(Json(super::OkDto { ok: true }))
            .header("Set-Cookie", session::clear_cookie(&st.config)))
    }

    /// 현재 로그인된 사용자 정보 (세션 필요).
    #[oai(path = "/me", method = "get")]
    async fn me(&self, auth: SessionAuth) -> Json<UserDto> {
        Json(UserDto::from(auth.0))
    }
}

/// 세션 생성 + Set-Cookie 헤더를 단 응답.
async fn issue_session(
    st: &AppState,
    user: User,
) -> Result<Response<Json<UserDto>>> {
    let token = session::create(&st.db, &user.id).await.map_err(internal)?;
    let cookie = session::build_cookie(&token, &st.config);
    Ok(Response::new(Json(UserDto::from(user))).header("Set-Cookie", cookie))
}

fn internal<E: std::fmt::Display>(e: E) -> Error {
    tracing::error!("internal error: {e}");
    Error::from_string("서버 오류", StatusCode::INTERNAL_SERVER_ERROR)
}

/// 존재하지 않는 사용자 로그인 시 타이밍 사이드채널 완화를 위한 더미 argon2 해시.
const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHRzb21lc2FsdA$RdescudvJCsgt3ub+b+dWRWJTmaaJObG";

use super::ApiTag;
