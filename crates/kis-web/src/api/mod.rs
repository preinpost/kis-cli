//! poem-openapi API 표면. 각 도메인별 #[OpenApi] 를 튜플로 합쳐 한 서비스로 노출.

pub mod account;
pub mod auth;
pub mod journal;
pub mod portfolio;
pub mod quotes;
pub mod stream;
pub mod symbols;
pub mod watchlist;

use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi, Tags};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Tags)]
pub enum ApiTag {
    /// 인증 (회원가입·로그인·세션)
    Auth,
    /// KIS 자격증명
    Account,
    /// 포트폴리오
    Portfolio,
    /// 현재가
    Quotes,
    /// 종목 검색
    Symbols,
    /// 관심종목
    Watchlist,
    /// 매매일지
    Journal,
    /// 시스템 상태
    System,
}

/// 단순 성공 응답 (여러 엔드포인트 공용 — 스키마 이름 중복 방지).
#[derive(Object)]
pub struct OkDto {
    pub ok: bool,
}

#[derive(Object)]
struct Health {
    status: String,
    version: String,
}

pub struct SystemApi;

#[OpenApi(tag = "ApiTag::System")]
impl SystemApi {
    /// 서버 헬스체크.
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<Health> {
        Json(Health {
            status: "ok".to_string(),
            version: VERSION.to_string(),
        })
    }
}

/// 모든 API 모듈을 합친 튜플 (OpenApiService::new 에 전달).
pub fn all() -> (
    SystemApi,
    auth::AuthApi,
    account::AccountApi,
    portfolio::PortfolioApi,
    quotes::QuotesApi,
    symbols::SymbolsApi,
    watchlist::WatchlistApi,
    journal::JournalApi,
) {
    (
        SystemApi,
        auth::AuthApi,
        account::AccountApi,
        portfolio::PortfolioApi,
        quotes::QuotesApi,
        symbols::SymbolsApi,
        watchlist::WatchlistApi,
        journal::JournalApi,
    )
}
