//! CLI 커맨드에서 공유하는 헬퍼.

use anyhow::Result;

use crate::config;
use crate::symbols::{resolve, ResolveMode, ResolvedSymbol, Store};

/// symbols DB 열기. 없으면 빈 DB (스키마만) — resolve에서 "sync 필요" 에러 반환.
pub fn open_store() -> Result<Store> {
    Store::open(&config::symbols_db_path()?)
}

/// 사용자가 입력한 심볼 문자열을 해석. 실패 시 원본을 그대로 쓸지 여부를 에러로 표현.
pub fn resolve_symbol(
    input: &str,
    mode: ResolveMode,
    pick: Option<usize>,
) -> Result<ResolvedSymbol> {
    let store = open_store()?;
    resolve(&store, input, mode, pick)
}

pub fn format_number(s: &str) -> String {
    let s = s.trim();
    let neg = s.starts_with('-');
    let body = s.trim_start_matches('-');
    let parts: Vec<&str> = body.split('.').collect();
    let integer = parts[0];
    let formatted: String = integer
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && i % 3 == 0 {
                acc.push(',');
            }
            acc.push(c);
            acc
        })
        .chars()
        .rev()
        .collect();
    let out = if parts.len() > 1 {
        format!("{}.{}", formatted, parts[1])
    } else {
        formatted
    };
    if neg { format!("-{}", out) } else { out }
}
