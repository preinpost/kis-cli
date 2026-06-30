//! 핸들러·미들웨어가 공유하는 애플리케이션 상태.
//!
//! `Arc<AppStateInner>` 를 poem `.data()` 로 주입하고, 핸들러는 `Data<&AppState>`,
//! SecurityScheme checker 는 `req.data::<AppState>()` 로 접근한다.

use std::sync::Arc;

use sqlx::SqlitePool;

use crate::config::Config;
use crate::kis::ClientManager;

pub type AppState = Arc<AppStateInner>;

pub struct AppStateInner {
    pub db: SqlitePool,
    pub config: Config,
    /// 사용자별 KisClient 캐시.
    pub clients: ClientManager,
}

impl AppStateInner {
    pub fn new(db: SqlitePool, config: Config) -> AppState {
        Arc::new(Self {
            db,
            config,
            clients: ClientManager::new(),
        })
    }
}
