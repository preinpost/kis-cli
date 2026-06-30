//! 실시간 시세 SSE 엔드포인트. KIS WebSocket(run_stream) 틱을 브라우저로 릴레이.
//!
//! poem-openapi(JSON 페이로드)가 아닌 일반 poem 핸들러 — SSE 스트리밍 응답이라서.
//! 인증은 세션 쿠키를 직접 조회한다(SecurityScheme 미사용).

use std::time::Duration;

use poem::http::StatusCode;
use poem::web::sse::{Event, SSE};
use poem::web::Query;
use poem::{handler, Error, Request, Result};
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use kis_core::ws::{self, Sub, Tick};

use crate::auth::session;
use crate::state::AppState;
use crate::symbols;

#[derive(Deserialize)]
struct StreamQuery {
    /// 쉼표 구분 종목들 (예: 005930,000660,TSLA)
    symbols: String,
}

/// SSE 스트림 Drop 시 WS 연결을 닫기 위한 가드.
struct CancelGuard(CancellationToken);
impl Drop for CancelGuard {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

/// `GET /api/quotes/stream?symbols=005930,TSLA` — 실시간 체결 틱을 SSE 로.
#[handler]
pub async fn quotes_stream(req: &Request, Query(q): Query<StreamQuery>) -> Result<SSE> {
    let state = req
        .data::<AppState>()
        .ok_or_else(|| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .clone();

    // 세션 쿠키로 인증
    let token = req
        .cookie()
        .get(session::COOKIE_NAME)
        .map(|c| c.value_str().to_string());
    let user = match token {
        Some(t) => session::lookup(&state.db, &t).await.ok().flatten(),
        None => None,
    };
    let Some(user) = user else {
        return Err(Error::from_status(StatusCode::UNAUTHORIZED));
    };

    // 사용자 KisClient (자격증명 필요)
    let client = state
        .clients
        .get(&state.db, &state.config.master_key, &user.id)
        .await
        .map_err(|e| {
            tracing::error!("stream client error: {e}");
            Error::from_string("서버 오류", StatusCode::INTERNAL_SERVER_ERROR)
        })?
        .ok_or_else(|| Error::from_string("KIS 자격증명을 먼저 등록하세요", StatusCode::CONFLICT))?;

    // 종목 → 구독(국내/해외) 변환
    let mut subs: Vec<Sub> = Vec::new();
    for raw in q.symbols.split(',') {
        let raw = raw.trim();
        if raw.is_empty() {
            continue;
        }
        let info = symbols::resolve(state.config.symbols_db_path.clone(), raw.to_string()).await;
        match info.kind {
            "domestic" => subs.push(Sub::Domestic(info.code)),
            "overseas" => subs.push(Sub::Overseas {
                excd: if info.excd.is_empty() { "NAS".into() } else { info.excd.to_string() },
                symbol: info.code,
            }),
            _ => {}
        }
    }
    if subs.is_empty() {
        return Err(Error::from_string("구독할 종목이 없습니다", StatusCode::BAD_REQUEST));
    }

    // 채널 + 취소 토큰으로 run_stream 백그라운드 실행
    let (tx, rx) = mpsc::channel::<Tick>(256);
    let cancel = CancellationToken::new();
    let tm = client.token_manager.clone();
    let cancel_bg = cancel.clone();
    tokio::spawn(async move {
        if let Err(e) = ws::run_stream(tm, subs, tx, cancel_bg).await {
            tracing::warn!("실시간 스트림 종료: {e}");
        }
    });

    // 수신 채널 → SSE 이벤트. 스트림 Drop(클라 종료) 시 guard 가 cancel → WS 종료.
    let guard = CancelGuard(cancel);
    let stream = futures_util::stream::unfold((rx, guard), |(mut rx, guard)| async move {
        rx.recv().await.map(|tick| {
            let data = serde_json::to_string(&tick).unwrap_or_default();
            (Event::message(data), (rx, guard))
        })
    });

    Ok(SSE::new(stream).keep_alive(Duration::from_secs(15)))
}
