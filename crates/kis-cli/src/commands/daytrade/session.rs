//! 시장별 세션 엔진 — 장 시간 판정, 다음 개장 대기, 마감 임박 판정.
//!
//! - **KRX**: 09:00–15:30 Asia/Seoul (평일, 한국 공휴일 제외 — `chk-holiday` API)
//! - **미장**: 09:30–16:00 America/New_York → KST 변환 (DST 자동, 평일만)
//!
//! Phase 1은 `is_in_session` / `time_until_open` / `next_bar_boundary` 만 사용.
//! 마감 10분 전 강제청산(`should_force_exit`)은 Phase 2(paper/run)에서 사용.
//!
//! 공휴일 인식이 필요한 호출자는 `is_in_session_async` / `time_until_open_async` 를 사용.
//! 이 두 함수는 `HolidayCache`를 통해 KIS `chk-holiday`를 1일 1회만 호출 (KIS 권고).

use std::collections::HashMap;

use chrono::{DateTime, Datelike, Duration, NaiveDate, NaiveTime, TimeZone, Timelike, Weekday};
use chrono_tz::America::New_York;
use chrono_tz::Asia::Seoul;
use chrono_tz::Tz;
use tokio::sync::Mutex;
use tracing::warn;

use crate::api::domestic_stock::sector::chk_holiday;
use crate::client::KisClient;

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

/// 공휴일까지 반영한 세션 판정. KRX는 `chk-holiday` API 결과를 캐시 통해 확인,
/// 미장은 평일만 판정 (해외 공휴일 API는 결제일 기반이라 직접 판정에 부적합).
pub async fn is_in_session_async(
    market: Market,
    now: DateTime<Tz>,
    client: &KisClient,
    cache: &HolidayCache,
) -> bool {
    if !is_in_session(market, now) {
        return false;
    }
    match market {
        Market::Krx => cache.is_krx_open(client, now.date_naive()).await,
        Market::Usa => true,
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

/// 공휴일 캐시를 통해 다음 개장까지 남은 시간 산출. 한국 공휴일은 건너뜀.
pub async fn time_until_open_async(
    market: Market,
    now: DateTime<Tz>,
    client: &KisClient,
    cache: &HolidayCache,
) -> Duration {
    if is_in_session_async(market, now, client, cache).await {
        return Duration::zero();
    }
    for offset in 0..14 {
        let cand_date = match market {
            Market::Krx => now.date_naive() + Duration::days(offset),
            Market::Usa => now.with_timezone(&New_York).date_naive() + Duration::days(offset),
        };
        if is_weekend(cand_date.weekday()) {
            continue;
        }
        if matches!(market, Market::Krx) && !cache.is_krx_open(client, cand_date).await {
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
/// 30초 주기면 :00, :30, 1:00 ... 초 단위 경계.
/// 데이터 체감 지연을 고려해 경계 + `slack_sec` 초를 더한 시각을 반환.
pub fn next_bar_boundary_kst(period: Period, now: DateTime<Tz>, slack_sec: i64) -> DateTime<Tz> {
    let step_sec = period.seconds() as i64;
    let total = (now.minute() as i64) * 60 + now.second() as i64;
    let next_total = ((total / step_sec) + 1) * step_sec;
    let base_hour = now
        .with_minute(0)
        .and_then(|d| d.with_second(0))
        .and_then(|d| d.with_nanosecond(0))
        .unwrap_or(now);
    base_hour + Duration::seconds(next_total + slack_sec)
}

// ─── 공휴일 캐시 ─────────────────────────────────────────────────────────

/// KRX `chk-holiday` API 결과 캐시. KIS 가이드: 동일 일자에 대해 1일 1회만 호출 권장.
/// API 실패 시 평일은 개장(true)으로 폴백 — 호출자가 평일 시간 체크는 따로 한다.
#[derive(Default)]
pub struct HolidayCache {
    krx: Mutex<HashMap<NaiveDate, bool>>,
}

impl HolidayCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn is_krx_open(&self, client: &KisClient, date: NaiveDate) -> bool {
        if is_weekend(date.weekday()) {
            return false;
        }
        if let Some(&v) = self.krx.lock().await.get(&date) {
            return v;
        }
        let bass_dt = date.format("%Y%m%d").to_string();
        let req = chk_holiday::Request {
            bass_dt: bass_dt.clone(),
            ctx_area_nk: " ".repeat(20),
            ctx_area_fk: " ".repeat(20),
        };
        let is_open = match chk_holiday::call(client, &req).await {
            Ok(resp) => {
                if resp.bass_dt == bass_dt {
                    resp.opnd_yn == "Y"
                } else {
                    warn!(
                        "chk-holiday 응답 일자 불일치 ({} != {}) — 평일 기본값(개장)",
                        resp.bass_dt, bass_dt
                    );
                    true
                }
            }
            Err(e) => {
                warn!("KRX 개장일 조회 실패 ({}): {} — 평일 기본값(개장)", date, e);
                true
            }
        };
        self.krx.lock().await.insert(date, is_open);
        is_open
    }
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
