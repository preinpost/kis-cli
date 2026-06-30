//! 핸들러·미들웨어가 공유하는 애플리케이션 상태.
//!
//! `Arc<AppStateInner>` 를 poem `.data()` 로 주입하고, 핸들러는 `Data<&AppState>`,
//! SecurityScheme checker 는 `req.data::<AppState>()` 로 접근한다.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sqlx::SqlitePool;

use crate::config::Config;
use crate::kis::ClientManager;

pub type AppState = Arc<AppStateInner>;

/// 미니차트(일봉) 캐시 엔트리.
struct SparkEntry {
    points: Vec<f64>,
    up: bool,
    at: Instant,
}

const SPARK_TTL: Duration = Duration::from_secs(30 * 60);

pub struct AppStateInner {
    pub db: SqlitePool,
    pub config: Config,
    /// 사용자별 KisClient 캐시.
    pub clients: ClientManager,
    /// 종목별 미니차트 캐시 (일봉이라 사용자 무관·30분 TTL → KIS API 부담 감소).
    spark_cache: Mutex<HashMap<String, SparkEntry>>,
}

impl AppStateInner {
    pub fn new(db: SqlitePool, config: Config) -> AppState {
        Arc::new(Self {
            db,
            config,
            clients: ClientManager::new(),
            spark_cache: Mutex::new(HashMap::new()),
        })
    }

    /// 캐시된 미니차트 (없거나 만료면 None).
    pub fn spark_get(&self, symbol: &str) -> Option<(Vec<f64>, bool)> {
        let cache = self.spark_cache.lock().unwrap();
        let e = cache.get(symbol)?;
        if e.at.elapsed() < SPARK_TTL {
            Some((e.points.clone(), e.up))
        } else {
            None
        }
    }

    pub fn spark_put(&self, symbol: &str, points: Vec<f64>, up: bool) {
        self.spark_cache.lock().unwrap().insert(
            symbol.to_string(),
            SparkEntry { points, up, at: Instant::now() },
        );
    }
}
