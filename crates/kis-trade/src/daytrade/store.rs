//! 데이트레이드 매매 기록 SQLite 저장소.
//!
//! paper/run 모드에서 체결 시마다 한 건씩 기록. 세션 종료 시 일일 리포트 집계용.
//!
//! 스키마:
//! - `trades(id, session_id, symbol, market, side, qty, price, ts, strategy, mode, pnl, pnl_pct, reason)`

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use rusqlite::{params, params_from_iter, types::ToSqlOutput, Connection, ToSql};
use serde::Serialize;
use std::path::Path;
use std::sync::Mutex;

/// Connection을 Mutex로 감싸 `&Storage` 를 await 경계 너머로 보낼 수 있게 한다 (Send + Sync).
/// daemon에서 multi-task 가 같은 DB를 쓰지만 SQLite는 직렬화된 쓰기로 충분.
pub struct Storage {
    conn: Mutex<Connection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    fn as_str(self) -> &'static str {
        match self {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Paper,
    Run,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Paper => "paper",
            Mode::Run => "run",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradeInsert<'a> {
    pub session_id: &'a str,
    pub symbol: &'a str,
    pub market: &'a str,
    pub side: Side,
    pub qty: u64,
    pub price: f64,
    pub ts: DateTime<Tz>,
    pub strategy: &'a str,
    pub mode: Mode,
    pub pnl: Option<f64>,
    pub pnl_pct: Option<f64>,
    pub reason: &'a str,
}

#[derive(Debug, Clone, Default)]
pub struct SessionSummary {
    pub trades: usize,
    pub sells: usize,
    pub wins: usize,
    pub total_pnl: f64,
    pub avg_pnl_pct: f64,
}

impl Storage {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(path)
            .with_context(|| format!("daytrade DB 열기 실패: {}", path.display()))?;
        let s = Storage { conn: Mutex::new(conn) };
        s.init_schema()?;
        Ok(s)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS trades (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id  TEXT NOT NULL,
                symbol      TEXT NOT NULL,
                market      TEXT NOT NULL,
                side        TEXT NOT NULL,
                qty         INTEGER NOT NULL,
                price       REAL NOT NULL,
                ts          TEXT NOT NULL,
                strategy    TEXT NOT NULL,
                mode        TEXT NOT NULL,
                pnl         REAL,
                pnl_pct     REAL,
                reason      TEXT NOT NULL DEFAULT ''
            );
            CREATE INDEX IF NOT EXISTS idx_trades_session ON trades(session_id);
            CREATE INDEX IF NOT EXISTS idx_trades_ts      ON trades(ts);
            "#,
        )?;
        Ok(())
    }

    pub fn insert_trade(&self, t: &TradeInsert) -> Result<i64> {
        // UTC 저장 (세션 간 비교 일관성)
        let ts_utc: DateTime<Utc> = t.ts.with_timezone(&Utc);
        let ts_str = ts_utc.to_rfc3339();
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO trades
                (session_id, symbol, market, side, qty, price, ts, strategy, mode, pnl, pnl_pct, reason)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
            params![
                t.session_id,
                t.symbol,
                t.market,
                t.side.as_str(),
                t.qty as i64,
                t.price,
                ts_str,
                t.strategy,
                t.mode.as_str(),
                t.pnl,
                t.pnl_pct,
                t.reason,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn session_summary(&self, session_id: &str) -> Result<SessionSummary> {
        let conn = self.conn.lock().unwrap();
        let mut s = SessionSummary::default();
        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM trades WHERE session_id = ?1",
        )?;
        s.trades = stmt.query_row([session_id], |r| r.get::<_, i64>(0))? as usize;

        let mut stmt = conn.prepare(
            "SELECT COUNT(*), COALESCE(SUM(pnl),0), COALESCE(AVG(pnl_pct),0)
               FROM trades
              WHERE session_id = ?1 AND side = 'SELL'",
        )?;
        let (sells, total_pnl, avg_pnl_pct): (i64, f64, f64) =
            stmt.query_row([session_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        s.sells = sells as usize;
        s.total_pnl = total_pnl;
        s.avg_pnl_pct = avg_pnl_pct;

        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM trades WHERE session_id = ?1 AND side = 'SELL' AND pnl > 0",
        )?;
        s.wins = stmt.query_row([session_id], |r| r.get::<_, i64>(0))? as usize;
        Ok(s)
    }

    /// 최근 세션 요약 — 완료 시각(MAX ts) 내림차순.
    pub fn recent_sessions(&self, limit: usize) -> Result<Vec<SessionRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT session_id,
                    MAX(market), MAX(symbol), MAX(strategy),
                    MIN(ts), MAX(ts),
                    COUNT(*),
                    SUM(CASE WHEN side='SELL' THEN 1 ELSE 0 END),
                    SUM(CASE WHEN side='SELL' AND pnl > 0 THEN 1 ELSE 0 END),
                    COALESCE(SUM(CASE WHEN side='SELL' THEN pnl ELSE 0 END), 0),
                    COALESCE(AVG(CASE WHEN side='SELL' THEN pnl_pct END), 0)
               FROM trades
              GROUP BY session_id
              ORDER BY MAX(ts) DESC
              LIMIT ?1",
        )?;
        let rows = stmt.query_map([limit as i64], |r| {
            Ok(SessionRow {
                session_id: r.get(0)?,
                market: r.get(1)?,
                symbol: r.get(2)?,
                strategy: r.get(3)?,
                start_ts: parse_ts(r, 4)?,
                end_ts: parse_ts(r, 5)?,
                trades: r.get::<_, i64>(6)? as usize,
                sells: r.get::<_, i64>(7)? as usize,
                wins: r.get::<_, i64>(8)? as usize,
                total_pnl: r.get(9)?,
                avg_pnl_pct: r.get(10)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// 특정 세션의 체결 내역 (ts 오름차순).
    pub fn trades_for_session(&self, session_id: &str) -> Result<Vec<TradeRow>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, session_id, symbol, market, side, qty, price, ts,
                    strategy, mode, pnl, pnl_pct, reason
               FROM trades WHERE session_id = ?1 ORDER BY ts ASC",
        )?;
        let rows = stmt.query_map([session_id], map_trade)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// 종목/기간 필터 체결 내역 (ts 내림차순 — 최신 먼저).
    pub fn trades_filtered(
        &self,
        symbol: Option<&str>,
        since: Option<DateTime<Utc>>,
    ) -> Result<Vec<TradeRow>> {
        let mut sql = String::from(
            "SELECT id, session_id, symbol, market, side, qty, price, ts,
                    strategy, mode, pnl, pnl_pct, reason
               FROM trades WHERE 1=1",
        );
        let mut bind: Vec<BindVal> = Vec::new();
        if let Some(s) = symbol {
            sql.push_str(" AND symbol = ?");
            bind.push(BindVal::Text(s.to_string()));
        }
        if let Some(t) = since {
            sql.push_str(" AND ts >= ?");
            bind.push(BindVal::Text(t.to_rfc3339()));
        }
        sql.push_str(" ORDER BY ts DESC");

        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_from_iter(bind.iter()), map_trade)?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeRow {
    pub id: i64,
    pub session_id: String,
    pub symbol: String,
    pub market: String,
    pub side: String,
    pub qty: u64,
    pub price: f64,
    pub ts: DateTime<Utc>,
    pub strategy: String,
    pub mode: String,
    pub pnl: Option<f64>,
    pub pnl_pct: Option<f64>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SessionRow {
    pub session_id: String,
    pub market: String,
    pub symbol: String,
    pub strategy: String,
    pub start_ts: DateTime<Utc>,
    pub end_ts: DateTime<Utc>,
    pub trades: usize,
    pub sells: usize,
    pub wins: usize,
    pub total_pnl: f64,
    pub avg_pnl_pct: f64,
}

fn map_trade(r: &rusqlite::Row) -> rusqlite::Result<TradeRow> {
    Ok(TradeRow {
        id: r.get(0)?,
        session_id: r.get(1)?,
        symbol: r.get(2)?,
        market: r.get(3)?,
        side: r.get(4)?,
        qty: r.get::<_, i64>(5)? as u64,
        price: r.get(6)?,
        ts: parse_ts(r, 7)?,
        strategy: r.get(8)?,
        mode: r.get(9)?,
        pnl: r.get(10)?,
        pnl_pct: r.get(11)?,
        reason: r.get(12)?,
    })
}

fn parse_ts(r: &rusqlite::Row, idx: usize) -> rusqlite::Result<DateTime<Utc>> {
    let s: String = r.get(idx)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(
                idx,
                rusqlite::types::Type::Text,
                Box::new(e),
            )
        })
}

// params_from_iter 용 동적 바인딩 — 문자열만 쓰므로 Text 단일 variant.
enum BindVal {
    Text(String),
}

impl ToSql for BindVal {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            BindVal::Text(s) => s.to_sql(),
        }
    }
}
