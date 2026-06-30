//! KIS 자격증명 관리. 시크릿은 AES-GCM 암호화 저장하고 클라이언트로 절대 반환하지 않는다.

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

use crate::auth::SessionAuth;
use crate::crypto::{self, Secret};
use crate::state::AppState;

use super::ApiTag;

#[derive(Object)]
struct KisCredentialsReq {
    app_key: String,
    app_secret: String,
    /// 계좌번호. "12345678-01" 또는 "1234567801" 형식.
    account_number: String,
    /// 모의투자 계좌 여부(기본 true — 안전).
    is_mock: Option<bool>,
}

/// 시크릿은 빠지고, 등록 여부·계좌·모의 여부만.
#[derive(Object)]
struct KisCredentialsStatus {
    configured: bool,
    account_number: Option<String>,
    is_mock: Option<bool>,
}

pub struct AccountApi;

#[OpenApi(prefix_path = "/account", tag = "ApiTag::Account")]
impl AccountApi {
    /// KIS 자격증명 등록/갱신 (암호화 저장).
    #[oai(path = "/kis-credentials", method = "put")]
    async fn put_credentials(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        body: Json<KisCredentialsReq>,
    ) -> Result<Json<KisCredentialsStatus>> {
        let st = state.0;
        let user = auth.0;
        let req = body.0;

        let app_key = req.app_key.trim().to_string();
        let app_secret = req.app_secret.trim().to_string();
        let account_number = req.account_number.trim().to_string();
        if app_key.is_empty() || app_secret.is_empty() || account_number.is_empty() {
            return Err(Error::from_string(
                "app_key / app_secret / 계좌번호를 모두 입력하세요",
                StatusCode::BAD_REQUEST,
            ));
        }
        let is_mock = req.is_mock.unwrap_or(true);

        let sealed = crypto::seal(
            &st.config.master_key,
            &Secret { app_key, app_secret },
        )
        .map_err(internal)?;

        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO kis_credentials (user_id, secret_enc, nonce, account_number, is_mock, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?) \
             ON CONFLICT(user_id) DO UPDATE SET \
               secret_enc = excluded.secret_enc, nonce = excluded.nonce, \
               account_number = excluded.account_number, is_mock = excluded.is_mock, \
               updated_at = excluded.updated_at",
        )
        .bind(&user.id)
        .bind(&sealed.ciphertext)
        .bind(&sealed.nonce)
        .bind(&account_number)
        .bind(if is_mock { 1 } else { 0 })
        .bind(&now)
        .execute(&st.db)
        .await
        .map_err(internal)?;

        // 캐시된 클라이언트 무효화 → 다음 요청에 새 자격증명으로 재생성
        st.clients.invalidate(&user.id);

        Ok(Json(KisCredentialsStatus {
            configured: true,
            account_number: Some(account_number),
            is_mock: Some(is_mock),
        }))
    }

    /// 자격증명 등록 여부 조회 (시크릿 미반환).
    #[oai(path = "/kis-credentials/status", method = "get")]
    async fn status(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
    ) -> Result<Json<KisCredentialsStatus>> {
        let st = state.0;
        let row: Option<(String, i64)> =
            sqlx::query_as("SELECT account_number, is_mock FROM kis_credentials WHERE user_id = ?")
                .bind(&auth.0.id)
                .fetch_optional(&st.db)
                .await
                .map_err(internal)?;

        Ok(Json(match row {
            Some((account_number, is_mock)) => KisCredentialsStatus {
                configured: true,
                account_number: Some(account_number),
                is_mock: Some(is_mock != 0),
            },
            None => KisCredentialsStatus {
                configured: false,
                account_number: None,
                is_mock: None,
            },
        }))
    }

    /// 자격증명 삭제.
    #[oai(path = "/kis-credentials", method = "delete")]
    async fn delete_credentials(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
    ) -> Result<Json<KisCredentialsStatus>> {
        let st = state.0;
        sqlx::query("DELETE FROM kis_credentials WHERE user_id = ?")
            .bind(&auth.0.id)
            .execute(&st.db)
            .await
            .map_err(internal)?;
        st.clients.invalidate(&auth.0.id);
        Ok(Json(KisCredentialsStatus {
            configured: false,
            account_number: None,
            is_mock: None,
        }))
    }
}

fn internal<E: std::fmt::Display>(e: E) -> Error {
    tracing::error!("account internal error: {e}");
    Error::from_string("서버 오류", StatusCode::INTERNAL_SERVER_ERROR)
}
