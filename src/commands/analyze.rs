//! `kis analyze <symbol>` — 캔들을 받아 MA/RSI/MACD/볼린저/일목 계산 후 출력.
//!
//! 두 모드: 사람-읽기 좋은 요약(기본) / `--json` 구조화 덤프.

use anyhow::{anyhow, Result};
use serde::Serialize;

use crate::analysis::{bollinger, ichimoku, macd, rsi, sma};
use crate::api::domestic_stock::quotations::inquire_daily_itemchartprice as dome_chart;
use crate::api::overseas_stock::quotations::inquire_daily_chartprice as usa_chart;
use crate::client::KisClient;
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::symbols::{Market, ResolveMode};

pub async fn run(
    client: &KisClient,
    symbol: &str,
    mode: ResolveMode,
    json: bool,
    save: Option<String>,
    pick: Option<usize>,
) -> Result<()> {
    let (_sym, _series, report, html) = prepare(client, symbol, mode, pick).await?;

    if let Some(path_str) = save.as_ref() {
        std::fs::write(path_str, &html)?;
        eprintln!("차트 저장: {}", path_str);
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_human(&report);
    }
    Ok(())
}

/// wry 뷰어용: 분석 결과 + HTML까지 만들어 반환. main 스레드에서 호출 후
/// `viewer::launch()` 로 창 띄우는 용도.
pub async fn prepare(
    client: &KisClient,
    symbol: &str,
    mode: ResolveMode,
    pick: Option<usize>,
) -> Result<(crate::symbols::ResolvedSymbol, Series, Report, String)> {
    let sym = resolve_symbol(symbol, mode, pick)?;
    let series = match mode {
        ResolveMode::Domestic => fetch_domestic(client, &sym.code).await?,
        ResolveMode::Overseas => fetch_overseas(client, &sym.code, sym.market).await?,
        _ => return Err(anyhow!("analyze는 국내/해외 주식만 지원")),
    };
    if series.closes.len() < 30 {
        return Err(anyhow!(
            "데이터 부족 ({}일) — 분석에 최소 30일 이상 필요",
            series.closes.len()
        ));
    }
    let report = compute(&sym.code, &sym.display_name(), &series);
    let html = render_html(&sym.code, &sym.display_name(), &series, &report);
    Ok((sym, series, report, html))
}

trait DisplayName {
    fn display_name(&self) -> String;
}

impl DisplayName for crate::symbols::ResolvedSymbol {
    fn display_name(&self) -> String {
        if !self.name_kr.is_empty() { self.name_kr.clone() }
        else if !self.name_en.is_empty() { self.name_en.clone() }
        else { self.code.clone() }
    }
}

pub struct Series {
    pub dates: Vec<String>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub closes: Vec<f64>,
    pub volume: Vec<f64>,
}

pub async fn fetch_domestic(client: &KisClient, code: &str) -> Result<Series> {
    fetch_domestic_with_period(client, code, 'D').await
}

pub async fn fetch_domestic_with_period(
    client: &KisClient,
    code: &str,
    period: char,
) -> Result<Series> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    fetch_domestic_chunk(client, code, period, &today).await
}

/// 주어진 `to_date`(YYYYMMDD)를 끝으로 최대 ~100봉 조회. 무한 스크롤용.
pub async fn fetch_domestic_chunk(
    client: &KisClient,
    code: &str,
    period: char,
    to_date: &str,
) -> Result<Series> {
    // calendar → trading day 여유: 일봉은 150일, 주봉 600, 월봉 3000
    let days_back = match period {
        'W' => 800,
        'M' => 3600,
        _ => 150,
    };
    let end = chrono::NaiveDate::parse_from_str(to_date, "%Y%m%d")
        .unwrap_or_else(|_| chrono::Local::now().date_naive());
    let from = (end - chrono::Duration::days(days_back)).format("%Y%m%d").to_string();
    let req = dome_chart::Request {
        fid_cond_mrkt_div_code: "J".into(),
        fid_input_iscd: code.into(),
        fid_input_date_1: from,
        fid_input_date_2: to_date.into(),
        fid_period_div_code: period.to_string(),
        fid_org_adj_prc: "0".into(),
    };
    let r = dome_chart::call(client, &req).await?;
    // 응답은 최신→오래된. 오래된→최신으로 뒤집는다.
    let mut candles = r.candles;
    candles.reverse();
    let mut s = Series { dates: vec![], open: vec![], high: vec![], low: vec![], closes: vec![], volume: vec![] };
    for c in candles {
        let close = parse_f(&c.stck_clpr);
        if close.is_nan() { continue; }
        s.dates.push(c.stck_bsop_date);
        s.open.push(parse_f(&c.stck_oprc));
        s.high.push(parse_f(&c.stck_hgpr));
        s.low.push(parse_f(&c.stck_lwpr));
        s.closes.push(close);
        s.volume.push(parse_f(&c.acml_vol));
    }
    Ok(s)
}

pub async fn fetch_overseas(client: &KisClient, code: &str, _market: Market) -> Result<Series> {
    fetch_overseas_with_period(client, code, 'D').await
}

pub async fn fetch_overseas_with_period(
    client: &KisClient,
    code: &str,
    period: char,
) -> Result<Series> {
    let today = chrono::Local::now().format("%Y%m%d").to_string();
    fetch_overseas_chunk(client, code, period, &today).await
}

pub async fn fetch_overseas_chunk(
    client: &KisClient,
    code: &str,
    period: char,
    to_date: &str,
) -> Result<Series> {
    let days_back = match period {
        'W' => 800,
        'M' => 3600,
        _ => 150,
    };
    let end = chrono::NaiveDate::parse_from_str(to_date, "%Y%m%d")
        .unwrap_or_else(|_| chrono::Local::now().date_naive());
    let from = (end - chrono::Duration::days(days_back)).format("%Y%m%d").to_string();
    let req = usa_chart::Request {
        fid_cond_mrkt_div_code: "N".into(),
        fid_input_iscd: code.into(),
        fid_input_date_1: from,
        fid_input_date_2: to_date.into(),
        fid_period_div_code: period.to_string(),
    };
    let r = usa_chart::call(client, &req).await?;
    let mut candles = r.candles;
    candles.reverse();
    let mut s = Series { dates: vec![], open: vec![], high: vec![], low: vec![], closes: vec![], volume: vec![] };
    for c in candles {
        let close = parse_f(&c.ovrs_nmix_prpr);
        if close.is_nan() { continue; }
        s.dates.push(c.stck_bsop_date);
        s.open.push(parse_f(&c.ovrs_nmix_oprc));
        s.high.push(parse_f(&c.ovrs_nmix_hgpr));
        s.low.push(parse_f(&c.ovrs_nmix_lwpr));
        s.closes.push(close);
        s.volume.push(parse_f(&c.acml_vol));
    }
    Ok(s)
}

fn parse_f(s: &str) -> f64 {
    s.trim().parse::<f64>().unwrap_or(f64::NAN)
}

#[derive(Debug, Serialize)]
pub struct Report {
    symbol: String,
    name: String,
    date: String,
    current_price: f64,
    bars: usize,
    ma: MaReport,
    rsi: RsiReport,
    macd: MacdReport,
    bollinger: BollingerReport,
    ichimoku: IchimokuReport,
    signals: Vec<String>,
    #[serde(skip)]
    chart: Option<String>,
}

#[derive(Debug, Serialize)]
struct MaReport {
    ma5: Option<f64>,
    ma20: Option<f64>,
    ma60: Option<f64>,
    ma120: Option<f64>,
    /// 정배열(5>20>60>120) / 역배열 / 혼조
    alignment: String,
}

#[derive(Debug, Serialize)]
struct RsiReport {
    period: usize,
    value: Option<f64>,
    state: String,
}

#[derive(Debug, Serialize)]
struct MacdReport {
    macd: Option<f64>,
    signal: Option<f64>,
    histogram: Option<f64>,
    cross: String,
}

#[derive(Debug, Serialize)]
struct BollingerReport {
    upper: Option<f64>,
    middle: Option<f64>,
    lower: Option<f64>,
    /// 현재가의 밴드 내 위치 (0=하단, 0.5=중단, 1=상단)
    percent_b: Option<f64>,
    bandwidth_pct: Option<f64>,
}

#[derive(Debug, Serialize)]
struct IchimokuReport {
    tenkan: Option<f64>,
    kijun: Option<f64>,
    senkou_a_now: Option<f64>,
    senkou_b_now: Option<f64>,
    cloud_color: String,
    price_vs_cloud: String,
    chikou_signal: String,
}

fn compute(code: &str, name: &str, s: &Series) -> Report {
    let n = s.closes.len();
    let last = n - 1;
    let price = s.closes[last];

    // MA
    let ma5 = sma(&s.closes, 5);
    let ma20 = sma(&s.closes, 20);
    let ma60 = sma(&s.closes, 60);
    let ma120 = sma(&s.closes, 120);
    let v5 = opt(ma5[last]);
    let v20 = opt(ma20[last]);
    let v60 = opt(ma60[last]);
    let v120 = opt(ma120[last]);
    let alignment = ma_alignment(&[v5, v20, v60, v120]);

    // RSI
    let rsi_series = rsi(&s.closes, 14);
    let rsi_val = opt(rsi_series[last]);
    let rsi_state = match rsi_val {
        Some(v) if v >= 70.0 => "과매수",
        Some(v) if v <= 30.0 => "과매도",
        Some(v) if v >= 60.0 => "상승 우세",
        Some(v) if v <= 40.0 => "하락 우세",
        Some(_) => "중립",
        None => "데이터 부족",
    }
    .to_string();

    // MACD
    let m = macd(&s.closes, 12, 26, 9);
    let macd_v = opt(m.macd[last]);
    let sig_v = opt(m.signal[last]);
    let hist_v = opt(m.histogram[last]);
    let cross = macd_cross(&m);

    // Bollinger
    let b = bollinger(&s.closes, 20, 2.0);
    let up = opt(b.upper[last]);
    let md = opt(b.middle[last]);
    let lo = opt(b.lower[last]);
    let percent_b = match (up, lo) {
        (Some(u), Some(l)) if u > l => Some((price - l) / (u - l)),
        _ => None,
    };
    let bandwidth_pct = match (up, md, lo) {
        (Some(u), Some(m), Some(l)) if m > 0.0 => Some((u - l) / m * 100.0),
        _ => None,
    };

    // Ichimoku
    let ic = ichimoku(&s.high, &s.low, &s.closes);
    let tenkan = opt(ic.tenkan[last]);
    let kijun = opt(ic.kijun[last]);
    // 현재 시점의 구름 = senkou_a[last], senkou_b[last] (senkou 배열은 n+26 길이, last 인덱스가 오늘 구름)
    let senkou_a_now = opt(ic.senkou_a[last]);
    let senkou_b_now = opt(ic.senkou_b[last]);
    let cloud_color = match (senkou_a_now, senkou_b_now) {
        (Some(a), Some(b)) if a > b => "양운",
        (Some(a), Some(b)) if a < b => "음운",
        (Some(_), Some(_)) => "중립",
        _ => "데이터 부족",
    }
    .to_string();
    let price_vs_cloud = match (senkou_a_now, senkou_b_now) {
        (Some(a), Some(b)) => {
            let top = a.max(b);
            let bot = a.min(b);
            if price > top { "구름 위" }
            else if price < bot { "구름 아래" }
            else { "구름 안" }
        }
        _ => "데이터 부족",
    }
    .to_string();
    let chikou_signal = if n > 26 {
        let ref_close = s.closes[last - 26];
        if price > ref_close { "양전환" }
        else if price < ref_close { "음전환" }
        else { "동일" }
    } else {
        "데이터 부족"
    }
    .to_string();

    // 시그널 요약
    let mut signals = Vec::new();
    if let (Some(h), Some(ph)) = (opt(m.histogram[last]), opt_idx(&m.histogram, last.wrapping_sub(1))) {
        if ph < 0.0 && h >= 0.0 { signals.push("MACD 골든크로스".into()); }
        if ph > 0.0 && h <= 0.0 { signals.push("MACD 데드크로스".into()); }
    }
    if let (Some(a), Some(b)) = (v5, v20) {
        if a > b { signals.push("5일선 > 20일선 (단기 상승)".into()); }
        else if a < b { signals.push("5일선 < 20일선 (단기 하락)".into()); }
    }
    if let Some(v) = rsi_val {
        if v >= 70.0 { signals.push("RSI 과매수 영역".into()); }
        if v <= 30.0 { signals.push("RSI 과매도 영역".into()); }
    }
    if let Some(pb) = percent_b {
        if pb >= 1.0 { signals.push("볼린저 상단 돌파".into()); }
        if pb <= 0.0 { signals.push("볼린저 하단 이탈".into()); }
    }
    if price_vs_cloud == "구름 위" && cloud_color == "양운" {
        signals.push("일목 양운 위 — 강세".into());
    }
    if price_vs_cloud == "구름 아래" && cloud_color == "음운" {
        signals.push("일목 음운 아래 — 약세".into());
    }

    // 최근 60일 차트 (종가 + MA20)
    let window = 60usize.min(n);
    let start = n - window;
    let chart = render_chart(
        &s.closes[start..],
        &ma20[start..],
        &b.upper[start..],
        &b.lower[start..],
        60,
        12,
    );

    Report {
        symbol: code.into(),
        name: name.into(),
        date: s.dates[last].clone(),
        current_price: price,
        bars: n,
        ma: MaReport { ma5: v5, ma20: v20, ma60: v60, ma120: v120, alignment },
        rsi: RsiReport { period: 14, value: rsi_val, state: rsi_state },
        macd: MacdReport { macd: macd_v, signal: sig_v, histogram: hist_v, cross },
        bollinger: BollingerReport {
            upper: up, middle: md, lower: lo, percent_b, bandwidth_pct,
        },
        ichimoku: IchimokuReport {
            tenkan, kijun, senkou_a_now, senkou_b_now,
            cloud_color, price_vs_cloud, chikou_signal,
        },
        signals,
        chart: Some(chart),
    }
}

/// 간단 ASCII 차트: 종가 line (●), MA20 overlay (·), 볼린저 상/하단 (·).
/// height × width 그리드에 price를 row에 매핑, x축은 time.
fn render_chart(
    closes: &[f64],
    ma20: &[f64],
    bb_upper: &[f64],
    bb_lower: &[f64],
    max_cols: usize,
    height: usize,
) -> String {
    let n = closes.len();
    if n == 0 || height < 3 { return String::new(); }
    let cols = n.min(max_cols);
    // 최근 cols개만 추출 (입력이 이미 창인 경우에도 안전)
    let start = n.saturating_sub(cols);
    let close_slice: Vec<f64> = closes[start..].to_vec();
    let ma_slice: Vec<f64> = ma20[start..].iter().cloned().collect();
    let up_slice: Vec<f64> = bb_upper[start..].iter().cloned().collect();
    let lo_slice: Vec<f64> = bb_lower[start..].iter().cloned().collect();

    // Y축은 **종가 기준** — 밴드가 극단이면 그건 위/아래 경계로 클립 표시.
    let closes_only: Vec<f64> = close_slice.iter().cloned().filter(|v| !v.is_nan()).collect();
    if closes_only.is_empty() { return String::new(); }
    let cmax = closes_only.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let cmin = closes_only.iter().cloned().fold(f64::INFINITY, f64::min);
    let pad = (cmax - cmin).max(cmax * 0.01) * 0.05;
    let max = cmax + pad;
    let min = (cmin - pad).max(0.0);
    let range = (max - min).max(1e-9);

    let row_of = |v: f64| -> Option<usize> {
        if v.is_nan() { return None; }
        let norm = ((v - min) / range).clamp(0.0, 1.0);
        let r = ((1.0 - norm) * (height - 1) as f64).round() as usize;
        Some(r.min(height - 1))
    };

    let mut grid: Vec<Vec<char>> = vec![vec![' '; close_slice.len()]; height];

    // 볼린저 밴드 (점선)
    for (x, (u, l)) in up_slice.iter().zip(lo_slice.iter()).enumerate() {
        if let Some(r) = row_of(*u) { if grid[r][x] == ' ' { grid[r][x] = '·'; } }
        if let Some(r) = row_of(*l) { if grid[r][x] == ' ' { grid[r][x] = '·'; } }
    }
    // MA20 (대시)
    for (x, v) in ma_slice.iter().enumerate() {
        if let Some(r) = row_of(*v) { grid[r][x] = '-'; }
    }
    // 종가 (채운 원) — 가장 위 우선
    for (x, v) in close_slice.iter().enumerate() {
        if let Some(r) = row_of(*v) { grid[r][x] = '●'; }
    }

    // Y축 라벨 (max/mid/min), 오른쪽 테두리
    let label_max = fmt_short(max);
    let label_mid = fmt_short((max + min) / 2.0);
    let label_min = fmt_short(min);
    let label_w = label_max.chars().count()
        .max(label_mid.chars().count())
        .max(label_min.chars().count());

    let mut out = String::new();
    for (r, row) in grid.iter().enumerate() {
        let label = if r == 0 { label_max.clone() }
        else if r == height / 2 { label_mid.clone() }
        else if r == height - 1 { label_min.clone() }
        else { String::new() };
        out.push_str(&format!("{:>w$} │", label, w = label_w));
        for c in row { out.push(*c); }
        out.push('\n');
    }
    // X축
    out.push_str(&format!("{:>w$} └", "", w = label_w));
    for _ in 0..close_slice.len() { out.push('─'); }
    out.push('\n');
    out.push_str(&format!("{:>w$}  {:<}일전{:>pad$}오늘",
        "", close_slice.len() - 1, "",
        w = label_w,
        pad = close_slice.len().saturating_sub(8)));

    out.push_str("\n범례: ● 종가  - MA20  · 볼린저");
    out
}

/// TradingView Lightweight Charts로 HTML 생성. 3개 패널 (가격/거래량/MACD).
fn render_html(symbol: &str, name: &str, s: &Series, r: &Report) -> String {
    let payload = series_to_js_payload(s);

    let signals_html = if r.signals.is_empty() {
        String::new()
    } else {
        format!(
            "<span class=\"signals\">{}</span>",
            r.signals.iter().map(|s| format!("<span class=\"sig\">{}</span>", s)).collect::<Vec<_>>().join(" ")
        )
    };

    format!(r#"<!DOCTYPE html>
<html lang="ko"><head>
<meta charset="utf-8">
<title>[{symbol}] {name} — kis-cli</title>
<script src="https://unpkg.com/lightweight-charts@4.2.0/dist/lightweight-charts.standalone.production.js"></script>
<style>
  * {{ box-sizing: border-box; }}
  body {{ margin: 0; font-family: -apple-system, BlinkMacSystemFont, system-ui, sans-serif;
         background: #131722; color: #d1d4dc; }}
  header {{ padding: 12px 20px; border-bottom: 1px solid #2a2e39; display: flex;
           align-items: center; gap: 16px; flex-wrap: wrap; }}
  .period-group {{ display: inline-flex; gap: 0; margin-left: auto; }}
  .period-btn {{ background: #1e222d; color: #888; border: 1px solid #2a2e39;
                padding: 4px 14px; cursor: pointer; font-size: 12px;
                transition: all 0.1s; }}
  .period-btn:first-child {{ border-radius: 3px 0 0 3px; }}
  .period-btn:last-child  {{ border-radius: 0 3px 3px 0; }}
  .period-btn:not(:first-child) {{ border-left: none; }}
  .period-btn.active {{ background: #2962ff; color: #fff; border-color: #2962ff; }}
  .period-btn:hover:not(.active) {{ background: #2a2e39; color: #d1d4dc; }}
  .status {{ font-size: 11px; color: #666; margin-left: 8px; }}
  .search-box {{ position: relative; margin-left: 12px; }}
  #search {{ background: #1e222d; border: 1px solid #2a2e39; color: #d1d4dc;
            padding: 5px 10px; border-radius: 3px; font-size: 12px; width: 220px;
            font-family: inherit; }}
  #search:focus {{ outline: none; border-color: #2962ff; }}
  #searchResults {{ position: absolute; top: calc(100% + 2px); right: 0;
                    background: #1e222d; border: 1px solid #2a2e39;
                    border-radius: 3px; max-height: 320px; overflow-y: auto;
                    display: none; z-index: 10; min-width: 320px; }}
  .sr {{ padding: 7px 12px; cursor: pointer; font-size: 12px;
        display: flex; gap: 10px; border-bottom: 1px solid #2a2e3966;
        align-items: center; }}
  .sr:last-child {{ border-bottom: none; }}
  .sr:hover, .sr.active {{ background: #2a2e39; }}
  .sr-code {{ color: #5d9cec; font-weight: 600; min-width: 68px; font-family: monospace; }}
  .sr-market {{ color: #888; min-width: 48px; font-size: 10px;
               background: #2a2e39; padding: 2px 6px; border-radius: 2px; }}
  .sr-name {{ color: #d1d4dc; flex: 1; overflow: hidden;
             text-overflow: ellipsis; white-space: nowrap; }}
  h1 {{ font-size: 15px; margin: 0; font-weight: 600; }}
  .muted {{ color: #787b86; font-size: 13px; }}
  .signals {{ font-size: 12px; }}
  .sig {{ background: #2962ff22; color: #5d9cec; padding: 2px 8px; border-radius: 3px;
         margin-right: 6px; border: 1px solid #2962ff44; }}
  #chart {{ width: 100vw; height: calc(100vh - 56px); }}
</style></head>
<body>
<header>
  <h1>[{symbol}] {name}</h1>
  <span class="muted">{date} · <span id="bars">{bars}</span>봉 · 현재가 {price} · RSI {rsi} · {align}</span>
  {signals_html}
  <div class="period-group">
    <button class="period-btn active" data-period="D" onclick="switchPeriod('D')">일</button>
    <button class="period-btn" data-period="W" onclick="switchPeriod('W')">주</button>
    <button class="period-btn" data-period="M" onclick="switchPeriod('M')">월</button>
  </div>
  <div class="search-box">
    <input id="search" placeholder="종목 검색 (예: TSLA, 삼성)" autocomplete="off">
    <div id="searchResults"></div>
  </div>
  <span class="status" id="status"></span>
</header>
<div id="chart"></div>
<script>
const data = {payload};

const chart = LightweightCharts.createChart(document.getElementById('chart'), {{
  layout: {{ background: {{ type: 'solid', color: '#131722' }}, textColor: '#d1d4dc' }},
  grid: {{ vertLines: {{ color: '#2a2e3933' }}, horzLines: {{ color: '#2a2e3933' }} }},
  crosshair: {{ mode: LightweightCharts.CrosshairMode.Normal }},
  timeScale: {{ borderColor: '#2a2e39', timeVisible: false }},
  rightPriceScale: {{ borderColor: '#2a2e39' }},
}});

// ━━━ 가격 패널 (상단 60%) ━━━
const candleSeries = chart.addCandlestickSeries({{
  upColor: '#26a69a', downColor: '#ef5350',
  borderUpColor: '#26a69a', borderDownColor: '#ef5350',
  wickUpColor: '#26a69a', wickDownColor: '#ef5350',
}});
candleSeries.setData(data.candles);
candleSeries.priceScale().applyOptions({{ scaleMargins: {{ top: 0.02, bottom: 0.4 }} }});

const ma5 = chart.addLineSeries({{ color: '#a678f0', lineWidth: 2, title: 'MA5',
  priceLineVisible: false, lastValueVisible: false }});
ma5.setData(data.ma5);

const ma20 = chart.addLineSeries({{ color: '#f4a62a', lineWidth: 2, title: 'MA20',
  priceLineVisible: false, lastValueVisible: false }});
ma20.setData(data.ma20);

const ma60 = chart.addLineSeries({{ color: '#4ec9b0', lineWidth: 2, title: 'MA60',
  priceLineVisible: false, lastValueVisible: false }});
ma60.setData(data.ma60);

const bbu = chart.addLineSeries({{ color: '#5090d0', lineWidth: 1, lineStyle: 2, title: 'BB Upper',
  priceLineVisible: false, lastValueVisible: false }});
bbu.setData(data.bbu);

const bbl = chart.addLineSeries({{ color: '#5090d0', lineWidth: 1, lineStyle: 2, title: 'BB Lower',
  priceLineVisible: false, lastValueVisible: false }});
bbl.setData(data.bbl);

// ━━━ 일목균형표 ━━━
const tenkan = chart.addLineSeries({{ color: '#ef5350', lineWidth: 1, title: '전환선',
  priceLineVisible: false, lastValueVisible: false }});
tenkan.setData(data.tenkan);

const kijun = chart.addLineSeries({{ color: '#2196f3', lineWidth: 1, title: '기준선',
  priceLineVisible: false, lastValueVisible: false }});
kijun.setData(data.kijun);

// 일목 선행A/B는 얇은 선으로만 그리고, 구름 fill은 아래의 Canvas 오버레이로 수동 렌더링.
const senkouA = chart.addLineSeries({{
  color: 'rgba(38, 166, 154, 0.7)', lineWidth: 1, title: '선행A',
  priceLineVisible: false, lastValueVisible: false, crosshairMarkerVisible: false,
}});
senkouA.setData(data.senkouA);

const senkouB = chart.addLineSeries({{
  color: 'rgba(239, 83, 80, 0.7)', lineWidth: 1, title: '선행B',
  priceLineVisible: false, lastValueVisible: false, crosshairMarkerVisible: false,
}});
senkouB.setData(data.senkouB);

// ━━━ 거래량 패널 (중단 20%) ━━━
const volSeries = chart.addHistogramSeries({{
  priceFormat: {{ type: 'volume' }},
  priceScaleId: 'volume',
  title: 'Vol',
}});
volSeries.setData(data.volume);
chart.priceScale('volume').applyOptions({{ scaleMargins: {{ top: 0.62, bottom: 0.2 }} }});

// ━━━ MACD 패널 (하단 20%) ━━━
const macdLine = chart.addLineSeries({{ color: '#2962ff', lineWidth: 1, title: 'MACD',
  priceScaleId: 'macd', priceLineVisible: false, lastValueVisible: false }});
macdLine.setData(data.macd);

const macdSignal = chart.addLineSeries({{ color: '#ff9800', lineWidth: 1, title: 'Signal',
  priceScaleId: 'macd', priceLineVisible: false, lastValueVisible: false }});
macdSignal.setData(data.signal);

const macdHist = chart.addHistogramSeries({{
  priceScaleId: 'macd', title: 'Hist',
  priceLineVisible: false,
}});
macdHist.setData(data.hist);
chart.priceScale('macd').applyOptions({{ scaleMargins: {{ top: 0.82, bottom: 0 }} }});

chart.timeScale().fitContent();
window.addEventListener('resize', () => chart.applyOptions({{
  width: window.innerWidth, height: window.innerHeight - 56,
}}));

// ─── 일목 구름 Canvas 오버레이 ───
const chartEl = document.getElementById('chart');
chartEl.style.position = 'relative';
const cloudCanvas = document.createElement('canvas');
cloudCanvas.style.position = 'absolute';
cloudCanvas.style.top = '0';
cloudCanvas.style.left = '0';
cloudCanvas.style.pointerEvents = 'none';
cloudCanvas.style.zIndex = '2';
chartEl.appendChild(cloudCanvas);

window._cloudData = {{ senkouA: data.senkouA || [], senkouB: data.senkouB || [] }};

function resizeCloudCanvas() {{
  const rect = chartEl.getBoundingClientRect();
  // 시간축·가격축 영역을 덮지 않도록 실제 drawing area만 커버
  const tsH = (chart.timeScale() && chart.timeScale().height) ? chart.timeScale().height() : 30;
  const psW = (chart.priceScale('right') && chart.priceScale('right').width) ? chart.priceScale('right').width() : 0;
  const w = Math.max(10, rect.width - psW);
  const h = Math.max(10, rect.height - tsH);
  const dpr = window.devicePixelRatio || 1;
  cloudCanvas.width = w * dpr;
  cloudCanvas.height = h * dpr;
  cloudCanvas.style.width = w + 'px';
  cloudCanvas.style.height = h + 'px';
  const ctx = cloudCanvas.getContext('2d');
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.scale(dpr, dpr);
}}

function drawCloud() {{
  const ctx = cloudCanvas.getContext('2d');
  const w = parseFloat(cloudCanvas.style.width) || chartEl.clientWidth;
  const h = parseFloat(cloudCanvas.style.height) || chartEl.clientHeight;
  ctx.clearRect(0, 0, w, h);

  const a = window._cloudData.senkouA || [];
  const b = window._cloudData.senkouB || [];
  if (a.length === 0 || b.length === 0) return;

  // time -> {{a,b}} map
  const byTime = new Map();
  for (const p of a) byTime.set(p.time, {{ a: p.value }});
  for (const p of b) {{
    const e = byTime.get(p.time) || {{}};
    e.b = p.value;
    byTime.set(p.time, e);
  }}
  const times = Array.from(byTime.keys()).sort();

  const tScale = chart.timeScale();
  const timeToX = (t) => tScale.timeToCoordinate(t);
  const priceToY = (v) => candleSeries.priceToCoordinate(v);

  // 같은 부호 연속 구간(segment)으로 쪼갬
  const segments = [];
  let seg = null;
  for (const t of times) {{
    const e = byTime.get(t);
    if (e.a == null || e.b == null) {{
      if (seg && seg.points.length > 1) segments.push(seg);
      seg = null;
      continue;
    }}
    const sign = e.a >= e.b ? 'pos' : 'neg';
    if (!seg || seg.sign !== sign) {{
      // 부호 전환 — 이전 세그먼트에 교차점 추가해 자연스럽게 닫음
      if (seg) {{
        seg.points.push({{ t, a: e.a, b: e.b }});
        if (seg.points.length > 1) segments.push(seg);
      }}
      seg = {{ sign, points: [] }};
      // 새 세그먼트 시작에도 교차점 공유
      if (segments.length > 0 || seg.points.length === 0) {{
        seg.points.push({{ t, a: e.a, b: e.b }});
      }}
    }} else {{
      seg.points.push({{ t, a: e.a, b: e.b }});
    }}
  }}
  if (seg && seg.points.length > 1) segments.push(seg);

  for (const s of segments) {{
    ctx.beginPath();
    let started = false;
    // 위쪽 경로 (pos: A, neg: B)
    for (let i = 0; i < s.points.length; i++) {{
      const p = s.points[i];
      const x = timeToX(p.t);
      const y = priceToY(s.sign === 'pos' ? p.a : p.b);
      if (x == null || y == null) continue;
      if (!started) {{ ctx.moveTo(x, y); started = true; }}
      else ctx.lineTo(x, y);
    }}
    // 아래쪽 경로 역방향 (pos: B, neg: A)
    for (let i = s.points.length - 1; i >= 0; i--) {{
      const p = s.points[i];
      const x = timeToX(p.t);
      const y = priceToY(s.sign === 'pos' ? p.b : p.a);
      if (x == null || y == null) continue;
      ctx.lineTo(x, y);
    }}
    ctx.closePath();
    ctx.fillStyle = s.sign === 'pos'
      ? 'rgba(38, 166, 154, 0.22)'
      : 'rgba(239, 83, 80, 0.22)';
    ctx.fill();
  }}
}}

let _cloudRaf = null;
function scheduleCloud() {{
  if (_cloudRaf) return;
  _cloudRaf = requestAnimationFrame(() => {{
    _cloudRaf = null;
    drawCloud();
  }});
}}

const _cloudResizeObs = new ResizeObserver(() => {{
  resizeCloudCanvas();
  scheduleCloud();
}});
_cloudResizeObs.observe(chartEl);

chart.timeScale().subscribeVisibleLogicalRangeChange(scheduleCloud);
chart.timeScale().subscribeVisibleTimeRangeChange(scheduleCloud);

resizeCloudCanvas();
setTimeout(scheduleCloud, 50);

// ─── 무한 스크롤: 왼쪽 끝 근접 시 과거 데이터 요청 ───
window._loading = false;
window._noMore = false;
chart.timeScale().subscribeVisibleLogicalRangeChange((range) => {{
  if (!range || window._loading || window._noMore) return;
  if (range.from < 10) {{
    window._loading = true;
    // 현재 시간 범위 저장 (prepend 후 복원용)
    window._savedRange = chart.timeScale().getVisibleRange();
    document.getElementById('status').textContent = '과거 데이터 로딩...';
    window.ipc && window.ipc.postMessage(JSON.stringify({{ type: 'loadMore' }}));
  }}
}});

// ─── IPC: 일/주/월 토글 ───
function switchPeriod(p) {{
  document.querySelectorAll('.period-btn').forEach(b =>
    b.classList.toggle('active', b.dataset.period === p));
  document.getElementById('status').textContent = '로딩...';
  // period 전환 시 무한 스크롤 상태 리셋
  window._noMore = false;
  window._savedRange = null;
  if (window.ipc && window.ipc.postMessage) {{
    window.ipc.postMessage(JSON.stringify({{ type: 'period', value: p }}));
  }} else {{
    console.warn('IPC 사용 불가 (브라우저 모드)');
  }}
}}

// Rust에서 새 데이터가 오면 이 함수가 호출됨
window.onChartData = function(p) {{
  const isPrepend = !!window._savedRange;
  candleSeries.setData(p.candles);
  volSeries.setData(p.volume);
  ma5.setData(p.ma5);
  ma20.setData(p.ma20);
  ma60.setData(p.ma60);
  bbu.setData(p.bbu);
  bbl.setData(p.bbl);
  tenkan.setData(p.tenkan);
  kijun.setData(p.kijun);
  senkouA.setData(p.senkouA);
  senkouB.setData(p.senkouB);
  macdLine.setData(p.macd);
  macdSignal.setData(p.signal);
  macdHist.setData(p.hist);
  if (isPrepend) {{
    try {{ chart.timeScale().setVisibleRange(window._savedRange); }}
    catch (e) {{ chart.timeScale().fitContent(); }}
    window._savedRange = null;
  }} else {{
    chart.timeScale().fitContent();
  }}
  document.getElementById('bars').textContent = p.candles.length;
  document.getElementById('status').textContent = '';
  window._loading = false;

  // 구름 데이터 갱신 + 재렌더
  window._cloudData = {{ senkouA: p.senkouA || [], senkouB: p.senkouB || [] }};
  if (typeof scheduleCloud === 'function') scheduleCloud();
}};

window.onChartError = function(msg) {{
  document.getElementById('status').textContent = '에러: ' + msg;
  window._loading = false;
  if (msg.indexOf('과거 데이터 없음') >= 0 || msg.indexOf('기존 데이터 없음') >= 0) {{
    window._noMore = true;
  }}
  window._savedRange = null;
}};

// ─── 심볼 검색 ───
let _searchTimer = null;
let _searchIdx = -1;
let _searchItems = [];
const _searchInput = document.getElementById('search');
const _searchResultsEl = document.getElementById('searchResults');

_searchInput.addEventListener('input', (e) => {{
  clearTimeout(_searchTimer);
  const q = e.target.value.trim();
  if (!q) {{ hideSearch(); return; }}
  _searchTimer = setTimeout(() => {{
    window.ipc && window.ipc.postMessage(JSON.stringify({{ type: 'search', query: q }}));
  }}, 180);
}});

_searchInput.addEventListener('keydown', (e) => {{
  if (_searchItems.length === 0 && e.key !== 'Escape') return;
  if (e.key === 'ArrowDown') {{ e.preventDefault(); _searchIdx = Math.min(_searchIdx + 1, _searchItems.length - 1); highlightSearch(); }}
  else if (e.key === 'ArrowUp') {{ e.preventDefault(); _searchIdx = Math.max(_searchIdx - 1, 0); highlightSearch(); }}
  else if (e.key === 'Enter') {{
    const it = _searchItems[_searchIdx] || _searchItems[0];
    if (it) selectSymbol(it.code, it.market);
  }} else if (e.key === 'Escape') {{
    hideSearch();
    _searchInput.blur();
  }}
}});

document.addEventListener('click', (e) => {{
  if (!e.target.closest('.search-box')) hideSearch();
}});

function hideSearch() {{
  _searchResultsEl.style.display = 'none';
  _searchItems = [];
  _searchIdx = -1;
}}

function highlightSearch() {{
  Array.from(_searchResultsEl.children).forEach((el, i) => {{
    el.classList.toggle('active', i === _searchIdx);
    if (i === _searchIdx) el.scrollIntoView({{ block: 'nearest' }});
  }});
}}

window.onSearchResults = function(items) {{
  _searchItems = items || [];
  _searchIdx = items.length ? 0 : -1;
  if (!items.length) {{ hideSearch(); return; }}
  _searchResultsEl.innerHTML = items.map((it, i) =>
    `<div class="sr${{i === 0 ? ' active' : ''}}" data-i="${{i}}">
      <span class="sr-code">${{escapeHtml(it.code)}}</span>
      <span class="sr-market">${{escapeHtml(it.market)}}</span>
      <span class="sr-name">${{escapeHtml(it.name)}}</span>
    </div>`
  ).join('');
  Array.from(_searchResultsEl.children).forEach((el) => {{
    el.addEventListener('click', () => {{
      const it = _searchItems[parseInt(el.dataset.i, 10)];
      if (it) selectSymbol(it.code, it.market);
    }});
  }});
  _searchResultsEl.style.display = 'block';
}};

function selectSymbol(code, market) {{
  hideSearch();
  _searchInput.value = '';
  _searchInput.blur();
  document.getElementById('status').textContent = '로딩...';
  // 심볼 전환 — 상태 리셋
  window._loading = false;
  window._noMore = false;
  window._savedRange = null;
  window.ipc && window.ipc.postMessage(JSON.stringify({{ type: 'select', code: code, market: market }}));
}}

function escapeHtml(s) {{
  return String(s).replace(/[&<>"']/g, c => ({{
    '&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;','\'':'&#39;'
  }}[c]));
}}

// onChartData에 meta가 오면 헤더 갱신
const _origOnChartData = window.onChartData;
window.onChartData = function(p) {{
  _origOnChartData(p);
  if (p.meta) {{
    document.querySelector('h1').textContent = `[${{p.meta.symbol}}] ${{p.meta.name}}`;
    document.title = `[${{p.meta.symbol}}] ${{p.meta.name}} — kis-cli`;
  }}
}};
</script></body></html>"#,
        symbol = symbol,
        name = name,
        date = r.date,
        bars = r.bars,
        price = fmt_num(r.current_price),
        rsi = r.rsi.value.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "-".into()),
        align = r.ma.alignment,
        signals_html = signals_html,
        payload = payload,
    )
}

/// series + 지표 전체를 JS `onChartData({...})` 형태로 직렬화.
/// 포함: 캔들, 거래량, MA5/20/60, 볼린저 상/하, 일목 4선, MACD 선·시그널·히스토그램.
pub fn series_to_js_payload(s: &Series) -> String {
    use crate::analysis::{bollinger, ichimoku, macd, sma};
    let n = s.closes.len();
    let ma5 = sma(&s.closes, 5);
    let ma20 = sma(&s.closes, 20);
    let ma60 = sma(&s.closes, 60);
    let bb = bollinger(&s.closes, 20, 2.0);
    let m = macd(&s.closes, 12, 26, 9);
    let ic = ichimoku(&s.high, &s.low, &s.closes);

    // 시계열 데이터 빌더 (time,value 배열)
    let line = |vals: &[f64]| -> String {
        let mut out = String::from("[");
        for i in 0..n {
            if i < vals.len() && !vals[i].is_nan() {
                if !out.ends_with('[') { out.push(','); }
                out.push_str(&format!(r#"{{"time":"{}","value":{}}}"#, to_iso_date(&s.dates[i]), vals[i]));
            }
        }
        out.push(']');
        out
    };

    // candles
    let mut candles = String::from("[");
    for i in 0..n {
        let t = to_iso_date(&s.dates[i]);
        if !candles.ends_with('[') { candles.push(','); }
        candles.push_str(&format!(
            r#"{{"time":"{}","open":{},"high":{},"low":{},"close":{}}}"#,
            t, nan_or(s.open[i]), nan_or(s.high[i]), nan_or(s.low[i]), nan_or(s.closes[i]),
        ));
    }
    candles.push(']');

    // volume (histogram with color based on up/down day)
    let mut volume = String::from("[");
    for i in 0..n {
        let t = to_iso_date(&s.dates[i]);
        if !volume.ends_with('[') { volume.push(','); }
        let color = if i > 0 && s.closes[i] >= s.closes[i - 1] { "#26a69a88" } else { "#ef535088" };
        volume.push_str(&format!(
            r#"{{"time":"{}","value":{},"color":"{}"}}"#,
            t, nan_or(s.volume[i]), color
        ));
    }
    volume.push(']');

    // Ichimoku 선행스팬: 과거(0..n) + 미래 26봉(n..n+26) 투영.
    // `ichimoku()`는 길이 n+shift(26) 배열 반환 — 뒤쪽 26개가 미래 구름.
    let future_dates = generate_future_dates(&s.dates, 26);
    let mut senkou_a_data = String::from("[");
    let mut senkou_b_data = String::from("[");
    // 과거 구간
    for i in 0..n {
        let t = to_iso_date(&s.dates[i]);
        if i < ic.senkou_a.len() && !ic.senkou_a[i].is_nan() {
            if !senkou_a_data.ends_with('[') { senkou_a_data.push(','); }
            senkou_a_data.push_str(&format!(r#"{{"time":"{}","value":{}}}"#, t, ic.senkou_a[i]));
        }
        if i < ic.senkou_b.len() && !ic.senkou_b[i].is_nan() {
            if !senkou_b_data.ends_with('[') { senkou_b_data.push(','); }
            senkou_b_data.push_str(&format!(r#"{{"time":"{}","value":{}}}"#, t, ic.senkou_b[i]));
        }
    }
    // 미래 구간 (투영된 구름)
    for i in 0..future_dates.len() {
        let idx = n + i;
        let t = to_iso_date(&future_dates[i]);
        if idx < ic.senkou_a.len() && !ic.senkou_a[idx].is_nan() {
            if !senkou_a_data.ends_with('[') { senkou_a_data.push(','); }
            senkou_a_data.push_str(&format!(r#"{{"time":"{}","value":{}}}"#, t, ic.senkou_a[idx]));
        }
        if idx < ic.senkou_b.len() && !ic.senkou_b[idx].is_nan() {
            if !senkou_b_data.ends_with('[') { senkou_b_data.push(','); }
            senkou_b_data.push_str(&format!(r#"{{"time":"{}","value":{}}}"#, t, ic.senkou_b[idx]));
        }
    }
    senkou_a_data.push(']');
    senkou_b_data.push(']');

    // MACD histogram (color per bar)
    let mut macd_hist = String::from("[");
    for i in 0..n {
        if !m.histogram[i].is_nan() {
            let t = to_iso_date(&s.dates[i]);
            if !macd_hist.ends_with('[') { macd_hist.push(','); }
            let color = if m.histogram[i] >= 0.0 { "#26a69a88" } else { "#ef535088" };
            macd_hist.push_str(&format!(
                r#"{{"time":"{}","value":{},"color":"{}"}}"#,
                t, m.histogram[i], color
            ));
        }
    }
    macd_hist.push(']');

    format!(
        r#"{{"candles":{candles},"volume":{volume},"ma5":{ma5},"ma20":{ma20},"ma60":{ma60},"bbu":{bbu},"bbl":{bbl},"tenkan":{tenkan},"kijun":{kijun},"senkouA":{senkou_a},"senkouB":{senkou_b},"macd":{macd_line},"signal":{macd_sig},"hist":{macd_hist}}}"#,
        candles = candles,
        volume = volume,
        ma5 = line(&ma5),
        ma20 = line(&ma20),
        ma60 = line(&ma60),
        bbu = line(&bb.upper),
        bbl = line(&bb.lower),
        tenkan = line(&ic.tenkan),
        kijun = line(&ic.kijun),
        senkou_a = senkou_a_data,
        senkou_b = senkou_b_data,
        macd_line = line(&m.macd),
        macd_sig = line(&m.signal),
        macd_hist = macd_hist,
    )
}

/// 일목 선행스팬을 미래 N봉 투영하기 위한 날짜 생성.
/// 입력 dates의 간격을 보고 일/주/월봉 판별:
/// - 중앙값 gap <= 3일 → 일봉 (주말 스킵하며 business day 생성)
/// - 그 외 → 중앙값 gap만큼 등간격 덧셈
fn generate_future_dates(dates: &[String], count: usize) -> Vec<String> {
    use chrono::Datelike;
    let n = dates.len();
    if n == 0 { return vec![]; }
    let last = match chrono::NaiveDate::parse_from_str(&dates[n - 1], "%Y%m%d") {
        Ok(d) => d,
        Err(_) => return vec![],
    };
    // 중앙값 gap 계산
    let median_gap = if n >= 2 {
        let mut gaps: Vec<i64> = dates
            .windows(2)
            .filter_map(|w| {
                let a = chrono::NaiveDate::parse_from_str(&w[0], "%Y%m%d").ok()?;
                let b = chrono::NaiveDate::parse_from_str(&w[1], "%Y%m%d").ok()?;
                Some((b - a).num_days())
            })
            .collect();
        gaps.sort();
        if gaps.is_empty() { 1 } else { gaps[gaps.len() / 2].max(1) }
    } else { 1 };

    let mut result = Vec::with_capacity(count);
    if median_gap <= 3 {
        // 일봉: 주말 스킵
        let mut d = last;
        while result.len() < count {
            d += chrono::Duration::days(1);
            let wd = d.weekday();
            if wd != chrono::Weekday::Sat && wd != chrono::Weekday::Sun {
                result.push(d.format("%Y%m%d").to_string());
            }
        }
    } else {
        // 주봉·월봉: 등간격
        for i in 1..=count {
            let d = last + chrono::Duration::days(median_gap * i as i64);
            result.push(d.format("%Y%m%d").to_string());
        }
    }
    result
}

fn to_iso_date(yyyymmdd: &str) -> String {
    if yyyymmdd.len() == 8 {
        format!("{}-{}-{}", &yyyymmdd[..4], &yyyymmdd[4..6], &yyyymmdd[6..8])
    } else {
        yyyymmdd.to_string()
    }
}

fn nan_or(v: f64) -> String {
    if v.is_nan() { "null".into() } else { format!("{}", v) }
}

fn fmt_short(v: f64) -> String {
    if v.abs() >= 1_000_000.0 { format!("{:.1}M", v / 1_000_000.0) }
    else if v.abs() >= 1_000.0 { format!("{:.1}K", v / 1_000.0) }
    else { format!("{:.2}", v) }
}

fn opt(v: f64) -> Option<f64> { if v.is_nan() { None } else { Some(v) } }
fn opt_idx(arr: &[f64], i: usize) -> Option<f64> {
    arr.get(i).copied().and_then(opt)
}

fn ma_alignment(vals: &[Option<f64>]) -> String {
    let nums: Vec<f64> = vals.iter().filter_map(|v| *v).collect();
    if nums.len() < 2 { return "데이터 부족".into(); }
    let asc = nums.windows(2).all(|w| w[0] >= w[1]);
    let desc = nums.windows(2).all(|w| w[0] <= w[1]);
    if asc { "정배열".into() }
    else if desc { "역배열".into() }
    else { "혼조".into() }
}

fn macd_cross(m: &crate::analysis::Macd) -> String {
    let n = m.histogram.len();
    if n < 2 { return "데이터 부족".into(); }
    let h = m.histogram[n - 1];
    let ph = m.histogram[n - 2];
    if ph.is_nan() || h.is_nan() { return "데이터 부족".into(); }
    if ph < 0.0 && h >= 0.0 { "골든크로스 (방금)".into() }
    else if ph > 0.0 && h <= 0.0 { "데드크로스 (방금)".into() }
    else if h > 0.0 { "상승 우세".into() }
    else if h < 0.0 { "하락 우세".into() }
    else { "중립".into() }
}

fn print_human(r: &Report) {
    println!("[{}] {} — 분석 ({})", r.symbol, r.name, r.date);
    println!("현재가: {}  (bars: {}일)", fmt_num(r.current_price), r.bars);
    println!();

    if let Some(chart) = r.chart.as_ref() {
        println!("{}", chart);
        println!();
    }

    println!("── 이평선 ──");
    if let Some(v) = r.ma.ma5   { println!("MA5:    {}", fmt_num(v)); }
    if let Some(v) = r.ma.ma20  { println!("MA20:   {}", fmt_num(v)); }
    if let Some(v) = r.ma.ma60  { println!("MA60:   {}", fmt_num(v)); }
    if let Some(v) = r.ma.ma120 { println!("MA120:  {}", fmt_num(v)); }
    println!("배열:   {}", r.ma.alignment);
    println!();

    println!("── RSI(14) ──");
    if let Some(v) = r.rsi.value {
        println!("RSI: {:.2}  ({})", v, r.rsi.state);
    } else { println!("데이터 부족"); }
    println!();

    println!("── MACD(12/26/9) ──");
    if let (Some(m), Some(s), Some(h)) = (r.macd.macd, r.macd.signal, r.macd.histogram) {
        println!("MACD:   {:+.2}", m);
        println!("Signal: {:+.2}", s);
        println!("Hist:   {:+.2}  ({})", h, r.macd.cross);
    }
    println!();

    println!("── 볼린저(20, 2σ) ──");
    if let (Some(u), Some(m), Some(l)) = (r.bollinger.upper, r.bollinger.middle, r.bollinger.lower) {
        println!("Upper:  {}", fmt_num(u));
        println!("Middle: {}", fmt_num(m));
        println!("Lower:  {}", fmt_num(l));
        if let Some(pb) = r.bollinger.percent_b {
            println!("%B:     {:.2}  (0=하단, 1=상단)", pb);
        }
        if let Some(bw) = r.bollinger.bandwidth_pct {
            println!("폭:     {:.2}%", bw);
        }
    }
    println!();

    println!("── 일목균형표(9/26/52) ──");
    if let Some(v) = r.ichimoku.tenkan      { println!("전환선:  {}", fmt_num(v)); }
    if let Some(v) = r.ichimoku.kijun       { println!("기준선:  {}", fmt_num(v)); }
    if let Some(v) = r.ichimoku.senkou_a_now { println!("선행A(오늘 구름): {}", fmt_num(v)); }
    if let Some(v) = r.ichimoku.senkou_b_now { println!("선행B(오늘 구름): {}", fmt_num(v)); }
    println!("구름:    {} / 현재가 {}", r.ichimoku.cloud_color, r.ichimoku.price_vs_cloud);
    println!("후행:    {}", r.ichimoku.chikou_signal);
    println!();

    if !r.signals.is_empty() {
        println!("── 시그널 ──");
        for s in &r.signals {
            println!("  · {}", s);
        }
    }
}

fn fmt_num(v: f64) -> String {
    if v.abs() >= 1000.0 {
        format_number(&format!("{:.0}", v))
    } else {
        format!("{:.4}", v)
    }
}
