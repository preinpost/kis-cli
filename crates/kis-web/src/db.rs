//! SQLite 풀 초기화 + 마이그레이션.

use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

/// 마이그레이션은 컴파일타임에 `migrations/` 폴더에서 임베드 (라이브 DB 불필요).
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

/// 풀을 열고(없으면 파일 생성) 마이그레이션을 적용한다.
pub async fn init(db_path: &str) -> Result<SqlitePool> {
    // SQLite 는 파일은 만들어도 상위 디렉터리는 안 만든다 — 먼저 생성.
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("DB 디렉터리 생성 실패: {}", parent.display()))?;
        }
    }

    let opts = SqliteConnectOptions::from_str(&format!("sqlite://{db_path}"))
        .with_context(|| format!("DB 경로 파싱 실패: {db_path}"))?
        .create_if_missing(true)
        .foreign_keys(true)
        .busy_timeout(std::time::Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await
        .with_context(|| format!("SQLite 열기 실패: {db_path}"))?;

    MIGRATOR
        .run(&pool)
        .await
        .context("마이그레이션 적용 실패")?;

    Ok(pool)
}
