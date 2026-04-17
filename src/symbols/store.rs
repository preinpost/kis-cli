//! SQLite + FTS5 기반 종목 저장소.
//!
//! 스키마:
//! - `symbols(code, market, name_kr, name_en, exchange)` — primary key: (code, market)
//! - `symbols_fts` — FTS5 virtual table (name_kr, name_en, code)
//! - `meta(key, value)` — 마지막 동기화 시각 등

use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};

/// 지원하는 시장 구분.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Market {
    /// 국내 KOSPI (6자리 숫자)
    Kospi,
    /// 국내 KOSDAQ (6자리 숫자)
    Kosdaq,
    /// 미국 NASDAQ
    Nasdaq,
    /// 미국 NYSE
    Nyse,
    /// 미국 AMEX
    Amex,
    /// KRX 지수선물옵션 (주간 단축코드 — 야간선물에도 통용 시도)
    FoIdx,
    /// KRX 주식선물옵션
    FoStk,
}

impl Market {
    pub fn as_str(&self) -> &'static str {
        match self {
            Market::Kospi => "KOSPI",
            Market::Kosdaq => "KOSDAQ",
            Market::Nasdaq => "NASD",
            Market::Nyse => "NYSE",
            Market::Amex => "AMEX",
            Market::FoIdx => "FOIDX",
            Market::FoStk => "FOSTK",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "KOSPI" => Some(Market::Kospi),
            "KOSDAQ" => Some(Market::Kosdaq),
            "NASD" | "NASDAQ" => Some(Market::Nasdaq),
            "NYSE" => Some(Market::Nyse),
            "AMEX" => Some(Market::Amex),
            "FOIDX" => Some(Market::FoIdx),
            "FOSTK" => Some(Market::FoStk),
            _ => None,
        }
    }

    pub fn is_domestic(&self) -> bool {
        matches!(self, Market::Kospi | Market::Kosdaq)
    }

    pub fn is_overseas(&self) -> bool {
        matches!(self, Market::Nasdaq | Market::Nyse | Market::Amex)
    }

    pub fn is_futureoption(&self) -> bool {
        matches!(self, Market::FoIdx | Market::FoStk)
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub code: String,
    pub market: Market,
    pub name_kr: String,
    pub name_en: String,
}

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)
            .with_context(|| format!("symbols DB 열기 실패: {}", path.display()))?;
        let store = Store { conn };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS symbols (
                code    TEXT NOT NULL,
                market  TEXT NOT NULL,
                name_kr TEXT NOT NULL DEFAULT '',
                name_en TEXT NOT NULL DEFAULT '',
                PRIMARY KEY (code, market)
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS symbols_fts USING fts5(
                code, name_kr, name_en, market UNINDEXED,
                tokenize = 'unicode61 remove_diacritics 2'
            );
            CREATE TABLE IF NOT EXISTS meta (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "#,
        )?;
        Ok(())
    }

    /// 한 시장의 심볼을 전체 교체 (동기화).
    pub fn replace_market(&mut self, market: Market, symbols: &[Symbol]) -> Result<usize> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "DELETE FROM symbols WHERE market = ?1",
            params![market.as_str()],
        )?;
        tx.execute(
            "DELETE FROM symbols_fts WHERE market = ?1",
            params![market.as_str()],
        )?;

        let mut n = 0usize;
        {
            let mut ins = tx.prepare(
                "INSERT INTO symbols (code, market, name_kr, name_en) VALUES (?1, ?2, ?3, ?4)",
            )?;
            let mut ins_fts = tx.prepare(
                "INSERT INTO symbols_fts (code, name_kr, name_en, market) VALUES (?1, ?2, ?3, ?4)",
            )?;
            for s in symbols {
                ins.execute(params![s.code, market.as_str(), s.name_kr, s.name_en])?;
                ins_fts.execute(params![s.code, s.name_kr, s.name_en, market.as_str()])?;
                n += 1;
            }
        }
        tx.commit()?;
        Ok(n)
    }

    /// 정확한 코드로 조회. 시장 힌트 없으면 여러 시장에서 찾아 중복 가능.
    pub fn find_by_code(&self, code: &str) -> Result<Vec<Symbol>> {
        let mut stmt = self.conn.prepare(
            "SELECT code, market, name_kr, name_en FROM symbols WHERE code = ?1",
        )?;
        let rows = stmt.query_map(params![code], |r| {
            Ok(Symbol {
                code: r.get(0)?,
                market: Market::from_str(&r.get::<_, String>(1)?).unwrap_or(Market::Kospi),
                name_kr: r.get(2)?,
                name_en: r.get(3)?,
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// FTS5 검색. `query`는 공백 분리 AND. 최대 `limit`개 반환.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<Symbol>> {
        let q = fts_sanitize(query);
        if q.is_empty() {
            return Ok(Vec::new());
        }
        let mut stmt = self.conn.prepare(
            "SELECT code, market, name_kr, name_en
             FROM symbols_fts
             WHERE symbols_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![q, limit as i64], |r| {
            Ok(Symbol {
                code: r.get(0)?,
                market: Market::from_str(&r.get::<_, String>(1)?).unwrap_or(Market::Kospi),
                name_kr: r.get(2)?,
                name_en: r.get(3)?,
            })
        })?;
        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO meta(key,value) VALUES(?1,?2)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row(
                "SELECT value FROM meta WHERE key = ?1",
                params![key],
                |r| r.get::<_, String>(0),
            )
            .optional()?)
    }

    pub fn count(&self, market: Market) -> Result<i64> {
        let n = self.conn.query_row(
            "SELECT COUNT(*) FROM symbols WHERE market = ?1",
            params![market.as_str()],
            |r| r.get::<_, i64>(0),
        )?;
        Ok(n)
    }
}

/// FTS5 쿼리 이스케이프 — 사용자 입력을 토큰별 prefix 매칭으로 변환.
fn fts_sanitize(input: &str) -> String {
    let tokens: Vec<String> = input
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .map(|t| {
            let escaped = t.replace('"', "\"\"");
            format!("\"{}\"*", escaped)
        })
        .collect();
    tokens.join(" ")
}
