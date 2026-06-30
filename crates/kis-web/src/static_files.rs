//! 빌드된 Vite SPA(`web/dist`) 정적 서빙 + SPA 폴백.
//!
//! 존재하는 파일은 그대로, 그 외 경로(클라이언트 라우트 `/login` 등)는 `index.html` 로 폴백해
//! TanStack Router 가 클라이언트에서 처리하게 한다. `/api`·`/openapi.json`·`/docs` 는
//! 더 구체적인 라우트가 먼저 매칭되므로 여기로 오지 않는다.

use poem::{get, handler, http::StatusCode, web::Path, IntoResponse, Response, Route};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/dist"]
struct Assets;

fn serve(path: &str) -> Response {
    let path = if path.is_empty() { "index.html" } else { path };
    if let Some(file) = Assets::get(path) {
        return Response::builder()
            .header("Content-Type", file.metadata.mimetype())
            .body(file.data.into_owned());
    }
    // SPA 폴백 — 알 수 없는 경로는 index.html (클라이언트 라우팅)
    match Assets::get("index.html") {
        Some(index) => Response::builder()
            .header("Content-Type", "text/html; charset=utf-8")
            .body(index.data.into_owned()),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[handler]
fn index_handler() -> Response {
    serve("index.html")
}

#[handler]
fn asset_handler(Path(path): Path<String>) -> Response {
    serve(&path)
}

/// `/` 와 그 하위 모든 경로를 SPA 로 서빙하는 라우터.
pub fn endpoint() -> Route {
    Route::new()
        .at("/", get(index_handler))
        .at("/*path", get(asset_handler))
}
