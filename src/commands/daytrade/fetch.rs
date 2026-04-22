//! 분봉 시계열 fetch — 국내/해외 각 API 래핑 + 필요 시 집계.
//!
//! - **국내**: `inquire_time_dailychartprice` 사용 (1분봉 고정, 120건/호출, 최대 1년).
//!   N분봉 요청 시 1분봉을 받아 클라이언트에서 집계.
//!   모의투자 미지원 — 실전 계정에서만 동작.
//! - **해외**: `overseas_stock::quotations::inquire_time_itemchartprice` 사용.
//!   `NMIN` 파라미터로 서버측 집계. 120건/호출. `KEYB` 페이징.
//!   모의투자 미지원.
//!
//! 반환 `Series.dates` 는 분봉에서 `"YYYYMMDDHHmm"` 12자리. 전략 계산은 인덱스 기반이라 포맷 비의존.

use anyhow::{anyhow, bail, Result};
use chrono::{Duration as ChronoDuration, NaiveDateTime};

use crate::api::domestic_stock::quotations::inquire_time_dailychartprice as dome_min;
use crate::api::overseas_stock::quotations::inquire_time_itemchartprice as usa_min;
use crate::client::KisClient;
use crate::commands::analyze::Series;
use crate::symbols::Market as SymMarket;

use super::period::Period;

const TARGET_BARS: usize = 200;
const DOME_PER_PAGE: usize = 120;
const USA_PER_PAGE: usize = 120;

/// 국내 분봉 시계열. 1분봉 페이징 → 클라이언트 집계.
pub async fn fetch_domestic(client: &KisClient, code: &str, period: Period) -> Result<Series> {
    if client.is_mock() {
        bail!("국내 분봉(일별) API는 모의투자 미지원 — 실전 계정으로 전환 필요");
    }
    let need_1m = TARGET_BARS * period.aggregate_step_min() as usize;
    let pages = need_1m.div_ceil(DOME_PER_PAGE) + 1;

    let mut all_1m: Vec<dome_min::Bar> = Vec::new();
    let now = chrono::Local::now();
    let mut cursor_date = now.format("%Y%m%d").to_string();
    let mut cursor_hour = now.format("%H%M%S").to_string();

    for _ in 0..pages {
        let req = dome_min::Request {
            fid_cond_mrkt_div_code: "J".into(),
            fid_input_iscd: code.into(),
            fid_input_hour_1: cursor_hour.clone(),
            fid_input_date_1: cursor_date.clone(),
            fid_pw_data_incu_yn: "Y".into(),
            fid_fake_tick_incu_yn: Some("".into()),
        };
        let resp = dome_min::call(client, &req).await?;
        if resp.bars.is_empty() {
            break;
        }
        let oldest = resp
            .bars
            .last()
            .ok_or_else(|| anyhow!("분봉 응답 비어있음"))?
            .clone();
        all_1m.extend(resp.bars);
        if all_1m.len() >= need_1m {
            break;
        }
        // 다음 페이지: 방금 받은 가장 오래된 봉 직전 1분으로 이동
        let next_dt = parse_dome_ts(&oldest.stck_bsop_date, &oldest.stck_cntg_hour)
            .ok_or_else(|| anyhow!("분봉 타임스탬프 파싱 실패"))?
            - ChronoDuration::minutes(1);
        cursor_date = next_dt.format("%Y%m%d").to_string();
        cursor_hour = next_dt.format("%H%M%S").to_string();
    }

    // 중복 제거 + 시간순 정렬
    dedup_sort_dome(&mut all_1m);
    let series_1m = to_series_dome(all_1m);
    Ok(aggregate(series_1m, period))
}

/// 해외 분봉 시계열. 서버측 NMIN 집계 활용.
pub async fn fetch_overseas(
    client: &KisClient,
    code: &str,
    market: SymMarket,
    period: Period,
) -> Result<Series> {
    if client.is_mock() {
        bail!("해외 분봉 API는 모의투자 미지원 — 실전 계정으로 전환 필요");
    }
    let excd = match market {
        SymMarket::Nasdaq => "NAS",
        SymMarket::Nyse => "NYS",
        SymMarket::Amex => "AMS",
        _ => bail!("해외 시장 코드 미지원: {:?}", market),
    };
    let pages = TARGET_BARS.div_ceil(USA_PER_PAGE) + 1;

    let mut bars: Vec<usa_min::Bar> = Vec::new();
    let mut keyb = String::new();
    let mut next_flag = String::new();

    for _ in 0..pages {
        let req = usa_min::Request {
            auth: "".into(),
            excd: excd.into(),
            symb: code.into(),
            nmin: period.api_nmin().to_string(),
            pinc: "1".into(),
            next: next_flag.clone(),
            nrec: USA_PER_PAGE.to_string(),
            fill: "".into(),
            keyb: keyb.clone(),
        };
        let resp = usa_min::call(client, &req).await?;
        if resp.bars.is_empty() {
            break;
        }
        let oldest = resp
            .bars
            .last()
            .ok_or_else(|| anyhow!("해외 분봉 응답 비어있음"))?
            .clone();
        bars.extend(resp.bars);
        if bars.len() >= TARGET_BARS {
            break;
        }
        // 다음 페이지: KEYB = 가장 오래된 봉 시각에서 `period`분 전
        let next_dt = parse_usa_ts(&oldest.kymd, &oldest.khms)
            .ok_or_else(|| anyhow!("해외 분봉 타임스탬프 파싱 실패"))?
            - ChronoDuration::minutes(period.api_nmin() as i64);
        keyb = next_dt.format("%Y%m%d%H%M%S").to_string();
        next_flag = "1".into();
    }

    dedup_sort_usa(&mut bars);
    Ok(to_series_usa(bars))
}

// ─── 국내 ───────────────────────────────────────────────────────────────

fn parse_dome_ts(date: &str, hms: &str) -> Option<NaiveDateTime> {
    let combined = format!("{}{}", date.trim(), pad_hms(hms));
    NaiveDateTime::parse_from_str(&combined, "%Y%m%d%H%M%S").ok()
}

fn pad_hms(hms: &str) -> String {
    let t = hms.trim();
    if t.len() == 6 { t.to_string() } else { format!("{:0>6}", t) }
}

fn dedup_sort_dome(bars: &mut Vec<dome_min::Bar>) {
    bars.sort_by(|a, b| {
        let ka = (a.stck_bsop_date.clone(), pad_hms(&a.stck_cntg_hour));
        let kb = (b.stck_bsop_date.clone(), pad_hms(&b.stck_cntg_hour));
        ka.cmp(&kb)
    });
    bars.dedup_by(|a, b| {
        a.stck_bsop_date == b.stck_bsop_date
            && pad_hms(&a.stck_cntg_hour) == pad_hms(&b.stck_cntg_hour)
    });
}

fn to_series_dome(bars: Vec<dome_min::Bar>) -> Series {
    let mut s = empty_series();
    for b in bars {
        let close = parse_f(&b.stck_prpr);
        if close.is_nan() {
            continue;
        }
        let hms = pad_hms(&b.stck_cntg_hour);
        let ts = format!("{}{}", b.stck_bsop_date, &hms[..hms.len().min(4)]); // YYYYMMDD + HHMM
        s.dates.push(ts);
        s.open.push(parse_f(&b.stck_oprc));
        s.high.push(parse_f(&b.stck_hgpr));
        s.low.push(parse_f(&b.stck_lwpr));
        s.closes.push(close);
        s.volume.push(parse_f(&b.cntg_vol));
    }
    s
}

/// 1분봉 Series → N분봉으로 집계. 첫 봉 시각을 N 경계에 맞추지 않고, 연속 N개씩 묶음.
/// 공백 구간(점심휴장/장외)은 date/hour prefix가 다르면 끊어서 집계.
fn aggregate(src: Series, period: Period) -> Series {
    let step = period.aggregate_step_min() as usize;
    if step <= 1 {
        return src;
    }
    let mut out = empty_series();
    let n = src.dates.len();
    let mut i = 0;
    while i < n {
        let end = (i + step).min(n);
        let o = src.open[i];
        let c = src.closes[end - 1];
        let h = src.high[i..end].iter().cloned().fold(f64::MIN, f64::max);
        let l = src.low[i..end].iter().cloned().fold(f64::MAX, f64::min);
        let v: f64 = src.volume[i..end].iter().sum();
        out.dates.push(src.dates[end - 1].clone()); // 봉 마감 시각
        out.open.push(o);
        out.high.push(h);
        out.low.push(l);
        out.closes.push(c);
        out.volume.push(v);
        i += step;
    }
    out
}

// ─── 해외 ───────────────────────────────────────────────────────────────

fn parse_usa_ts(kymd: &str, khms: &str) -> Option<NaiveDateTime> {
    let combined = format!("{}{}", kymd.trim(), pad_hms(khms));
    NaiveDateTime::parse_from_str(&combined, "%Y%m%d%H%M%S").ok()
}

fn dedup_sort_usa(bars: &mut Vec<usa_min::Bar>) {
    bars.sort_by(|a, b| {
        let ka = (a.kymd.clone(), pad_hms(&a.khms));
        let kb = (b.kymd.clone(), pad_hms(&b.khms));
        ka.cmp(&kb)
    });
    bars.dedup_by(|a, b| a.kymd == b.kymd && pad_hms(&a.khms) == pad_hms(&b.khms));
}

fn to_series_usa(bars: Vec<usa_min::Bar>) -> Series {
    let mut s = empty_series();
    for b in bars {
        let close = parse_f(&b.last);
        if close.is_nan() {
            continue;
        }
        let hms = pad_hms(&b.khms);
        let ts = format!("{}{}", b.kymd, &hms[..hms.len().min(4)]);
        s.dates.push(ts);
        s.open.push(parse_f(&b.open));
        s.high.push(parse_f(&b.high));
        s.low.push(parse_f(&b.low));
        s.closes.push(close);
        s.volume.push(parse_f(&b.evol));
    }
    s
}

// ─── 공통 ───────────────────────────────────────────────────────────────

fn empty_series() -> Series {
    Series {
        dates: vec![],
        open: vec![],
        high: vec![],
        low: vec![],
        closes: vec![],
        volume: vec![],
    }
}

fn parse_f(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(f64::NAN)
}
