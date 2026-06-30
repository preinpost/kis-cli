//! 매매일지 — 수동 매매 기록 CRUD + 통계.

use poem::http::StatusCode;
use poem::web::Data;
use poem::{Error, Result};
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use uuid::Uuid;

use crate::auth::SessionAuth;
use crate::positions::{stats_from_trades, TradeRow};
use crate::state::AppState;
use crate::symbols;

use super::ApiTag;

/// DB trades 행.
#[derive(sqlx::FromRow)]
struct TradeDbRow {
    id: String,
    traded_at: String,
    symbol: String,
    name: Option<String>,
    market: Option<String>,
    broker: Option<String>,
    side: String,
    quantity: f64,
    price: f64,
    fee: f64,
    currency: String,
    reason: Option<String>,
    tags: Option<String>,
    memo: Option<String>,
}

#[derive(Object)]
struct TradeDto {
    id: String,
    traded_at: String,
    symbol: String,
    name: Option<String>,
    market: Option<String>,
    broker: Option<String>,
    side: String,
    quantity: f64,
    price: f64,
    fee: f64,
    currency: String,
    reason: Option<String>,
    tags: Option<String>,
    memo: Option<String>,
}

impl From<TradeDbRow> for TradeDto {
    fn from(r: TradeDbRow) -> Self {
        TradeDto {
            id: r.id,
            traded_at: r.traded_at,
            symbol: r.symbol,
            name: r.name,
            market: r.market,
            broker: r.broker,
            side: r.side,
            quantity: r.quantity,
            price: r.price,
            fee: r.fee,
            currency: r.currency,
            reason: r.reason,
            tags: r.tags,
            memo: r.memo,
        }
    }
}

#[derive(Object)]
struct TradeInput {
    /// 체결일시 (RFC3339 또는 "2026-06-30").
    traded_at: String,
    /// 종목 코드/티커/이름 (KIS 해석 시도, 미해석도 허용).
    symbol: String,
    /// "buy" | "sell"
    side: String,
    quantity: f64,
    price: f64,
    #[oai(default)]
    fee: f64,
    broker: Option<String>,
    currency: Option<String>,
    reason: Option<String>,
    tags: Option<String>,
    memo: Option<String>,
}

#[derive(Object)]
struct CurrencyAmount {
    currency: String,
    amount: f64,
}

#[derive(Object)]
struct TradeStatsDto {
    trade_count: i64,
    buy_count: i64,
    sell_count: i64,
    win_count: i64,
    win_rate: f64, // 0~1 (sell 기준)
    symbol_count: i64,
    realized: Vec<CurrencyAmount>,
}

pub struct JournalApi;

#[OpenApi(prefix_path = "/trades", tag = "ApiTag::Journal")]
impl JournalApi {
    /// 매매 기록 목록 (필터: symbol/side/broker/from/to).
    #[oai(path = "/", method = "get")]
    async fn list(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        #[oai(name = "symbol")] symbol: Query<Option<String>>,
        #[oai(name = "side")] side: Query<Option<String>>,
        #[oai(name = "broker")] broker: Query<Option<String>>,
        #[oai(name = "from")] from: Query<Option<String>>,
        #[oai(name = "to")] to: Query<Option<String>>,
    ) -> Result<Json<Vec<TradeDto>>> {
        let st = state.0;
        let rows = fetch_trades(st, &auth.0.id, &symbol.0, &side.0, &broker.0, &from.0, &to.0)
            .await
            .map_err(internal)?;
        Ok(Json(rows.into_iter().map(TradeDto::from).collect()))
    }

    /// 매매 기록 추가.
    #[oai(path = "/", method = "post")]
    async fn create(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        body: Json<TradeInput>,
    ) -> Result<Json<TradeDto>> {
        let st = state.0;
        let inp = body.0;
        validate(&inp)?;

        // 종목 해석 → name/market/currency 스냅샷 (미해석이면 입력 그대로 보존)
        let info = symbols::resolve(st.config.symbols_db_path.clone(), inp.symbol.trim().to_string()).await;
        let (symbol, name, market) = if info.matched {
            (info.code, Some(info.name), Some(info.kind.to_string()))
        } else {
            let raw = inp.symbol.trim().to_string();
            (raw.clone(), Some(raw), Some("other".to_string()))
        };
        let currency = inp
            .currency
            .clone()
            .filter(|c| !c.trim().is_empty())
            .unwrap_or_else(|| if market.as_deref() == Some("overseas") { "USD".into() } else { "KRW".into() });

        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO trades (id, user_id, traded_at, symbol, name, market, broker, side, \
             quantity, price, fee, currency, reason, tags, memo, created_at) \
             VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(&id)
        .bind(&auth.0.id)
        .bind(inp.traded_at.trim())
        .bind(&symbol)
        .bind(&name)
        .bind(&market)
        .bind(inp.broker.as_deref())
        .bind(&inp.side)
        .bind(inp.quantity)
        .bind(inp.price)
        .bind(inp.fee)
        .bind(&currency)
        .bind(inp.reason.as_deref())
        .bind(inp.tags.as_deref())
        .bind(inp.memo.as_deref())
        .bind(&now)
        .execute(&st.db)
        .await
        .map_err(internal)?;

        let row = fetch_one(st, &auth.0.id, &id).await.map_err(internal)?;
        row.map(|r| Json(TradeDto::from(r)))
            .ok_or_else(|| internal("insert 후 조회 실패"))
    }

    /// 매매 기록 수정.
    #[oai(path = "/:id", method = "put")]
    async fn update(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        id: Path<String>,
        body: Json<TradeInput>,
    ) -> Result<Json<TradeDto>> {
        let st = state.0;
        let inp = body.0;
        validate(&inp)?;

        let info = symbols::resolve(st.config.symbols_db_path.clone(), inp.symbol.trim().to_string()).await;
        let (symbol, name, market) = if info.matched {
            (info.code, Some(info.name), Some(info.kind.to_string()))
        } else {
            let raw = inp.symbol.trim().to_string();
            (raw.clone(), Some(raw), Some("other".to_string()))
        };
        let currency = inp
            .currency
            .clone()
            .filter(|c| !c.trim().is_empty())
            .unwrap_or_else(|| if market.as_deref() == Some("overseas") { "USD".into() } else { "KRW".into() });

        let res = sqlx::query(
            "UPDATE trades SET traded_at=?, symbol=?, name=?, market=?, broker=?, side=?, \
             quantity=?, price=?, fee=?, currency=?, reason=?, tags=?, memo=? \
             WHERE id=? AND user_id=?",
        )
        .bind(inp.traded_at.trim())
        .bind(&symbol)
        .bind(&name)
        .bind(&market)
        .bind(inp.broker.as_deref())
        .bind(&inp.side)
        .bind(inp.quantity)
        .bind(inp.price)
        .bind(inp.fee)
        .bind(&currency)
        .bind(inp.reason.as_deref())
        .bind(inp.tags.as_deref())
        .bind(inp.memo.as_deref())
        .bind(&id.0)
        .bind(&auth.0.id)
        .execute(&st.db)
        .await
        .map_err(internal)?;

        if res.rows_affected() == 0 {
            return Err(Error::from_string("기록을 찾을 수 없습니다", StatusCode::NOT_FOUND));
        }
        let row = fetch_one(st, &auth.0.id, &id.0).await.map_err(internal)?;
        row.map(|r| Json(TradeDto::from(r)))
            .ok_or_else(|| internal("update 후 조회 실패"))
    }

    /// 매매 기록 삭제.
    #[oai(path = "/:id", method = "delete")]
    async fn delete(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        id: Path<String>,
    ) -> Result<Json<super::OkDto>> {
        let st = state.0;
        sqlx::query("DELETE FROM trades WHERE id=? AND user_id=?")
            .bind(&id.0)
            .bind(&auth.0.id)
            .execute(&st.db)
            .await
            .map_err(internal)?;
        Ok(Json(super::OkDto { ok: true }))
    }

    /// 매매 통계 (실현손익·승률·횟수).
    #[oai(path = "/stats", method = "get")]
    async fn stats(
        &self,
        state: Data<&AppState>,
        auth: SessionAuth,
        #[oai(name = "from")] from: Query<Option<String>>,
        #[oai(name = "to")] to: Query<Option<String>>,
    ) -> Result<Json<TradeStatsDto>> {
        let st = state.0;
        let rows = fetch_trades(st, &auth.0.id, &None, &None, &None, &from.0, &to.0)
            .await
            .map_err(internal)?;
        let calc: Vec<TradeRow> = rows.iter().map(to_calc_row).collect();
        let s = stats_from_trades(&calc);
        Ok(Json(TradeStatsDto {
            trade_count: s.trade_count as i64,
            buy_count: s.buy_count as i64,
            sell_count: s.sell_count as i64,
            win_count: s.win_count as i64,
            win_rate: if s.sell_count > 0 {
                s.win_count as f64 / s.sell_count as f64
            } else {
                0.0
            },
            symbol_count: s.symbol_count as i64,
            realized: s
                .realized_by_currency
                .into_iter()
                .map(|(currency, amount)| CurrencyAmount { currency, amount })
                .collect(),
        }))
    }
}

fn validate(inp: &TradeInput) -> Result<()> {
    if inp.side != "buy" && inp.side != "sell" {
        return Err(Error::from_string("side 는 buy 또는 sell", StatusCode::BAD_REQUEST));
    }
    if inp.quantity <= 0.0 || inp.price < 0.0 {
        return Err(Error::from_string("수량/가격이 올바르지 않습니다", StatusCode::BAD_REQUEST));
    }
    if inp.traded_at.trim().is_empty() {
        return Err(Error::from_string("체결일시를 입력하세요", StatusCode::BAD_REQUEST));
    }
    Ok(())
}

fn to_calc_row(r: &TradeDbRow) -> TradeRow {
    TradeRow {
        traded_at: r.traded_at.clone(),
        symbol: r.symbol.clone(),
        name: r.name.clone(),
        market: r.market.clone(),
        broker: r.broker.clone(),
        side: r.side.clone(),
        quantity: r.quantity,
        price: r.price,
        fee: r.fee,
        currency: r.currency.clone(),
    }
}

async fn fetch_trades(
    st: &AppState,
    user_id: &str,
    symbol: &Option<String>,
    side: &Option<String>,
    broker: &Option<String>,
    from: &Option<String>,
    to: &Option<String>,
) -> anyhow::Result<Vec<TradeDbRow>> {
    let mut qb = sqlx::QueryBuilder::<sqlx::Sqlite>::new(
        "SELECT id, traded_at, symbol, name, market, broker, side, quantity, price, fee, \
         currency, reason, tags, memo FROM trades WHERE user_id = ",
    );
    qb.push_bind(user_id.to_string());
    if let Some(v) = nonempty(symbol) {
        qb.push(" AND symbol = ").push_bind(v);
    }
    if let Some(v) = nonempty(side) {
        qb.push(" AND side = ").push_bind(v);
    }
    if let Some(v) = nonempty(broker) {
        qb.push(" AND broker = ").push_bind(v);
    }
    if let Some(v) = nonempty(from) {
        qb.push(" AND traded_at >= ").push_bind(v);
    }
    if let Some(v) = nonempty(to) {
        qb.push(" AND traded_at <= ").push_bind(v);
    }
    qb.push(" ORDER BY traded_at DESC, created_at DESC");
    Ok(qb.build_query_as::<TradeDbRow>().fetch_all(&st.db).await?)
}

async fn fetch_one(st: &AppState, user_id: &str, id: &str) -> anyhow::Result<Option<TradeDbRow>> {
    Ok(sqlx::query_as::<_, TradeDbRow>(
        "SELECT id, traded_at, symbol, name, market, broker, side, quantity, price, fee, \
         currency, reason, tags, memo FROM trades WHERE id=? AND user_id=?",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(&st.db)
    .await?)
}

fn nonempty(o: &Option<String>) -> Option<String> {
    o.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

fn internal<E: std::fmt::Display>(e: E) -> Error {
    tracing::error!("journal internal error: {e}");
    Error::from_string("서버 오류", StatusCode::INTERNAL_SERVER_ERROR)
}
