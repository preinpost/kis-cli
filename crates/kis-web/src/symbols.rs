//! 종목 마스터(symbols.db) 읽기 헬퍼. kis-data Store(rusqlite, 동기)를 spawn_blocking 으로 감싼다.

use std::path::Path;

use anyhow::Result;
use kis_data::symbols::store::{Market, Store};

/// 검색 결과/해석에 쓰는 가벼운 종목 표현.
#[derive(Clone)]
pub struct SymbolInfo {
    pub code: String,
    pub name: String,
    pub name_en: String,
    /// "domestic" | "overseas" | "fo"
    pub kind: &'static str,
    /// "KOSPI"/"KOSDAQ"/"NASDAQ"/... 표시용
    pub market_label: &'static str,
    /// 해외 시세 excd ("NAS"/"NYS"/"AMS"), 그 외 ""
    pub excd: &'static str,
    /// symbols.db 에서 실제 매칭됐는지(false = 휴리스틱 폴백).
    pub matched: bool,
}

fn kind_of(m: Market) -> &'static str {
    if m.is_domestic() {
        "domestic"
    } else if m.is_overseas() {
        "overseas"
    } else {
        "fo"
    }
}

fn label_of(m: Market) -> &'static str {
    match m {
        Market::Kospi => "KOSPI",
        Market::Kosdaq => "KOSDAQ",
        Market::Nasdaq => "NASDAQ",
        Market::Nyse => "NYSE",
        Market::Amex => "AMEX",
        Market::FoIdx => "선물옵션",
        Market::FoStk => "주식선물옵션",
    }
}

fn from_symbol(s: kis_data::symbols::store::Symbol) -> SymbolInfo {
    SymbolInfo {
        code: s.code,
        name: s.name_kr,
        name_en: s.name_en,
        kind: kind_of(s.market),
        market_label: label_of(s.market),
        excd: if s.market.is_overseas() { s.market.excd() } else { "" },
        matched: true,
    }
}

/// 검색. FTS 우선, 결과 없으면 부분일치(LIKE) 폴백. symbols.db 없거나 비면 빈 결과.
pub async fn search(db_path: String, query: String, limit: usize) -> Result<Vec<SymbolInfo>> {
    tokio::task::spawn_blocking(move || {
        if !Path::new(&db_path).exists() {
            return Ok(Vec::new());
        }
        let store = Store::open(Path::new(&db_path))?;
        let mut rows = store.search(&query, limit)?;
        if rows.is_empty() {
            rows = store.search_like(&query, limit)?;
        }
        Ok(rows.into_iter().map(from_symbol).collect())
    })
    .await?
}

/// 코드/티커로 종목 해석. DB에 없으면 휴리스틱 폴백(6자리 숫자→국내, 그 외→해외 NAS).
pub async fn resolve(db_path: String, input: String) -> SymbolInfo {
    let fallback_input = input.clone();
    tokio::task::spawn_blocking(move || resolve_blocking(&db_path, &input))
        .await
        .unwrap_or_else(|_| fallback(&fallback_input))
}

fn resolve_blocking(db_path: &str, input: &str) -> SymbolInfo {
    if Path::new(db_path).exists() {
        if let Ok(store) = Store::open(Path::new(db_path)) {
            // 1) 코드/티커 정확 매칭
            for candidate in [input.to_string(), input.to_uppercase()] {
                if let Ok(mut rows) = store.find_by_code(&candidate) {
                    if let Some(first) = rows.drain(..).next() {
                        return from_symbol(first);
                    }
                }
            }
            // 2) 이름 검색 (예: "삼성전자" → 005930). FTS → LIKE 폴백. 첫 결과 채택.
            if let Ok(mut rows) = store.search(input, 1) {
                if let Some(first) = rows.drain(..).next() {
                    return from_symbol(first);
                }
            }
            if let Ok(mut rows) = store.search_like(input, 1) {
                if let Some(first) = rows.drain(..).next() {
                    return from_symbol(first);
                }
            }
        }
    }
    fallback(input)
}

/// 종목 마스터 동기화 (공개 마스터 파일 다운로드, 인증 불필요).
/// `if_stale=true` 면 24시간 이내 동기화 시 skip. 반환: 동기화된 총 종목 수.
pub async fn sync(db_path: String, if_stale: bool) -> Result<usize> {
    // sync_all 은 자체 reqwest 로 다운로드. 부모 디렉터리는 Store::open 이 생성.
    let report =
        kis_data::symbols::sync::sync_all(Path::new(&db_path), if_stale).await?;
    Ok(report.results.iter().map(|r| r.count).sum())
}

/// DB 미스 시: 6자리 이하 전부 숫자면 국내, 아니면 해외(NASDAQ 가정).
fn fallback(input: &str) -> SymbolInfo {
    let is_domestic = !input.is_empty()
        && input.len() <= 6
        && input.chars().all(|c| c.is_ascii_digit());
    if is_domestic {
        SymbolInfo {
            code: input.to_string(),
            name: input.to_string(),
            name_en: String::new(),
            kind: "domestic",
            market_label: "",
            excd: "",
            matched: false,
        }
    } else {
        SymbolInfo {
            code: input.to_uppercase(),
            name: input.to_uppercase(),
            name_en: String::new(),
            kind: "overseas",
            market_label: "",
            excd: "NAS",
            matched: false,
        }
    }
}
