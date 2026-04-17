//! 사용자 입력(코드 또는 이름)을 구체적 심볼로 해석.
//!
//! 전략:
//! 1. 6자리 숫자 → 국내 주식 코드로 간주, 먼저 code 검색
//! 2. 대문자/숫자 조합 1~8자 → 해외 티커로 간주, 먼저 code 검색
//! 3. 위에서 실패하거나 한글 포함 → FTS5 name 검색
//! 4. 결과 0 → 에러. 1 → 반환. 복수 → pick 지정되면 그것, 아니면 TTY에서 프롬프트.

use std::io::{self, BufRead, Write};

use anyhow::{anyhow, Result};

use crate::symbols::store::{Market, Store, Symbol};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveMode {
    /// 국내주식만 대상 (KOSPI/KOSDAQ)
    Domestic,
    /// 해외주식만 대상 (NASDAQ/NYSE/AMEX)
    Overseas,
    /// 선물옵션 마스터 (FoIdx/FoStk)
    FutureOption,
    /// 제한 없음
    Any,
}

#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    pub code: String,
    pub market: Market,
    pub name_kr: String,
    pub name_en: String,
}

impl From<Symbol> for ResolvedSymbol {
    fn from(s: Symbol) -> Self {
        ResolvedSymbol {
            code: s.code,
            market: s.market,
            name_kr: s.name_kr,
            name_en: s.name_en,
        }
    }
}

pub fn resolve(
    store: &Store,
    input: &str,
    mode: ResolveMode,
    pick: Option<usize>,
) -> Result<ResolvedSymbol> {
    let input = input.trim();
    if input.is_empty() {
        return Err(anyhow!("심볼 입력이 비어있음"));
    }

    let mut candidates = Vec::new();

    // 1. 코드 패턴이면 정확 일치 먼저 시도
    if looks_like_code(input) {
        for s in store.find_by_code(input)? {
            if mode_allows(mode, s.market) {
                candidates.push(s);
            }
        }
        // 대문자로 표준화한 티커도 시도 (해외 티커 대소문자 입력 대응)
        if candidates.is_empty() && input != input.to_uppercase() {
            for s in store.find_by_code(&input.to_uppercase())? {
                if mode_allows(mode, s.market) {
                    candidates.push(s);
                }
            }
        }
    }

    // 2. 코드로 못 찾았으면 이름 검색
    if candidates.is_empty() {
        for s in store.search(input, 20)? {
            if mode_allows(mode, s.market) {
                candidates.push(s);
            }
        }
    }

    match candidates.len() {
        0 => Err(anyhow!(
            "'{}' 일치하는 종목 없음. `kis symbols sync`로 마스터 갱신 필요할 수 있음.",
            input
        )),
        1 => Ok(candidates.into_iter().next().unwrap().into()),
        _ => {
            if let Some(idx) = pick {
                let cand = candidates
                    .into_iter()
                    .nth(idx.saturating_sub(1))
                    .ok_or_else(|| anyhow!("--pick 범위 초과"))?;
                return Ok(cand.into());
            }
            prompt_pick(&candidates).map(Into::into)
        }
    }
}

fn mode_allows(mode: ResolveMode, m: Market) -> bool {
    match mode {
        ResolveMode::Domestic => m.is_domestic(),
        ResolveMode::Overseas => m.is_overseas(),
        ResolveMode::FutureOption => m.is_futureoption(),
        ResolveMode::Any => true,
    }
}

fn looks_like_code(s: &str) -> bool {
    if s.is_empty() || s.len() > 10 {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}

fn prompt_pick(candidates: &[Symbol]) -> Result<Symbol> {
    if !is_tty() {
        return Err(anyhow!(
            "복수 매칭({})이나 TTY 아님. --pick N 으로 선택하세요.",
            candidates.len()
        ));
    }

    eprintln!("\n여러 종목이 일치합니다:");
    for (i, s) in candidates.iter().enumerate() {
        eprintln!(
            "  [{}] {:10} {:<7} {} / {}",
            i + 1,
            s.code,
            s.market.as_str(),
            s.name_kr,
            s.name_en
        );
    }
    eprint!("번호 선택 (1-{}, Enter=1): ", candidates.len());
    io::stderr().flush().ok();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let s = line.trim();
    let idx = if s.is_empty() {
        1usize
    } else {
        s.parse::<usize>()
            .map_err(|_| anyhow!("숫자를 입력하세요"))?
    };
    let cand = candidates
        .get(idx.saturating_sub(1))
        .ok_or_else(|| anyhow!("범위 초과"))?
        .clone();
    Ok(cand)
}

fn is_tty() -> bool {
    // std에 isatty가 없어 libc FFI로 확인.
    unsafe extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(0) == 1 }
}
