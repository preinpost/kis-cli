//! 관심종목(watchlist) CRUD. 종목 코드+시장만 저장하고, 가격/이름은 프론트가 /quotes 로 가져온다.

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

use crate::auth::SessionAuth;
use crate::state::AppState;
use crate::symbols;

use super::ApiTag;

#[derive(Object)]
struct WatchlistItem {
    symbol: String,
    market: String, // "domestic" | "overseas"
}

#[derive(Object)]
struct AddReq {
    symbol: String,
}

pub struct WatchlistApi;

#[OpenApi(prefix_path = "/watchlist", tag = "ApiTag::Watchlist")]
impl WatchlistApi {
    /// 내 관심종목 목록.
    #[oai(path = "/", method = "get")]
    async fn list(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
    ) -> Result<Json<Vec<WatchlistItem>>> {
        let st = state.0;
        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT symbol, market FROM watchlist WHERE user_id = ? ORDER BY added_at",
        )
        .bind(&auth.0.id)
        .fetch_all(&st.db)
        .await
        .map_err(internal)?;

        Ok(Json(
            rows.into_iter()
                .map(|(symbol, market)| WatchlistItem { symbol, market })
                .collect(),
        ))
    }

    /// 관심종목 추가 (코드/티커 → 정규화 후 저장).
    #[oai(path = "/", method = "post")]
    async fn add(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        body: Json<AddReq>,
    ) -> Result<Json<WatchlistItem>> {
        let st = state.0;
        let input = body.0.symbol.trim().to_string();
        if input.is_empty() {
            return Err(Error::from_string("종목을 입력하세요", StatusCode::BAD_REQUEST));
        }

        // 한글 등 비-ASCII 입력이 매칭 실패하면(폴백) 해외 쓰레기로 추가하지 않고 거부.
        let has_non_ascii = input.chars().any(|c| !c.is_ascii());
        let info = symbols::resolve(st.config.symbols_db_path.clone(), input).await;
        if !info.matched && has_non_ascii {
            return Err(Error::from_string(
                "종목을 찾을 수 없습니다. 검색 후 목록에서 선택하세요.",
                StatusCode::NOT_FOUND,
            ));
        }
        let market = if info.kind == "fo" { "domestic" } else { info.kind };

        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO watchlist (user_id, symbol, market, sort, added_at) VALUES (?, ?, ?, 0, ?) \
             ON CONFLICT(user_id, symbol) DO NOTHING",
        )
        .bind(&auth.0.id)
        .bind(&info.code)
        .bind(market)
        .bind(&now)
        .execute(&st.db)
        .await
        .map_err(internal)?;

        Ok(Json(WatchlistItem {
            symbol: info.code,
            market: market.to_string(),
        }))
    }

    /// 관심종목 삭제.
    #[oai(path = "/:symbol", method = "delete")]
    async fn remove(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        symbol: Path<String>,
    ) -> Result<Json<super::OkDto>> {
        let st = state.0;
        sqlx::query("DELETE FROM watchlist WHERE user_id = ? AND symbol = ?")
            .bind(&auth.0.id)
            .bind(&symbol.0)
            .execute(&st.db)
            .await
            .map_err(internal)?;
        Ok(Json(super::OkDto { ok: true }))
    }
}

fn internal<E: std::fmt::Display>(e: E) -> Error {
    tracing::error!("watchlist internal error: {e}");
    Error::from_string("서버 오류", StatusCode::INTERNAL_SERVER_ERROR)
}
