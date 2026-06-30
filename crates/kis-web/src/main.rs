//! kis-web — 포트폴리오/시세 웹앱 백엔드.
//!
//! poem-openapi 로 타입드 REST API(`/api`)를 노출하고, Vite SPA(`web/dist`)를 정적 서빙한다.
//! P1: 멀티유저 인증(회원가입·로그인·세션). KIS 연동·포트폴리오는 P2.

use poem::{EndpointExt, Route, Server, get, listener::TcpListener, middleware::CookieJarManager};
use poem_openapi::OpenApiService;

mod api;
mod auth;
mod config;
mod crypto;
mod db;
mod kis;
mod state;
mod static_files;
mod symbols;

use config::Config;
use state::AppStateInner;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 라우터: `/api/*` REST, `/openapi.json` 스펙, `/docs` Swagger UI, 그 외 SPA.
fn build_app(state: state::AppState) -> impl poem::Endpoint {
    let api_service = OpenApiService::new(api::all(), "kis-web", VERSION).server("/api");
    let spec = api_service.spec_endpoint();
    let swagger = api_service.swagger_ui();

    // /api 하위: SSE 스트림(구체 경로)을 OpenApi 캐치올보다 먼저 매칭.
    let api = Route::new()
        .at("/quotes/stream", get(api::stream::quotes_stream))
        .nest("/", api_service);

    Route::new()
        .nest("/api", api)
        .at("/openapi.json", spec)
        .nest("/docs", swagger)
        .nest("/", static_files::endpoint())
        .with(CookieJarManager::new()) // req.cookie() (SSE 인증)용 — 쿠키 파싱
        .data(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;
    tracing::info!(bind = %config.bind, db = %config.db_path, version = VERSION, "kis-web 시작");

    let pool = db::init(&config.db_path).await?;
    let bind = config.bind.clone();

    // 종목 마스터 자동 동기화 (백그라운드, 비차단). 오프라인이어도 서버는 정상 기동.
    let symbols_path = config.symbols_db_path.clone();
    tokio::spawn(async move {
        match symbols::sync(symbols_path, true).await {
            Ok(0) => tracing::info!("종목 마스터: 최신 상태(skip)"),
            Ok(n) => tracing::info!("종목 마스터 동기화 완료: {n}건"),
            Err(e) => tracing::warn!("종목 마스터 동기화 실패(검색·종목명 제한): {e}"),
        }
    });

    let state = AppStateInner::new(pool, config);

    Server::new(TcpListener::bind(bind))
        .run(build_app(state))
        .await?;
    Ok(())
}
