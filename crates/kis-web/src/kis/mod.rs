//! 사용자별 KisClient 매니저.
//!
//! 로그인 사용자 → DB의 암호화 자격증명 복호화 → `KisClient::with_in_memory_cache` 로
//! 인스턴스 생성 후 캐시. 토큰은 클라이언트의 메모리 L1 캐시에만 존재하며 디스크/타사용자와
//! 공유되지 않는다(NullTokenStore). 캐시는 자격증명 변경 시 무효화한다.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::Result;
use kis_core::client::KisClient;
use kis_core::config::Credentials;
use sqlx::SqlitePool;

use crate::crypto;

#[derive(sqlx::FromRow)]
struct CredRow {
    secret_enc: Vec<u8>,
    nonce: Vec<u8>,
    account_number: String,
    is_mock: i64,
}

#[derive(Default)]
pub struct ClientManager {
    cache: RwLock<HashMap<String, Arc<KisClient>>>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// 사용자의 KisClient 를 반환. 자격증명 미등록 시 `Ok(None)`.
    pub async fn get(
        &self,
        db: &SqlitePool,
        master_key: &[u8; 32],
        user_id: &str,
    ) -> Result<Option<Arc<KisClient>>> {
        // 빠른 경로: 캐시 히트 (guard 는 이 문장에서 즉시 해제 — await 안 넘김)
        if let Some(c) = self.cache.read().unwrap().get(user_id).cloned() {
            return Ok(Some(c));
        }

        let row: Option<CredRow> = sqlx::query_as(
            "SELECT secret_enc, nonce, account_number, is_mock FROM kis_credentials WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_optional(db)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let secret = crypto::open(master_key, &row.secret_enc, &row.nonce)?;
        let creds = Credentials {
            app_key: secret.app_key,
            app_secret: secret.app_secret,
            account_number: row.account_number,
        };
        let client = Arc::new(KisClient::with_in_memory_cache(creds, row.is_mock != 0));

        self.cache
            .write()
            .unwrap()
            .insert(user_id.to_string(), client.clone());
        Ok(Some(client))
    }

    /// 자격증명 변경/삭제 시 캐시 무효화.
    pub fn invalidate(&self, user_id: &str) {
        self.cache.write().unwrap().remove(user_id);
    }
}
