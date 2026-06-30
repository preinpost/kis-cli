//! 종목 검색 — kis-data symbols.db FTS.

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};

use crate::auth::SessionAuth;
use crate::state::AppState;
use crate::symbols;

use super::ApiTag;

#[derive(Object)]
struct SymbolDto {
    code: String,
    name: String,
    name_en: String,
    kind: String,         // "domestic" | "overseas" | "fo"
    market_label: String, // "KOSPI" 등
}

#[derive(Object)]
struct SyncResult {
    synced: i64,
}

pub struct SymbolsApi;

#[OpenApi(prefix_path = "/symbols", tag = "ApiTag::Symbols")]
impl SymbolsApi {
    /// 종목 검색(이름·코드). symbols.db 미존재 시 빈 결과.
    #[oai(path = "/search", method = "get")]
    async fn search(
        &self,
        state: Data<&AppState>,
        _auth: SessionAuth,
        #[oai(name = "q")] q: Query<String>,
        #[oai(name = "limit")] limit: Query<Option<u32>>,
    ) -> Result<Json<Vec<SymbolDto>>> {
        let st = state.0;
        let limit = limit.0.unwrap_or(20).clamp(1, 50) as usize;

        let rows = symbols::search(st.config.symbols_db_path.clone(), q.0, limit)
            .await
            .map_err(|e| {
                tracing::error!("symbol search 실패: {e}");
                Error::from_string("종목 검색 실패", StatusCode::INTERNAL_SERVER_ERROR)
            })?;

        Ok(Json(
            rows.into_iter()
                .map(|s| SymbolDto {
                    code: s.code,
                    name: s.name,
                    name_en: s.name_en,
                    kind: s.kind.to_string(),
                    market_label: s.market_label.to_string(),
                })
                .collect(),
        ))
    }

    /// 종목 마스터 강제 동기화 (공개 마스터 다운로드, 인증 불필요).
    #[oai(path = "/sync", method = "post")]
    async fn sync(
        &self,
        state: Data<&AppState>,
        _auth: SessionAuth,
    ) -> Result<Json<SyncResult>> {
        let st = state.0;
        let n = symbols::sync(st.config.symbols_db_path.clone(), false)
            .await
            .map_err(|e| {
                tracing::error!("symbol sync 실패: {e}");
                Error::from_string(format!("종목 동기화 실패: {e}"), StatusCode::BAD_GATEWAY)
            })?;
        Ok(Json(SyncResult { synced: n as i64 }))
    }
}
