//! KIS 종목 마스터 파일 파서.
//!
//! 포맷은 시장별로 다름:
//! - **KOSPI/KOSDAQ (`.mst`)**: EUC-KR 고정/가변 폭. `[9 단축코드][12 표준코드][가변 한글명][228바이트 트레일러]`.
//!   트레일러 일부 오프셋에 영문명이 들어가는 경우가 있으나 버전에 따라 달라 여기서는 한글명만 추출.
//! - **NAS/NYS/AMS (`.cod`)**: EUC-KR 고정 폭. 스펙을 알 수 없으므로 tab/multi-space split으로 관용 파싱.
//!
//! 파싱이 손상된 라인은 조용히 스킵 (KIS가 헤더·빈 줄을 섞기도 함).

use anyhow::Result;
use encoding_rs::EUC_KR;

use crate::symbols::store::{Market, Symbol};

/// KOSPI/KOSDAQ `.mst` 바이너리 파싱.
///
/// 라인 단위. 각 라인 끝 228바이트는 트레일러, 그 앞 9바이트는 단축코드,
/// 다음 12바이트는 표준코드, 나머지는 한글명.
pub fn parse_domestic_mst(bytes: &[u8], market: Market) -> Result<Vec<Symbol>> {
    let (cow, _, _) = EUC_KR.decode(bytes);
    // EUC-KR 디코딩 후 `.chars().count()`는 실제 길이와 다를 수 있어, 다시 바이트로 다룬다.
    // 단, KIS의 mst 파일은 실제로 EUC-KR "바이트 길이" 기준 고정/가변.
    // 그래서 EUC-KR 디코딩 결과의 char 길이가 아니라 **라인 원문 바이트 배열**을 써야 함.

    let mut out = Vec::new();
    for line_bytes in bytes.split(|b| *b == b'\n') {
        // CR 제거
        let line_bytes = strip_cr(line_bytes);
        if line_bytes.len() < 9 + 12 + 228 {
            continue;
        }
        let code_bytes = &line_bytes[0..9];
        let name_bytes = &line_bytes[21..(line_bytes.len() - 228)];

        let code = bytes_to_utf8_trim(code_bytes);
        // 단축코드는 공백 trim 후 6자리 유효값만 채택 (KIS는 앞/뒤 공백으로 패딩).
        let code = code.trim().to_string();
        if code.is_empty() {
            continue;
        }

        let name_kr = bytes_to_utf8_trim(name_bytes).trim().to_string();
        if name_kr.is_empty() {
            continue;
        }

        out.push(Symbol {
            code,
            market,
            name_kr,
            name_en: String::new(),
        });
    }
    // `cow`는 단지 디코딩 확인용. 본 파싱은 바이트 기반.
    drop(cow);
    Ok(out)
}

/// 선물옵션 `fo_idx_code.mst` / `fo_stk_code.mst` 파싱.
///
/// 고정폭 포맷 (121자). 관찰된 오프셋:
/// - `[0..7]` 단축코드 (7 chars)
/// - `[10..22]` 표준코드 (ISIN 12 chars)
/// - `[22]` 유형 (F=futures, C=call, P=put)
/// - `[24..30]` 만기월 (YYYYMM)
/// - 라인 끝부분에 영문 종목명 (EUC-KR 디코딩)
///
/// 종목명은 마지막 대문자 토큰 연속(공백 스킵).
pub fn parse_fo_mst(bytes: &[u8], market: Market) -> Result<Vec<Symbol>> {
    let (text, _, _) = EUC_KR.decode(bytes);
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        if line.len() < 30 {
            continue;
        }
        let chars: Vec<char> = line.chars().collect();
        let code: String = chars.iter().take(9).collect::<String>().trim().to_string();
        if code.is_empty() || !is_ticker_like(&code) {
            continue;
        }
        if !seen.insert(code.clone()) {
            continue;
        }
        let kind: String = chars.iter().nth(22).map(|c| c.to_string()).unwrap_or_default();
        let expiry: String = chars.iter().skip(24).take(6).collect::<String>().trim().to_string();

        // 기초자산명: 라인 끝에서부터 연속된 ASCII 영문/숫자 토큰을 name_en으로 추출.
        let tail: String = chars.iter().skip(80).collect::<String>();
        let name_en: String = tail
            .split_whitespace()
            .find(|t| t.chars().any(|c| c.is_ascii_alphabetic()))
            .unwrap_or("")
            .to_string();

        // name_kr은 영문명과 만기월을 합성해 검색 친화적으로.
        let name_kr = if !name_en.is_empty() && !expiry.is_empty() {
            format!("{} {} {}", name_en, expiry, kind_label(&kind))
        } else {
            String::new()
        };

        if name_en.is_empty() {
            continue;
        }

        out.push(Symbol {
            code,
            market,
            name_kr,
            name_en,
        });
    }
    Ok(out)
}

fn kind_label(k: &str) -> &'static str {
    match k {
        "F" => "선물",
        "C" => "콜",
        "P" => "풋",
        _ => "",
    }
}

/// 해외 `.cod` 파싱. 탭 구분 포맷. 실제 관찰된 필드 순서:
/// `[0] 국가코드 / [1] 거래소번호 / [2] 거래소구분(NAS/NYS/AMS) / [3] 거래소명(한글) /
///  [4] 종목코드 / [5] 실시간심볼 / [6] 한글종목명 / [7] 영문종목명 / ...`
///
/// EUC-KR로 디코딩. 빈 줄/짧은 줄/헤더는 스킵. 중복 코드는 첫 번째만 채택.
pub fn parse_overseas_cod(bytes: &[u8], market: Market) -> Result<Vec<Symbol>> {
    let (text, _, _) = EUC_KR.decode(bytes);
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.split('\t').collect();
        if tokens.len() < 8 {
            continue;
        }
        let code = tokens[4].trim().to_string();
        if code.is_empty() || !is_ticker_like(&code) {
            continue;
        }
        if !seen.insert(code.clone()) {
            continue;
        }
        let name_kr = tokens[6].trim().to_string();
        let name_en = tokens[7].trim().to_string();
        if name_kr.is_empty() && name_en.is_empty() {
            continue;
        }
        out.push(Symbol {
            code,
            market,
            name_kr,
            name_en,
        });
    }
    Ok(out)
}

fn strip_cr(s: &[u8]) -> &[u8] {
    if let Some(&b) = s.last() {
        if b == b'\r' {
            return &s[..s.len() - 1];
        }
    }
    s
}

fn bytes_to_utf8_trim(bytes: &[u8]) -> String {
    let (cow, _, _) = EUC_KR.decode(bytes);
    cow.into_owned()
}

fn is_ticker_like(t: &str) -> bool {
    if t.is_empty() || t.len() > 10 {
        return false;
    }
    t.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        && t.chars().any(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
}

fn split_kr_en(tokens: &[&str]) -> (String, String) {
    // 연속 한글 토큰을 name_kr로, 연속 영문 토큰을 name_en으로.
    let mut kr: Vec<&str> = Vec::new();
    let mut en: Vec<&str> = Vec::new();
    for t in tokens {
        if t.chars().any(|c| (c as u32) >= 0xAC00 && (c as u32) <= 0xD7A3) {
            kr.push(t);
        } else if t.chars().any(|c| c.is_ascii_alphabetic()) {
            en.push(t);
        }
    }
    (kr.join(" "), en.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticker_like_basic() {
        assert!(is_ticker_like("TSLA"));
        assert!(is_ticker_like("BRK.B"));
        assert!(is_ticker_like("005930"));
        assert!(!is_ticker_like("Tesla"));
        assert!(!is_ticker_like(""));
        assert!(!is_ticker_like("verylongticker1"));
    }

    #[test]
    fn split_names() {
        let (kr, en) = split_kr_en(&["삼성전자", "Samsung", "Electronics"]);
        assert_eq!(kr, "삼성전자");
        assert_eq!(en, "Samsung Electronics");
    }
}
