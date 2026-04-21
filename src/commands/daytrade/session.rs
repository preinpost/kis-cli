//! 시장별 세션 엔진 — 장 시간 판정, 다음 개장 대기, 마감 임박 판정.
//!
//! - **KRX**: 09:00–15:30 Asia/Seoul (평일)
//! - **미장**: 09:30–16:00 America/New_York → KST 변환 (DST 자동)
//!
//! Phase 1은 `is_in_session` / `time_until_open` / `next_bar_boundary` 만 사용.
//! 마감 10분 전 강제청산(`should_force_exit`)은 Phase 2(paper/run)에서 사용.

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone, Timelike, Weekday};
use chrono_tz::America::New_York;
use chrono_tz::Asia::Seoul;
use chrono_tz::Tz;

use super::period::Period;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Market {
    Krx,
    Usa,
}

impl Market {
    pub fn label(self) -> &'static str {
        match self {
            Market::Krx => "KRX",
            Market::Usa => "USA",
        }
    }
}

pub fn now_kst() -> DateTime<Tz> {
    chrono::Utc::now().with_timezone(&Seoul)
}

pub fn is_in_session(market: Market, now: DateTime<Tz>) -> bool {
    match market {
        Market::Krx => {
            if is_weekend(now.weekday()) {
                return false;
            }
            let t = now.time();
            t >= NaiveTime::from_hms_opt(9, 0, 0).unwrap()
                && t < NaiveTime::from_hms_opt(15, 30, 0).unwrap()
        }
        Market::Usa => {
            let ny = now.with_timezone(&New_York);
            if is_weekend(ny.weekday()) {
                return false;
            }
            let t = ny.time();
            t >= NaiveTime::from_hms_opt(9, 30, 0).unwrap()
                && t < NaiveTime::from_hms_opt(16, 0, 0).unwrap()
        }
    }
}

/// 현재 세션의 마감 시각 (KST). 세션 밖이면 "다음" 마감 시각.
pub fn session_close_kst(market: Market, now: DateTime<Tz>) -> DateTime<Tz> {
    match market {
        Market::Krx => {
            let d = now.date_naive();
            let candidate = close_krx(d);
            if candidate > now {
                candidate
            } else {
                close_krx(next_weekday(d))
            }
        }
        Market::Usa => {
            // 미장 마감 = 16:00 America/New_York 해당일. 현재 NY 시각 기준으로 판정.
            let ny = now.with_timezone(&New_York);
            let d = ny.date_naive();
            let candidate = close_usa(d);
            if candidate > now {
                candidate
            } else {
                close_usa(next_weekday(d))
            }
        }
    }
}

/// 장 마감 `before_min`분 전 이내면 true (현재 세션 중에 한해).
pub fn should_force_exit(market: Market, now: DateTime<Tz>, before_min: i64) -> bool {
    if !is_in_session(market, now) {
        return false;
    }
    let close = session_close_kst(market, now);
    let mins = (close - now).num_minutes();
    mins >= 0 && mins <= before_min
}

/// 다음 개장까지 남은 시간. 세션 중이면 `Duration::zero()`.
pub fn time_until_open(market: Market, now: DateTime<Tz>) -> Duration {
    if is_in_session(market, now) {
        return Duration::zero();
    }
    for offset in 0..8 {
        let cand_date = match market {
            Market::Krx => now.date_naive() + Duration::days(offset),
            Market::Usa => now.with_timezone(&New_York).date_naive() + Duration::days(offset),
        };
        if is_weekend(cand_date.weekday()) {
            continue;
        }
        let open = match market {
            Market::Krx => open_krx(cand_date),
            Market::Usa => open_usa(cand_date),
        };
        if open > now {
            return open - now;
        }
    }
    Duration::hours(24)
}

/// 다음 봉 경계 시각 (KST). 5분봉이면 :00, :05, :10 ... 에 맞춘다.
/// 데이터 체감 지연을 고려해 경계 + `slack_sec` 초를 더한 시각을 반환.
pub fn next_bar_boundary_kst(period: Period, now: DateTime<Tz>, slack_sec: i64) -> DateTime<Tz> {
    let step = period.minutes() as i64;
    let m = now.minute() as i64;
    let next_m = ((m / step) + 1) * step;
    let base = now
        .with_second(0)
        .and_then(|d| d.with_nanosecond(0))
        .unwrap_or(now);
    base + Duration::minutes(next_m - m) + Duration::seconds(slack_sec)
}

// ─── 내부 헬퍼 ───────────────────────────────────────────────────────────

fn is_weekend(w: Weekday) -> bool {
    matches!(w, Weekday::Sat | Weekday::Sun)
}

fn next_weekday(d: NaiveDate) -> NaiveDate {
    let mut d = d + Duration::days(1);
    while is_weekend(d.weekday()) {
        d = d + Duration::days(1);
    }
    d
}

fn open_krx(d: NaiveDate) -> DateTime<Tz> {
    Seoul
        .from_local_datetime(&d.and_hms_opt(9, 0, 0).unwrap())
        .single()
        .unwrap()
}

fn close_krx(d: NaiveDate) -> DateTime<Tz> {
    Seoul
        .from_local_datetime(&d.and_hms_opt(15, 30, 0).unwrap())
        .single()
        .unwrap()
}

fn open_usa(d: NaiveDate) -> DateTime<Tz> {
    New_York
        .from_local_datetime(&d.and_hms_opt(9, 30, 0).unwrap())
        .single()
        .unwrap()
        .with_timezone(&Seoul)
}

fn close_usa(d: NaiveDate) -> DateTime<Tz> {
    New_York
        .from_local_datetime(&d.and_hms_opt(16, 0, 0).unwrap())
        .single()
        .unwrap()
        .with_timezone(&Seoul)
}
