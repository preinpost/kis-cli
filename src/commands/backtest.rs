//! `kis backtest <symbol>` — 내장 전략을 과거 캔들에 시뮬레이션.
//!
//! 확장 기능:
//! - 전략: `ma-cross` / `rsi` / `macd` / `bollinger` / `ichimoku`
//! - 포지션: long-only (기본) 또는 `--allow-short` 양방향
//! - 레버리지(`--leverage`), 슬리피지(`--slippage-bps`)
//! - 리스크 관리: `--stop-loss-pct`, `--take-profit-pct` (종가 기준 체크)
//! - 리스크 지표: CAGR / Sharpe / Sortino / Calmar / MDD
//! - 파라미터 스윕: `--sweep` (전략별 내장 그리드)
//! - 차트 마커: `--chart` (wry 정적 뷰어, 진입·청산 표시)
//!
//! 단순화 가정:
//! - 모든 체결은 해당 봉의 **종가**에 일어남 (인트라바 경로 무시)
//! - `--stop-loss-pct` / `--take-profit-pct` 도 종가 터치에서만 트리거
//! - 레버리지는 수익률·수수료에 승수로만 작용 (증거금·청산 모델 없음)

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::analysis::{bollinger, ichimoku, macd, obv, rsi, sma};
use crate::client::KisClient;
use crate::commands::analyze::{self, Series};
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::symbols::ResolveMode;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum StrategyKind {
    MaCross,
    Rsi,
    Macd,
    Bollinger,
    Ichimoku,
    Obv,
    Manual,
}

impl StrategyKind {
    fn as_str(&self) -> &'static str {
        match self {
            StrategyKind::MaCross => "ma-cross",
            StrategyKind::Rsi => "rsi",
            StrategyKind::Macd => "macd",
            StrategyKind::Bollinger => "bollinger",
            StrategyKind::Ichimoku => "ichimoku",
            StrategyKind::Obv => "obv",
            StrategyKind::Manual => "manual",
        }
    }
}

#[derive(Clone, Debug)]
enum Strategy {
    MaCross { fast: usize, slow: usize },
    Rsi { period: usize, oversold: f64, overbought: f64 },
    Macd,
    Bollinger { period: usize, sigma: f64 },
    Ichimoku,
    Obv { period: usize },
    /// 고정 진입/청산. 진입일 도달 전엔 flat, 이후 direction 방향 유지, 청산일(옵션) 이후 다시 flat.
    Manual {
        entry_date: String,
        exit_date: Option<String>,
        direction: i8,
    },
}

impl Strategy {
    fn label(&self) -> String {
        match self {
            Strategy::MaCross { fast, slow } => format!("ma-cross({fast}/{slow})"),
            Strategy::Rsi { period, oversold, overbought } => {
                format!("rsi({period}, {oversold:.0}/{overbought:.0})")
            }
            Strategy::Macd => "macd(12/26/9)".into(),
            Strategy::Bollinger { period, sigma } => format!("bollinger({period}, {sigma}σ)"),
            Strategy::Ichimoku => "ichimoku(9/26/52)".into(),
            Strategy::Obv { period } => format!("obv({period})"),
            Strategy::Manual { entry_date, exit_date, direction } => {
                let dir = if *direction > 0 { "long" } else { "short" };
                match exit_date {
                    Some(ex) => format!("manual({} {} → {})", dir, entry_date, ex),
                    None => format!("manual({} {} → end)", dir, entry_date),
                }
            }
        }
    }
}

pub struct Params {
    pub strategy: StrategyKind,
    pub period: char,
    pub from: Option<String>,
    pub to: Option<String>,
    pub fee_bps: f64,
    pub slippage_bps: f64,
    pub allow_short: bool,
    pub leverage: f64,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
    /// Manual 전략 전용: 진입일 (YYYYMMDD, normalize 이후 형식)
    pub manual_entry_date: Option<String>,
    pub manual_exit_date: Option<String>,
    /// "long" 또는 "short"
    pub manual_direction: Option<String>,
}

pub struct RunOpts {
    pub json: bool,
    pub sweep: bool,
}

pub async fn run(
    client: &KisClient,
    symbol: &str,
    mode: ResolveMode,
    params: Params,
    opts: RunOpts,
    pick: Option<usize>,
) -> Result<()> {
    if !matches!(mode, ResolveMode::Domestic | ResolveMode::Overseas) {
        return Err(anyhow!("backtest는 국내/해외 주식만 지원"));
    }
    let sym = resolve_symbol(symbol, mode, pick)?;
    let name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };

    let series = fetch_range(
        client,
        &sym.code,
        mode,
        params.period,
        params.from.as_deref(),
        params.to.as_deref(),
    )
    .await?;

    if series.closes.len() < 30 {
        return Err(anyhow!(
            "데이터 부족 ({}봉) — 백테스트에 최소 30봉 이상 필요",
            series.closes.len()
        ));
    }

    if opts.sweep {
        return run_sweep(&sym.code, &name, &series, &params, opts.json);
    }

    let strategy = build_strategy(&params);
    let signals = compute_signals(&strategy, &series);
    let result = simulate(&series, &signals, &params);
    let report = build_report(&sym.code, &name, &series, &strategy, &params, &result);

    if opts.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print_human(&report);
    }
    Ok(())
}

pub struct ChartPrep {
    pub title: String,
    pub html: String,
    pub code: String,
    pub name: String,
    pub series: Series,
    pub period: char,
    pub from: Option<String>,
    pub to: Option<String>,
}

/// wry 뷰어용: fetch 후 (title, html, series, ...) 반환.
/// 메인 스레드에서 `viewer::launch_backtest` 하기 전 비동기 준비 단계.
pub async fn prepare_chart(
    client: &KisClient,
    symbol: &str,
    mode: ResolveMode,
    params: Params,
    pick: Option<usize>,
) -> Result<ChartPrep> {
    if !matches!(mode, ResolveMode::Domestic | ResolveMode::Overseas) {
        return Err(anyhow!("backtest는 국내/해외 주식만 지원"));
    }
    let sym = resolve_symbol(symbol, mode, pick)?;
    let name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };

    let series = fetch_range(
        client, &sym.code, mode, params.period,
        params.from.as_deref(), params.to.as_deref(),
    ).await?;
    if series.closes.len() < 30 {
        return Err(anyhow!("데이터 부족 ({}봉) — 백테스트에 최소 30봉 이상 필요", series.closes.len()));
    }

    let payload = compute_payload_json(&sym.code, &name, &series, &params);
    let html = render_html(&sym.code, &name, &params, &payload);
    let title = format!("[{}] {} — backtest", sym.code, name);
    Ok(ChartPrep {
        title,
        html,
        code: sym.code,
        name,
        series,
        period: params.period,
        from: params.from,
        to: params.to,
    })
}

/// 재계산 가능한 코어: params 변경에 대응해 JSON payload 생성.
/// 반환값은 `window.onBacktestData(...)` 에 그대로 넘길 객체.
pub fn compute_payload_json(code: &str, name: &str, series: &Series, params: &Params) -> String {
    let strategy = build_strategy(params);
    let signals = compute_signals(&strategy, series);
    let result = simulate(series, &signals, params);
    let report = build_report(code, name, series, &strategy, params, &result);

    let candles = candles_json(series);
    let markers = markers_json(&result.trades, result.open_position.as_ref(), series);
    let overlays = overlays_json(series, &strategy);
    let equity = equity_curve_json(series, &result.equity_curve);
    let report_json = serde_json::to_string(&report).unwrap_or_else(|_| "null".into());
    let strategy_label = serde_json::to_string(&strategy.label()).unwrap_or_else(|_| "\"\"".into());
    let code_json = serde_json::to_string(code).unwrap_or_else(|_| "\"\"".into());
    let name_json = serde_json::to_string(name).unwrap_or_else(|_| "\"\"".into());

    format!(
        r#"{{"candles":{candles},"markers":{markers},"overlays":{overlays},"equity":{equity},"report":{report_json},"strategy_label":{strategy_label},"meta":{{"symbol":{code_json},"name":{name_json}}}}}"#
    )
}

/// GUI 폼 → IPC로 받은 파라미터 DTO. 기존 CLI `Params`로 변환해 재계산.
#[derive(Debug, Deserialize)]
pub struct IpcParams {
    pub strategy: String,
    pub period: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub fee_bps: f64,
    pub slippage_bps: f64,
    pub allow_short: bool,
    pub leverage: f64,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
    pub manual_entry_date: Option<String>,
    pub manual_exit_date: Option<String>,
    pub manual_direction: Option<String>,
}

/// "2024-03-15" → "20240315", "" → None, "20240315" → "20240315"
pub fn normalize_date(s: Option<String>) -> Option<String> {
    s.and_then(|x| {
        let t = x.trim();
        if t.is_empty() {
            None
        } else if t.len() == 10 && t.contains('-') {
            Some(t.replace('-', ""))
        } else {
            Some(t.to_string())
        }
    })
}

impl IpcParams {
    pub fn period_char(&self) -> char {
        self.period.chars().next().unwrap_or('D')
    }

    pub fn from_norm(&self) -> Option<String> {
        normalize_date(self.from.clone())
    }

    pub fn to_norm(&self) -> Option<String> {
        normalize_date(self.to.clone())
    }

    pub fn into_params(self, from: Option<String>, to: Option<String>) -> Params {
        let strategy = match self.strategy.as_str() {
            "rsi" => StrategyKind::Rsi,
            "macd" => StrategyKind::Macd,
            "bollinger" => StrategyKind::Bollinger,
            "ichimoku" => StrategyKind::Ichimoku,
            "obv" => StrategyKind::Obv,
            "manual" => StrategyKind::Manual,
            _ => StrategyKind::MaCross,
        };
        let period = self.period_char();
        Params {
            strategy,
            period,
            from,
            to,
            fee_bps: self.fee_bps,
            slippage_bps: self.slippage_bps,
            allow_short: self.allow_short,
            leverage: self.leverage,
            stop_loss_pct: self.stop_loss_pct,
            take_profit_pct: self.take_profit_pct,
            fast: self.fast,
            slow: self.slow,
            rsi_period: self.rsi_period,
            rsi_oversold: self.rsi_oversold,
            rsi_overbought: self.rsi_overbought,
            bb_period: self.bb_period,
            bb_sigma: self.bb_sigma,
            obv_period: self.obv_period,
            manual_entry_date: normalize_date(self.manual_entry_date),
            manual_exit_date: normalize_date(self.manual_exit_date),
            manual_direction: self.manual_direction,
        }
    }
}

/// 주어진 시리즈·파라미터로 **마지막 봉의 신호** 만 반환. signal-watch 가 사용.
/// 반환값: +1 (long 유지 신호), 0 (flat), -1 (short 유지 신호).
pub fn latest_signal(series: &Series, params: &Params) -> i8 {
    let strategy = build_strategy(params);
    let signals = compute_signals(&strategy, series);
    signals.last().copied().unwrap_or(0)
}

/// `fetch_range` 를 외부(viewer)에서 쓸 수 있게 public 래퍼 제공.
pub async fn fetch_series_range(
    client: &KisClient,
    code: &str,
    mode: ResolveMode,
    period: char,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Series> {
    fetch_range(client, code, mode, period, from, to).await
}

fn build_strategy(p: &Params) -> Strategy {
    match p.strategy {
        StrategyKind::MaCross => Strategy::MaCross {
            fast: p.fast.unwrap_or(20),
            slow: p.slow.unwrap_or(60),
        },
        StrategyKind::Rsi => Strategy::Rsi {
            period: p.rsi_period.unwrap_or(14),
            oversold: p.rsi_oversold.unwrap_or(30.0),
            overbought: p.rsi_overbought.unwrap_or(70.0),
        },
        StrategyKind::Macd => Strategy::Macd,
        StrategyKind::Bollinger => Strategy::Bollinger {
            period: p.bb_period.unwrap_or(20),
            sigma: p.bb_sigma.unwrap_or(2.0),
        },
        StrategyKind::Ichimoku => Strategy::Ichimoku,
        StrategyKind::Obv => Strategy::Obv {
            period: p.obv_period.unwrap_or(20),
        },
        StrategyKind::Manual => {
            let entry_date = p.manual_entry_date.clone().unwrap_or_default();
            let exit_date = p.manual_exit_date.clone();
            let direction = match p.manual_direction.as_deref() {
                Some("short") => -1i8,
                _ => 1i8,
            };
            Strategy::Manual { entry_date, exit_date, direction }
        }
    }
}

async fn fetch_range(
    client: &KisClient,
    code: &str,
    mode: ResolveMode,
    period: char,
    from: Option<&str>,
    to: Option<&str>,
) -> Result<Series> {
    let end = to
        .map(String::from)
        .unwrap_or_else(|| chrono::Local::now().format("%Y%m%d").to_string());

    // --from 미지정 시 기본 룩백 (일봉 5청크≈2년, 주봉 2청크≈4.4년, 월봉 1청크≈10년)
    let default_from = if from.is_none() {
        let (per_chunk, chunks): (i64, i64) = match period {
            'W' => (800, 2),
            'M' => (3600, 1),
            _ => (150, 5),
        };
        let end_dt = chrono::NaiveDate::parse_from_str(&end, "%Y%m%d")
            .unwrap_or_else(|_| chrono::Local::now().date_naive());
        Some((end_dt - chrono::Duration::days(per_chunk * chunks)).format("%Y%m%d").to_string())
    } else {
        None
    };
    let from_eff: Option<&str> = from.or(default_from.as_deref());

    let mut series = fetch_chunk(client, code, mode, period, &end).await?;

    if let Some(from_str) = from_eff {
        // KIS API 초당 호출 제한(EGW00201) 회피용 인터-청크 딜레이
        const CHUNK_DELAY: std::time::Duration = std::time::Duration::from_millis(250);
        for iter in 0..30 {
            let Some(oldest) = series.dates.first() else { break };
            if oldest.as_str() <= from_str {
                break;
            }
            if iter > 0 {
                tokio::time::sleep(CHUNK_DELAY).await;
            }
            let prev_end = chrono::NaiveDate::parse_from_str(oldest, "%Y%m%d")
                .map_err(|_| anyhow!("날짜 파싱 실패: {oldest}"))?
                - chrono::Duration::days(1);
            let next_to = prev_end.format("%Y%m%d").to_string();
            let chunk = fetch_chunk(client, code, mode, period, &next_to).await?;
            if chunk.dates.is_empty() {
                break;
            }
            series = prepend(chunk, series);
        }
    }
    Ok(trim_range(series, from_eff, to))
}

async fn fetch_chunk(
    client: &KisClient,
    code: &str,
    mode: ResolveMode,
    period: char,
    to_date: &str,
) -> Result<Series> {
    match mode {
        ResolveMode::Domestic => analyze::fetch_domestic_chunk(client, code, period, to_date).await,
        ResolveMode::Overseas => analyze::fetch_overseas_chunk(client, code, period, to_date).await,
        _ => Err(anyhow!("주식 외 시장은 미지원")),
    }
}

fn prepend(older: Series, current: Series) -> Series {
    let mut s = older;
    s.dates.extend(current.dates);
    s.open.extend(current.open);
    s.high.extend(current.high);
    s.low.extend(current.low);
    s.closes.extend(current.closes);
    s.volume.extend(current.volume);
    s
}

fn trim_range(s: Series, from: Option<&str>, to: Option<&str>) -> Series {
    let lo = from
        .and_then(|f| s.dates.iter().position(|d| d.as_str() >= f))
        .unwrap_or(0);
    let hi = to
        .and_then(|t| s.dates.iter().rposition(|d| d.as_str() <= t).map(|i| i + 1))
        .unwrap_or(s.dates.len());
    if lo == 0 && hi == s.dates.len() {
        return s;
    }
    Series {
        dates: s.dates[lo..hi].to_vec(),
        open: s.open[lo..hi].to_vec(),
        high: s.high[lo..hi].to_vec(),
        low: s.low[lo..hi].to_vec(),
        closes: s.closes[lo..hi].to_vec(),
        volume: s.volume[lo..hi].to_vec(),
    }
}

/// 각 바에서 "바 종가 이후 유지하고 싶은 포지션": -1 short, 0 flat, +1 long.
fn compute_signals(strategy: &Strategy, s: &Series) -> Vec<i8> {
    let n = s.closes.len();
    let mut out = vec![0i8; n];
    match strategy {
        Strategy::MaCross { fast, slow } => {
            let f = sma(&s.closes, *fast);
            let sl = sma(&s.closes, *slow);
            for i in 0..n {
                if f[i].is_nan() || sl[i].is_nan() {
                    continue;
                }
                out[i] = if f[i] > sl[i] { 1 } else if f[i] < sl[i] { -1 } else { 0 };
            }
        }
        Strategy::Rsi { period, oversold, overbought } => {
            let r = rsi(&s.closes, *period);
            let mut pos: i8 = 0;
            for i in 0..n {
                if !r[i].is_nan() {
                    if r[i] <= *oversold {
                        pos = 1;
                    } else if r[i] >= *overbought {
                        pos = -1;
                    }
                }
                out[i] = pos;
            }
        }
        Strategy::Macd => {
            let m = macd(&s.closes, 12, 26, 9);
            for i in 0..n {
                if m.histogram[i].is_nan() {
                    continue;
                }
                out[i] = if m.histogram[i] > 0.0 { 1 } else if m.histogram[i] < 0.0 { -1 } else { 0 };
            }
        }
        Strategy::Bollinger { period, sigma } => {
            let b = bollinger(&s.closes, *period, *sigma);
            let mut pos: i8 = 0;
            for i in 0..n {
                let c = s.closes[i];
                let (u, m, l) = (b.upper[i], b.middle[i], b.lower[i]);
                if !u.is_nan() && !m.is_nan() && !l.is_nan() && !c.is_nan() {
                    if pos == 0 {
                        if c < l {
                            pos = 1;
                        } else if c > u {
                            pos = -1;
                        }
                    } else if pos == 1 && c >= m {
                        pos = 0;
                    } else if pos == -1 && c <= m {
                        pos = 0;
                    }
                }
                out[i] = pos;
            }
        }
        Strategy::Ichimoku => {
            let ic = ichimoku(&s.high, &s.low, &s.closes);
            for i in 0..n {
                let a = ic.senkou_a[i];
                let b = ic.senkou_b[i];
                let t = ic.tenkan[i];
                let k = ic.kijun[i];
                let c = s.closes[i];
                if a.is_nan() || b.is_nan() || t.is_nan() || k.is_nan() || c.is_nan() {
                    continue;
                }
                let top = a.max(b);
                let bot = a.min(b);
                out[i] = if c > top && t > k {
                    1
                } else if c < bot && t < k {
                    -1
                } else {
                    0
                };
            }
        }
        Strategy::Obv { period } => {
            let o = obv(&s.closes, &s.volume);
            let sig = sma(&o, *period);
            let mut pos: i8 = 0;
            for i in 0..n {
                if !o[i].is_nan() && !sig[i].is_nan() {
                    if o[i] > sig[i] {
                        pos = 1;
                    } else if o[i] < sig[i] {
                        pos = -1;
                    }
                }
                out[i] = pos;
            }
        }
        Strategy::Manual { entry_date, exit_date, direction } => {
            for i in 0..n {
                let d = s.dates[i].as_str();
                let entered = !entry_date.is_empty() && d >= entry_date.as_str();
                let exited = exit_date.as_ref().map_or(false, |e| d >= e.as_str());
                out[i] = if entered && !exited { *direction } else { 0 };
            }
        }
    }
    out
}

#[derive(Debug, Serialize)]
pub struct Trade {
    pub entry_date: String,
    pub entry_price: f64,
    pub exit_date: String,
    pub exit_price: f64,
    /// +1 long, -1 short
    pub direction: i8,
    pub pnl_pct: f64,
    /// 정상 신호 청산 vs 스탑/익절 강제 청산 표시
    pub exit_reason: String,
}

#[derive(Debug, Serialize)]
pub struct OpenPosition {
    pub entry_date: String,
    pub entry_price: f64,
    pub direction: i8,
    pub unrealized_pct: f64,
}

struct SimResult {
    trades: Vec<Trade>,
    open_position: Option<OpenPosition>,
    equity_curve: Vec<f64>,
    bar_returns: Vec<f64>,
    final_equity: f64,
    peak_equity: f64,
    max_drawdown_pct: f64,
}

fn simulate(series: &Series, signals: &[i8], p: &Params) -> SimResult {
    let n = series.closes.len();
    let fee = p.fee_bps / 10_000.0;
    let slip = p.slippage_bps / 10_000.0;
    let leverage = p.leverage.max(0.01);
    let trade_cost = (fee + slip) * leverage;

    let mut equity = 1.0_f64;
    let mut pos: i8 = 0;
    let mut entry_price = 0.0_f64;
    let mut entry_date = String::new();
    let mut entry_equity = 1.0_f64;

    let mut peak = 1.0_f64;
    let mut max_dd: f64 = 0.0;
    let mut trades: Vec<Trade> = Vec::new();
    let mut equity_curve = Vec::with_capacity(n);
    let mut bar_returns = Vec::with_capacity(n);
    let mut liquidated = false;

    for i in 0..n {
        let price = series.closes[i];
        if price.is_nan() {
            equity_curve.push(equity);
            bar_returns.push(0.0);
            continue;
        }

        // 1) 현재 포지션의 바 수익률
        let bar_ret = if i > 0 && pos != 0 && !liquidated {
            let prev = series.closes[i - 1];
            if prev.is_finite() && prev > 0.0 {
                let raw = price / prev - 1.0;
                raw * pos as f64 * leverage
            } else {
                0.0
            }
        } else {
            0.0
        };
        // -100% 미만 방지 (레버리지·숏으로 폭락)
        let capped = bar_ret.max(-1.0);
        equity *= 1.0 + capped;
        bar_returns.push(capped);

        if equity <= 1e-9 && !liquidated {
            if pos != 0 {
                trades.push(Trade {
                    entry_date: entry_date.clone(),
                    entry_price,
                    exit_date: series.dates[i].clone(),
                    exit_price: price,
                    direction: pos,
                    pnl_pct: -100.0,
                    exit_reason: "liquidated".into(),
                });
                pos = 0;
            }
            equity = 0.0;
            liquidated = true;
            equity_curve.push(equity);
            continue;
        }

        // 2) 스탑/익절 (종가 기준)
        let mut forced_reason: Option<&'static str> = None;
        if pos != 0 {
            let u = (price / entry_price - 1.0) * pos as f64 * 100.0;
            if let Some(sl) = p.stop_loss_pct {
                if u <= -sl.abs() {
                    forced_reason = Some("stop-loss");
                }
            }
            if forced_reason.is_none() {
                if let Some(tp) = p.take_profit_pct {
                    if u >= tp.abs() {
                        forced_reason = Some("take-profit");
                    }
                }
            }
        }

        // 3) 목표 포지션
        let natural = signals[i];
        let desired = if p.allow_short { natural } else { natural.max(0) };
        let target = if forced_reason.is_some() { 0 } else { desired };

        // 4) 포지션 전환
        if !liquidated && pos != target {
            if pos != 0 {
                equity *= 1.0 - trade_cost;
                let pnl_pct = (equity / entry_equity - 1.0) * 100.0;
                trades.push(Trade {
                    entry_date: entry_date.clone(),
                    entry_price,
                    exit_date: series.dates[i].clone(),
                    exit_price: price,
                    direction: pos,
                    pnl_pct,
                    exit_reason: forced_reason.unwrap_or("signal").into(),
                });
            }
            if target != 0 {
                equity *= 1.0 - trade_cost;
                entry_price = price;
                entry_date = series.dates[i].clone();
                entry_equity = equity;
            }
            pos = target;
        }

        // 5) Peak / DD
        if equity > peak {
            peak = equity;
        }
        let dd = if peak > 0.0 { (peak - equity) / peak } else { 0.0 };
        if dd > max_dd {
            max_dd = dd;
        }
        equity_curve.push(equity);
    }

    let final_price = series.closes.last().copied().unwrap_or(f64::NAN);
    let open_position = if pos != 0 && !liquidated && final_price.is_finite() && entry_price > 0.0 {
        let u = (final_price / entry_price - 1.0) * pos as f64 * 100.0;
        Some(OpenPosition {
            entry_date,
            entry_price,
            direction: pos,
            unrealized_pct: u,
        })
    } else {
        None
    };

    SimResult {
        trades,
        open_position,
        equity_curve,
        bar_returns,
        final_equity: equity,
        peak_equity: peak,
        max_drawdown_pct: max_dd * 100.0,
    }
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub symbol: String,
    pub name: String,
    pub strategy: String,
    pub period: String,
    pub from: String,
    pub to: String,
    pub bars: usize,
    pub fee_bps: f64,
    pub slippage_bps: f64,
    pub leverage: f64,
    pub allow_short: bool,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,

    pub total_return_pct: f64,
    pub buy_and_hold_pct: f64,
    pub cagr_pct: f64,
    pub max_drawdown_pct: f64,
    pub sharpe: f64,
    pub sortino: f64,
    pub calmar: f64,

    pub n_trades: usize,
    pub win_rate_pct: f64,
    pub avg_win_pct: f64,
    pub avg_loss_pct: f64,
    pub profit_factor: f64,
    pub n_stops: usize,
    pub n_takes: usize,
    pub n_liquidations: usize,

    pub trades: Vec<Trade>,
    pub open_position: Option<OpenPosition>,
}

fn build_report(
    code: &str,
    name: &str,
    s: &Series,
    strategy: &Strategy,
    p: &Params,
    r: &SimResult,
) -> Report {
    let total_return_pct = (r.final_equity - 1.0) * 100.0;
    let bars = s.closes.len();

    let first_price = s.closes.first().copied().unwrap_or(f64::NAN);
    let last_price = s.closes.last().copied().unwrap_or(f64::NAN);
    let buy_and_hold_pct = if first_price.is_finite() && last_price.is_finite() && first_price > 0.0 {
        (last_price / first_price - 1.0) * 100.0
    } else {
        0.0
    };

    let periods_per_year = match p.period {
        'W' => 52.0,
        'M' => 12.0,
        _ => 252.0,
    };
    let years = bars as f64 / periods_per_year;
    let cagr_pct = if years > 1e-6 && r.final_equity > 0.0 {
        (r.final_equity.powf(1.0 / years) - 1.0) * 100.0
    } else {
        0.0
    };

    let (sharpe, sortino) = risk_ratios(&r.bar_returns, periods_per_year);
    let calmar = if r.max_drawdown_pct > 1e-9 {
        cagr_pct / r.max_drawdown_pct
    } else {
        0.0
    };

    let n_trades = r.trades.len();
    let wins: Vec<f64> = r.trades.iter().filter(|t| t.pnl_pct > 0.0).map(|t| t.pnl_pct).collect();
    let losses: Vec<f64> = r.trades.iter().filter(|t| t.pnl_pct <= 0.0).map(|t| t.pnl_pct).collect();
    let win_rate_pct = if n_trades > 0 {
        wins.len() as f64 / n_trades as f64 * 100.0
    } else {
        0.0
    };
    let avg_win_pct = if wins.is_empty() { 0.0 } else { wins.iter().sum::<f64>() / wins.len() as f64 };
    let avg_loss_pct = if losses.is_empty() { 0.0 } else { losses.iter().sum::<f64>() / losses.len() as f64 };
    let gross_win = wins.iter().sum::<f64>();
    let gross_loss: f64 = losses.iter().map(|v| v.abs()).sum();
    let profit_factor = if gross_loss > 1e-9 { gross_win / gross_loss } else { 0.0 };

    let n_stops = r.trades.iter().filter(|t| t.exit_reason == "stop-loss").count();
    let n_takes = r.trades.iter().filter(|t| t.exit_reason == "take-profit").count();
    let n_liquidations = r.trades.iter().filter(|t| t.exit_reason == "liquidated").count();

    Report {
        symbol: code.into(),
        name: name.into(),
        strategy: strategy.label(),
        period: p.period.to_string(),
        from: s.dates.first().cloned().unwrap_or_default(),
        to: s.dates.last().cloned().unwrap_or_default(),
        bars,
        fee_bps: p.fee_bps,
        slippage_bps: p.slippage_bps,
        leverage: p.leverage,
        allow_short: p.allow_short,
        stop_loss_pct: p.stop_loss_pct,
        take_profit_pct: p.take_profit_pct,
        total_return_pct,
        buy_and_hold_pct,
        cagr_pct,
        max_drawdown_pct: r.max_drawdown_pct,
        sharpe,
        sortino,
        calmar,
        n_trades,
        win_rate_pct,
        avg_win_pct,
        avg_loss_pct,
        profit_factor,
        n_stops,
        n_takes,
        n_liquidations,
        trades: r
            .trades
            .iter()
            .map(|t| Trade {
                entry_date: t.entry_date.clone(),
                entry_price: t.entry_price,
                exit_date: t.exit_date.clone(),
                exit_price: t.exit_price,
                direction: t.direction,
                pnl_pct: t.pnl_pct,
                exit_reason: t.exit_reason.clone(),
            })
            .collect(),
        open_position: r.open_position.as_ref().map(|o| OpenPosition {
            entry_date: o.entry_date.clone(),
            entry_price: o.entry_price,
            direction: o.direction,
            unrealized_pct: o.unrealized_pct,
        }),
    }
}

fn risk_ratios(returns: &[f64], periods_per_year: f64) -> (f64, f64) {
    if returns.is_empty() {
        return (0.0, 0.0);
    }
    let n = returns.len() as f64;
    let mean = returns.iter().sum::<f64>() / n;
    let var = returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / n;
    let std = var.sqrt();
    let down_sq = returns
        .iter()
        .filter(|r| **r < 0.0)
        .map(|r| r.powi(2))
        .sum::<f64>()
        / n;
    let downside = down_sq.sqrt();
    let sharpe = if std > 1e-12 {
        mean / std * periods_per_year.sqrt()
    } else {
        0.0
    };
    let sortino = if downside > 1e-12 {
        mean / downside * periods_per_year.sqrt()
    } else {
        0.0
    };
    (sharpe, sortino)
}

// ─────────────────────────── 스윕 ───────────────────────────

fn sweep_space(kind: StrategyKind) -> Vec<Strategy> {
    match kind {
        StrategyKind::MaCross => {
            let mut out = Vec::new();
            for fast in [5usize, 10, 20] {
                for slow in [20usize, 50, 60, 120] {
                    if fast < slow {
                        out.push(Strategy::MaCross { fast, slow });
                    }
                }
            }
            out
        }
        StrategyKind::Rsi => {
            let mut out = Vec::new();
            for period in [9usize, 14, 21] {
                for oversold in [20.0f64, 25.0, 30.0] {
                    for overbought in [70.0f64, 75.0, 80.0] {
                        out.push(Strategy::Rsi { period, oversold, overbought });
                    }
                }
            }
            out
        }
        StrategyKind::Bollinger => {
            let mut out = Vec::new();
            for period in [10usize, 14, 20] {
                for sigma in [1.5f64, 2.0, 2.5] {
                    out.push(Strategy::Bollinger { period, sigma });
                }
            }
            out
        }
        StrategyKind::Macd => vec![Strategy::Macd],
        StrategyKind::Ichimoku => vec![Strategy::Ichimoku],
        StrategyKind::Obv => [10usize, 20, 30]
            .into_iter()
            .map(|period| Strategy::Obv { period })
            .collect(),
        // Manual 은 스윕 대상이 아님 (단일 시나리오)
        StrategyKind::Manual => vec![],
    }
}

#[derive(Debug, Serialize)]
struct SweepRow {
    strategy: String,
    total_return_pct: f64,
    cagr_pct: f64,
    max_drawdown_pct: f64,
    sharpe: f64,
    n_trades: usize,
    win_rate_pct: f64,
}

fn run_sweep(
    code: &str,
    name: &str,
    series: &Series,
    params: &Params,
    json: bool,
) -> Result<()> {
    let space = sweep_space(params.strategy);
    if space.len() <= 1 {
        eprintln!("전략 `{}`은 내장 튜닝 파라미터가 없습니다.", params.strategy.as_str());
    }
    let mut rows: Vec<(SweepRow, Report)> = Vec::with_capacity(space.len());
    for strategy in space {
        let signals = compute_signals(&strategy, series);
        let r = simulate(series, &signals, params);
        let rep = build_report(code, name, series, &strategy, params, &r);
        rows.push((
            SweepRow {
                strategy: rep.strategy.clone(),
                total_return_pct: rep.total_return_pct,
                cagr_pct: rep.cagr_pct,
                max_drawdown_pct: rep.max_drawdown_pct,
                sharpe: rep.sharpe,
                n_trades: rep.n_trades,
                win_rate_pct: rep.win_rate_pct,
            },
            rep,
        ));
    }
    rows.sort_by(|a, b| b.0.sharpe.partial_cmp(&a.0.sharpe).unwrap_or(std::cmp::Ordering::Equal));

    if json {
        let out: Vec<&SweepRow> = rows.iter().map(|(r, _)| r).collect();
        println!("{}", serde_json::to_string_pretty(&out)?);
        return Ok(());
    }

    println!(
        "[{}] {} — 스윕 ({}개 조합, {}-{})",
        code,
        name,
        rows.len(),
        series.dates.first().cloned().unwrap_or_default(),
        series.dates.last().cloned().unwrap_or_default(),
    );
    println!();
    println!(
        "{:<30}  {:>12}  {:>10}  {:>10}  {:>8}  {:>6}  {:>8}",
        "전략", "총수익(%)", "CAGR(%)", "MDD(%)", "Sharpe", "트레이드", "승률(%)"
    );
    println!("{}", "─".repeat(98));
    for (r, _) in rows.iter().take(15) {
        println!(
            "{:<30}  {:>+12.2}  {:>+10.2}  {:>10.2}  {:>8.2}  {:>6}  {:>8.1}",
            r.strategy,
            r.total_return_pct,
            r.cagr_pct,
            r.max_drawdown_pct,
            r.sharpe,
            r.n_trades,
            r.win_rate_pct,
        );
    }
    Ok(())
}

// ─────────────────────────── 사람 출력 ───────────────────────────

fn print_human(r: &Report) {
    println!(
        "[{}] {} — 백테스트 (전략: {}, {}-{} {}봉/{})",
        r.symbol, r.name, r.strategy, r.from, r.to, r.bars, r.period
    );
    let mut meta = Vec::new();
    meta.push(format!("수수료 {:.1}bps", r.fee_bps));
    if r.slippage_bps > 0.0 {
        meta.push(format!("슬리피지 {:.1}bps", r.slippage_bps));
    }
    if (r.leverage - 1.0).abs() > 1e-9 {
        meta.push(format!("레버리지 {:.1}x", r.leverage));
    }
    if r.allow_short {
        meta.push("숏 허용".into());
    }
    if let Some(sl) = r.stop_loss_pct {
        meta.push(format!("SL {:.1}%", sl));
    }
    if let Some(tp) = r.take_profit_pct {
        meta.push(format!("TP {:.1}%", tp));
    }
    println!("옵션: {}", meta.join(" · "));
    println!();

    println!("── 수익성 ──");
    println!("총수익률:       {:+.2}%", r.total_return_pct);
    println!(
        "Buy & Hold:     {:+.2}%  (차이 {:+.2}%p)",
        r.buy_and_hold_pct,
        r.total_return_pct - r.buy_and_hold_pct
    );
    println!("CAGR:           {:+.2}%", r.cagr_pct);
    println!();

    println!("── 리스크 ──");
    println!("최대낙폭(MDD):  -{:.2}%", r.max_drawdown_pct);
    println!("Sharpe:         {:.2}", r.sharpe);
    println!("Sortino:        {:.2}", r.sortino);
    println!("Calmar:         {:.2}", r.calmar);
    println!();

    println!("── 트레이드 ──");
    println!("총 트레이드:    {}", r.n_trades);
    if r.n_trades > 0 {
        println!("승률:           {:.1}%", r.win_rate_pct);
        println!("평균 수익:      {:+.2}%", r.avg_win_pct);
        println!("평균 손실:      {:+.2}%", r.avg_loss_pct);
        println!("Profit Factor:  {:.2}", r.profit_factor);
    }
    if r.n_stops > 0 || r.n_takes > 0 || r.n_liquidations > 0 {
        println!(
            "강제청산:       스탑 {} · 익절 {} · 파산 {}",
            r.n_stops, r.n_takes, r.n_liquidations
        );
    }

    if let Some(op) = &r.open_position {
        println!();
        println!("── 미청산 포지션 ──");
        println!(
            "{} @ {}  ({:+.2}%, {})",
            op.entry_date,
            format_price(op.entry_price),
            op.unrealized_pct,
            if op.direction > 0 { "long" } else { "short" }
        );
    }

    if !r.trades.is_empty() {
        println!();
        println!("── 최근 트레이드 (최대 10건) ──");
        let start = r.trades.len().saturating_sub(10);
        for t in &r.trades[start..] {
            let dir = if t.direction > 0 { "L" } else { "S" };
            let tag = if t.exit_reason == "signal" {
                String::new()
            } else {
                format!(" [{}]", t.exit_reason)
            };
            println!(
                "{} {} → {}   {} → {}   {:+.2}%{}",
                dir,
                t.entry_date,
                t.exit_date,
                format_price(t.entry_price),
                format_price(t.exit_price),
                t.pnl_pct,
                tag,
            );
        }
    }
}

fn format_price(v: f64) -> String {
    if v.abs() >= 1000.0 {
        format_number(&format!("{:.0}", v))
    } else {
        format!("{:.4}", v)
    }
}

// ─────────────────────────── 차트 (wry) ───────────────────────────

fn render_html(code: &str, name: &str, params: &Params, initial_payload: &str) -> String {
    let strat_str = params.strategy.as_str();
    let fast = params.fast.unwrap_or(20);
    let slow = params.slow.unwrap_or(60);
    let rsi_p = params.rsi_period.unwrap_or(14);
    let rsi_lo = params.rsi_oversold.unwrap_or(30.0);
    let rsi_hi = params.rsi_overbought.unwrap_or(70.0);
    let bb_p = params.bb_period.unwrap_or(20);
    let bb_s = params.bb_sigma.unwrap_or(2.0);
    let obv_p = params.obv_period.unwrap_or(20);
    let sl_v = params.stop_loss_pct.map(|v| v.to_string()).unwrap_or_default();
    let tp_v = params.take_profit_pct.map(|v| v.to_string()).unwrap_or_default();
    let short_checked = if params.allow_short { " checked" } else { "" };
    let period_char = params.period;
    let active = |c: char| if c == period_char { " active" } else { "" };
    let p_d = active('D');
    let p_w = active('W');
    let p_m = active('M');
    // YYYYMMDD → YYYY-MM-DD for HTML date input
    let iso = |s: &str| if s.len() == 8 {
        format!("{}-{}-{}", &s[..4], &s[4..6], &s[6..8])
    } else { s.to_string() };
    let from_v = params.from.as_deref().map(iso).unwrap_or_default();
    let to_v = params.to.as_deref().map(iso).unwrap_or_default();
    let me_v = params.manual_entry_date.as_deref().map(iso).unwrap_or_default();
    let mx_v = params.manual_exit_date.as_deref().map(iso).unwrap_or_default();
    let dir_v = params.manual_direction.clone().unwrap_or_else(|| "long".into());

    format!(r#"<!DOCTYPE html>
<html lang="ko"><head>
<meta charset="utf-8">
<title>[{code}] {name} — backtest</title>
<script src="https://unpkg.com/lightweight-charts@4.2.0/dist/lightweight-charts.standalone.production.js"></script>
<style>
  * {{ box-sizing: border-box; }}
  body {{ margin:0; font-family: -apple-system, system-ui, sans-serif;
         background:#ffffff; color:#1a1a1a; display:flex; height:100vh; }}
  #main {{ flex:1; display:flex; flex-direction:column; min-width:0; }}
  header {{ padding:10px 16px; border-bottom:1px solid #e0e3eb; font-size:13px;
           display:flex; align-items:center; gap:12px; }}
  header .title {{ flex:1; min-width:0; }}
  header h1 {{ margin:0 0 4px 0; font-size:14px; }}
  header .muted {{ color:#787b86; font-size:11px; }}
  .search-box {{ position:relative; }}
  #search {{ background:#ffffff; border:1px solid #d1d4dc; color:#1a1a1a;
            padding:5px 10px; border-radius:3px; font-size:12px; width:220px;
            font-family:inherit; }}
  #search:focus {{ outline:none; border-color:#2962ff; }}
  #searchResults {{ position:absolute; top:calc(100% + 2px); right:0;
                    background:#ffffff; border:1px solid #d1d4dc;
                    border-radius:3px; max-height:320px; overflow-y:auto;
                    display:none; z-index:20; min-width:320px;
                    box-shadow:0 4px 12px rgba(0,0,0,0.08); }}
  .sr {{ padding:7px 12px; cursor:pointer; font-size:12px;
        display:flex; gap:10px; border-bottom:1px solid #e0e3eb;
        align-items:center; }}
  .sr:last-child {{ border-bottom:none; }}
  .sr:hover, .sr.active {{ background:#f0f3fa; }}
  .sr-code {{ color:#2962ff; font-weight:600; min-width:68px;
             font-family:monospace; }}
  .sr-market {{ color:#666; min-width:48px; font-size:10px;
               background:#e0e3eb; padding:2px 6px; border-radius:2px; }}
  .sr-name {{ color:#1a1a1a; flex:1; overflow:hidden;
             text-overflow:ellipsis; white-space:nowrap; }}
  #chart {{ flex:1; min-height:0; position:relative; }}
  .zoom-controls {{ position:absolute; top:10px; left:10px; display:flex;
                    flex-direction:row; gap:4px; z-index:10; }}
  .zoom-btn {{ width:28px; height:28px; background:rgba(255,255,255,0.92);
              border:1px solid #d1d4dc; border-radius:3px; color:#1a1a1a;
              font-size:13px; cursor:pointer; font-family:inherit;
              transition:all 0.1s; padding:0; line-height:26px;
              display:flex; align-items:center; justify-content:center; }}
  .zoom-btn:hover {{ background:#ffffff; border-color:#2962ff; color:#2962ff; }}
  .zoom-btn:active {{ background:#f0f3fa; }}
  #legend {{ position:absolute; top:10px; left:118px; z-index:10; display:flex;
             gap:10px; font-size:11px; background:rgba(255,255,255,0.92);
             padding:4px 8px; border:1px solid #d1d4dc; border-radius:3px; }}
  #legend:empty {{ display:none; }}
  #legend .item {{ display:inline-flex; align-items:center; gap:4px; color:#1a1a1a; }}
  #legend .swatch {{ width:10px; height:2px; display:inline-block; }}
  #equity {{ height:140px; border-top:1px solid #e0e3eb; }}
  aside {{ width:360px; border-left:1px solid #e0e3eb; padding:12px 14px;
          overflow-y:auto; font-size:12px; background:#fafbfc; }}
  aside h2 {{ margin:0 0 8px 0; font-size:12px; color:#666;
             text-transform:uppercase; letter-spacing:0.5px; }}
  .m {{ display:flex; justify-content:space-between; padding:4px 0;
        border-bottom:1px solid #e0e3eb; font-variant-numeric:tabular-nums; }}
  .m .k {{ color:#787b86; }}
  .pos {{ color:#089981; }} .neg {{ color:#f23645; }}
  table {{ width:100%; border-collapse:collapse; margin-top:6px; }}
  th {{ text-align:left; color:#787b86; font-weight:500; font-size:10px;
        padding:4px; border-bottom:1px solid #d1d4dc; }}
  td {{ padding:3px 4px; font-size:11px; font-variant-numeric:tabular-nums;
        border-bottom:1px solid #e0e3eb; }}
  td:first-child {{ width:14px; color:#2962ff; }}
  tr.trade-row {{ cursor:pointer; }}
  tr.trade-row:hover td {{ background:#f0f3fa; }}
  .tag {{ font-size:9px; background:#e0e3eb; color:#666; padding:1px 5px;
         border-radius:2px; margin-left:4px; }}
  .row {{ display:flex; align-items:center; justify-content:space-between;
          padding:4px 0; gap:8px; }}
  .row label {{ color:#666; font-size:11px; flex:0 0 auto; }}
  .row input, .row select {{ background:#ffffff; color:#1a1a1a; border:1px solid #d1d4dc;
                              padding:4px 6px; border-radius:3px; font-size:11px;
                              font-family:inherit; width:110px; text-align:right;
                              font-variant-numeric:tabular-nums; }}
  .row input[type="checkbox"] {{ width:auto; }}
  .row input[type="date"] {{ text-align:left; }}
  .row input:focus, .row select:focus {{ outline:none; border-color:#2962ff; }}
  #apply {{ width:100%; margin-top:10px; padding:8px; background:#2962ff;
            color:#fff; border:none; border-radius:3px; font-size:12px;
            font-weight:600; cursor:pointer; }}
  #apply:hover {{ background:#1e54e5; }}
  #apply:disabled {{ background:#e0e3eb; color:#999; cursor:not-allowed; }}
  .status {{ display:inline-block; margin-left:8px; font-size:10px; color:#787b86; }}
  .period-group {{ display:inline-flex; gap:0; }}
  .period-btn {{ background:#ffffff; color:#666; border:1px solid #d1d4dc;
                padding:4px 12px; cursor:pointer; font-size:11px;
                font-family:inherit; transition:all 0.1s; }}
  .period-btn:first-child {{ border-radius:3px 0 0 3px; }}
  .period-btn:last-child {{ border-radius:0 3px 3px 0; }}
  .period-btn:not(:first-child) {{ border-left:none; }}
  .period-btn.active {{ background:#2962ff; color:#fff; border-color:#2962ff; }}
  .period-btn:hover:not(.active) {{ background:#f0f3fa; color:#1a1a1a; }}
  .open {{ padding:8px 10px; margin:8px 0; background:#f0f3fa;
          border:1px solid #e0e3eb; border-radius:3px; font-size:11px; }}
</style></head>
<body>
<div id="main">
  <header>
    <div class="title">
      <h1 id="symbol-title">[{code}] {name}</h1>
      <div class="muted"><span id="strat-label"></span> · <span id="range"></span></div>
    </div>
    <div class="search-box">
      <input id="search" placeholder="종목 검색 (예: TSLA, 삼성)" autocomplete="off">
      <div id="searchResults"></div>
    </div>
  </header>
  <div id="chart">
    <div class="zoom-controls">
      <button class="zoom-btn" data-zoom="in" title="확대 (+)">+</button>
      <button class="zoom-btn" data-zoom="out" title="축소 (−)">−</button>
      <button class="zoom-btn" data-zoom="fit" title="전체 보기 (0)">⤢</button>
    </div>
    <div id="legend"></div>
  </div>
  <div id="equity"></div>
</div>
<aside>
  <h2>파라미터</h2>
  <div class="row"><label>주기</label>
    <div class="period-group">
      <button type="button" class="period-btn{p_d}" data-period="D">일</button>
      <button type="button" class="period-btn{p_w}" data-period="W">주</button>
      <button type="button" class="period-btn{p_m}" data-period="M">월</button>
    </div>
  </div>
  <div class="row"><label>시작일</label><input id="from" type="date" value="{from_v}"></div>
  <div class="row"><label>종료일</label><input id="to" type="date" value="{to_v}"></div>
  <div class="row"><label>전략</label>
    <select id="strategy">
      <option value="ma-cross">MA Cross</option>
      <option value="rsi">RSI</option>
      <option value="macd">MACD</option>
      <option value="bollinger">Bollinger</option>
      <option value="ichimoku">Ichimoku</option>
      <option value="obv">OBV</option>
      <option value="manual">Manual (고정 진입)</option>
    </select>
  </div>
  <div class="row" data-strat="ma-cross"><label>단기 MA</label><input id="fast" type="number" min="1" value="{fast}"></div>
  <div class="row" data-strat="ma-cross"><label>장기 MA</label><input id="slow" type="number" min="2" value="{slow}"></div>
  <div class="row" data-strat="rsi"><label>RSI 기간</label><input id="rsi_period" type="number" min="2" value="{rsi_p}"></div>
  <div class="row" data-strat="rsi"><label>과매도</label><input id="rsi_oversold" type="number" step="1" value="{rsi_lo}"></div>
  <div class="row" data-strat="rsi"><label>과매수</label><input id="rsi_overbought" type="number" step="1" value="{rsi_hi}"></div>
  <div class="row" data-strat="bollinger"><label>BB 기간</label><input id="bb_period" type="number" min="2" value="{bb_p}"></div>
  <div class="row" data-strat="bollinger"><label>BB σ</label><input id="bb_sigma" type="number" step="0.1" value="{bb_s}"></div>
  <div class="row" data-strat="obv"><label>OBV 기간</label><input id="obv_period" type="number" min="2" value="{obv_p}"></div>
  <div class="row" data-strat="manual"><label>진입일</label><input id="manual_entry_date" type="date" value="{me_v}"></div>
  <div class="row" data-strat="manual"><label>청산일</label><input id="manual_exit_date" type="date" placeholder="비우면 끝까지" value="{mx_v}"></div>
  <div class="row" data-strat="manual"><label>방향</label>
    <select id="manual_direction">
      <option value="long">Long (매수)</option>
      <option value="short">Short (매도)</option>
    </select>
  </div>
  <div class="row"><label>수수료 (bps)</label><input id="fee_bps" type="number" step="0.1" value="{fee}"></div>
  <div class="row"><label>슬리피지 (bps)</label><input id="slippage_bps" type="number" step="0.1" value="{slip}"></div>
  <div class="row"><label>레버리지</label><input id="leverage" type="number" step="0.1" min="0.1" value="{lev}"></div>
  <div class="row"><label>숏 허용</label><input id="allow_short" type="checkbox"{short_checked}></div>
  <div class="row"><label>손절 (%)</label><input id="stop_loss_pct" type="number" step="0.5" placeholder="없음" value="{sl_v}"></div>
  <div class="row"><label>익절 (%)</label><input id="take_profit_pct" type="number" step="0.5" placeholder="없음" value="{tp_v}"></div>
  <button id="apply">적용 (Enter)</button>
  <span id="status" class="status"></span>

  <h2 style="margin-top:16px">성과</h2>
  <div class="m"><span class="k">총수익</span><span class="v" id="m-total"></span></div>
  <div class="m"><span class="k">Buy & Hold</span><span class="v" id="m-bnh"></span></div>
  <div class="m"><span class="k">CAGR</span><span class="v" id="m-cagr"></span></div>
  <div class="m"><span class="k">MDD</span><span class="v" id="m-mdd"></span></div>
  <div class="m"><span class="k">Sharpe</span><span class="v" id="m-sharpe"></span></div>
  <div class="m"><span class="k">Sortino</span><span class="v" id="m-sortino"></span></div>
  <div class="m"><span class="k">Calmar</span><span class="v" id="m-calmar"></span></div>

  <h2 style="margin-top:14px">트레이드</h2>
  <div class="m"><span class="k">총 건수</span><span class="v" id="m-ntrades"></span></div>
  <div class="m"><span class="k">승률</span><span class="v" id="m-winrate"></span></div>
  <div class="m"><span class="k">Profit Factor</span><span class="v" id="m-pf"></span></div>
  <div class="open" id="open-pos" style="display:none"></div>
  <table>
    <thead><tr><th></th><th>진입</th><th>청산</th><th>손익</th></tr></thead>
    <tbody id="trades-body"></tbody>
  </table>
</aside>
<script>
const INITIAL_PAYLOAD = {payload};
const INITIAL_STRATEGY = '{strat_str}';
const INITIAL_DIRECTION = '{dir_v}';

const state = {{ overlaySeries: [] }};

function init() {{
  state.priceChart = LightweightCharts.createChart(document.getElementById('chart'), {{
    layout: {{ background: {{ type:'solid', color:'#ffffff' }}, textColor:'#1a1a1a' }},
    grid: {{ vertLines:{{color:'#e0e3eb'}}, horzLines:{{color:'#e0e3eb'}} }},
    rightPriceScale: {{ borderColor:'#d1d4dc' }},
    timeScale: {{ borderColor:'#d1d4dc' }},
    crosshair: {{ mode: LightweightCharts.CrosshairMode.Normal }},
  }});
  state.candleSeries = state.priceChart.addCandlestickSeries({{
    upColor:'#089981', downColor:'#f23645',
    borderUpColor:'#089981', borderDownColor:'#f23645',
    wickUpColor:'#089981', wickDownColor:'#f23645',
  }});

  state.eqChart = LightweightCharts.createChart(document.getElementById('equity'), {{
    layout: {{ background: {{ type:'solid', color:'#ffffff' }}, textColor:'#787b86' }},
    grid: {{ vertLines:{{color:'#f0f3fa'}}, horzLines:{{color:'#f0f3fa'}} }},
    rightPriceScale: {{ borderColor:'#d1d4dc' }},
    timeScale: {{ borderColor:'#d1d4dc', visible:false }},
  }});
  state.eqLine = state.eqChart.addLineSeries({{ color:'#2962ff', lineWidth:2,
    title:'Equity', priceLineVisible:false, lastValueVisible:true }});

  sync(state.priceChart, state.eqChart);
  sync(state.eqChart, state.priceChart);

  document.getElementById('strategy').value = INITIAL_STRATEGY;
  document.getElementById('manual_direction').value = INITIAL_DIRECTION;
  document.getElementById('strategy').addEventListener('change', updateStrategyFields);
  document.getElementById('apply').addEventListener('click', apply);
  document.querySelectorAll('input, select').forEach(el => {{
    el.addEventListener('keydown', e => {{ if (e.key === 'Enter') apply(); }});
  }});
  document.querySelectorAll('.period-btn').forEach(btn => {{
    btn.addEventListener('click', () => {{
      document.querySelectorAll('.period-btn').forEach(b =>
        b.classList.toggle('active', b === btn));
      apply();
    }});
  }});
  document.querySelectorAll('.zoom-btn').forEach(btn => {{
    btn.addEventListener('click', () => {{
      const action = btn.dataset.zoom;
      if (action === 'in') zoomBy(0.6);
      else if (action === 'out') zoomBy(1.6);
      else state.priceChart.timeScale().fitContent();
    }});
  }});
  // 단축키: + 확대, - 축소, 0 전체보기, ←/→ 좌우 이동 (입력 필드 포커스 시 무시)
  document.addEventListener('keydown', e => {{
    if (e.target.matches('input, select, textarea')) return;
    if (e.key === '+' || e.key === '=') {{ e.preventDefault(); zoomBy(0.6); }}
    else if (e.key === '-' || e.key === '_') {{ e.preventDefault(); zoomBy(1.6); }}
    else if (e.key === '0') {{ e.preventDefault(); state.priceChart.timeScale().fitContent(); }}
    else if (e.key === 'ArrowRight') {{ e.preventDefault(); panBy(0.2); }}
    else if (e.key === 'ArrowLeft') {{ e.preventDefault(); panBy(-0.2); }}
  }});

  updateStrategyFields();
  onBacktestData(INITIAL_PAYLOAD);
  state.priceChart.timeScale().fitContent();
  state.eqChart.timeScale().fitContent();
}}

function onBacktestData(p) {{
  state.candleSeries.setData(p.candles);
  state.baseMarkers = p.markers || [];
  state.candleSeries.setMarkers(state.baseMarkers);
  if (state.hlTimer) {{ clearTimeout(state.hlTimer); state.hlTimer = null; }}

  for (const s of state.overlaySeries) state.priceChart.removeSeries(s);
  state.overlaySeries = [];
  // title 을 의도적으로 생략: v4.2.0 은 lastValueVisible:false 이어도 title pill 을 표시해
  // 우측 가격 스케일을 넘어 잘리는 이슈가 있음. 대신 legend 로 표시.
  for (const ov of (p.overlays || [])) {{
    const line = state.priceChart.addLineSeries({{
      color: ov.color, lineWidth: 1,
      priceLineVisible: false, lastValueVisible: false,
    }});
    line.setData(ov.data);
    state.overlaySeries.push(line);
  }}
  renderLegend(p.overlays || []);

  state.eqLine.setData(p.equity || []);

  updateMetrics(p.report);
  updateTrades(p.report.trades, p.report.open_position);

  document.getElementById('strat-label').textContent = p.strategy_label;
  document.getElementById('range').textContent =
    p.report.from + ' – ' + p.report.to + ' · ' + p.report.bars + '봉/' + p.report.period;
  document.getElementById('status').textContent = '';
  document.getElementById('apply').disabled = false;
}}

function onBacktestError(msg) {{
  document.getElementById('status').textContent = '에러: ' + msg;
  document.getElementById('apply').disabled = false;
}}

function renderLegend(overlays) {{
  const el = document.getElementById('legend');
  if (!el) return;
  el.innerHTML = '';
  for (const ov of overlays) {{
    const item = document.createElement('span');
    item.className = 'item';
    const sw = document.createElement('span');
    sw.className = 'swatch';
    sw.style.backgroundColor = ov.color;
    const txt = document.createElement('span');
    txt.textContent = ov.name;
    item.appendChild(sw);
    item.appendChild(txt);
    el.appendChild(item);
  }}
}}

window.onBacktestData = onBacktestData;
window.onBacktestError = onBacktestError;

function fmtPct(v) {{
  if (v === null || v === undefined || isNaN(v)) return '-';
  return (v >= 0 ? '+' : '') + v.toFixed(2) + '%';
}}
function setCell(id, text, cls) {{
  const el = document.getElementById(id);
  el.textContent = text;
  el.className = 'v' + (cls ? ' ' + cls : '');
}}

function updateMetrics(r) {{
  setCell('m-total', fmtPct(r.total_return_pct), r.total_return_pct >= 0 ? 'pos' : 'neg');
  setCell('m-bnh', fmtPct(r.buy_and_hold_pct), r.buy_and_hold_pct >= 0 ? 'pos' : 'neg');
  setCell('m-cagr', fmtPct(r.cagr_pct));
  setCell('m-mdd', '-' + r.max_drawdown_pct.toFixed(2) + '%', 'neg');
  setCell('m-sharpe', r.sharpe.toFixed(2));
  setCell('m-sortino', r.sortino.toFixed(2));
  setCell('m-calmar', r.calmar.toFixed(2));
  setCell('m-ntrades', r.n_trades);
  setCell('m-winrate', r.win_rate_pct.toFixed(1) + '%');
  setCell('m-pf', r.profit_factor.toFixed(2));
}}

function updateTrades(trades, openPos) {{
  const rows = (trades || []).slice().reverse().slice(0, 80).map(t => {{
    const dir = t.direction > 0 ? 'L' : 'S';
    const cls = t.pnl_pct >= 0 ? 'pos' : 'neg';
    const tag = t.exit_reason === 'signal' ? '' :
      ' <span class="tag">' + t.exit_reason + '</span>';
    return '<tr class="trade-row" data-entry="' + t.entry_date +
      '" data-exit="' + t.exit_date + '" title="클릭해서 차트 이동"><td>' + dir +
      '</td><td>' + t.entry_date + '</td><td>' + t.exit_date +
      '</td><td class="' + cls + '">' + fmtPct(t.pnl_pct) + tag + '</td></tr>';
  }});
  document.getElementById('trades-body').innerHTML = rows.join('');
  document.querySelectorAll('#trades-body tr.trade-row').forEach(r => {{
    r.addEventListener('click', () => jumpToTrade(r.dataset.entry, r.dataset.exit));
  }});

  const openEl = document.getElementById('open-pos');
  if (openPos) {{
    const cls = openPos.unrealized_pct >= 0 ? 'pos' : 'neg';
    const dir = openPos.direction > 0 ? 'long' : 'short';
    openEl.innerHTML = '미청산 (' + dir + '): ' + openPos.entry_date +
      ' @ ' + openPos.entry_price + ' <span class="' + cls + '">' +
      fmtPct(openPos.unrealized_pct) + '</span>';
    openEl.style.display = '';
  }} else {{
    openEl.style.display = 'none';
  }}
}}

function getOpt(id) {{
  const v = document.getElementById(id).value.trim();
  return v === '' ? null : parseFloat(v);
}}
function getOptInt(id) {{
  const v = document.getElementById(id).value.trim();
  return v === '' ? null : parseInt(v, 10);
}}

function gatherParams() {{
  const pBtn = document.querySelector('.period-btn.active');
  const fromV = document.getElementById('from').value.trim();
  const toV = document.getElementById('to').value.trim();
  return {{
    strategy: document.getElementById('strategy').value,
    period: pBtn ? pBtn.dataset.period : 'D',
    from: fromV === '' ? null : fromV,
    to: toV === '' ? null : toV,
    fee_bps: parseFloat(document.getElementById('fee_bps').value) || 0,
    slippage_bps: parseFloat(document.getElementById('slippage_bps').value) || 0,
    allow_short: document.getElementById('allow_short').checked,
    leverage: parseFloat(document.getElementById('leverage').value) || 1,
    stop_loss_pct: getOpt('stop_loss_pct'),
    take_profit_pct: getOpt('take_profit_pct'),
    fast: getOptInt('fast'),
    slow: getOptInt('slow'),
    rsi_period: getOptInt('rsi_period'),
    rsi_oversold: getOpt('rsi_oversold'),
    rsi_overbought: getOpt('rsi_overbought'),
    bb_period: getOptInt('bb_period'),
    bb_sigma: getOpt('bb_sigma'),
    obv_period: getOptInt('obv_period'),
    manual_entry_date: getStr('manual_entry_date'),
    manual_exit_date: getStr('manual_exit_date'),
    manual_direction: document.getElementById('manual_direction').value,
  }};
}}

function getStr(id) {{
  const v = document.getElementById(id).value.trim();
  return v === '' ? null : v;
}}

function apply() {{
  document.getElementById('status').textContent = '계산 중...';
  document.getElementById('apply').disabled = true;
  const params = gatherParams();
  if (window.ipc && window.ipc.postMessage) {{
    window.ipc.postMessage(JSON.stringify({{ type: 'run', params: params }}));
  }} else {{
    onBacktestError('IPC 미지원 (브라우저 모드)');
  }}
}}

function updateStrategyFields() {{
  const s = document.getElementById('strategy').value;
  document.querySelectorAll('[data-strat]').forEach(el => {{
    const ss = el.dataset.strat.split(',');
    el.style.display = ss.includes(s) ? '' : 'none';
  }});
}}

function sync(src, dst) {{
  src.timeScale().subscribeVisibleLogicalRangeChange(r => {{
    if (r) dst.timeScale().setVisibleLogicalRange(r);
  }});
}}

function zoomBy(factor) {{
  const range = state.priceChart.timeScale().getVisibleLogicalRange();
  if (!range) return;
  const center = (range.from + range.to) / 2;
  const half = (range.to - range.from) / 2 * factor;
  state.priceChart.timeScale().setVisibleLogicalRange({{
    from: center - half,
    to: center + half,
  }});
}}

function panBy(factor) {{
  const range = state.priceChart.timeScale().getVisibleLogicalRange();
  if (!range) return;
  const delta = (range.to - range.from) * factor;
  state.priceChart.timeScale().setVisibleLogicalRange({{
    from: range.from + delta,
    to: range.to + delta,
  }});
}}

function toIsoDate(s) {{
  if (!s) return s;
  if (s.length === 8 && /^\d+$/.test(s)) {{
    return s.slice(0, 4) + '-' + s.slice(4, 6) + '-' + s.slice(6, 8);
  }}
  return s;
}}

function addDays(iso, n) {{
  const d = new Date(iso + 'T00:00:00Z');
  d.setUTCDate(d.getUTCDate() + n);
  return d.toISOString().slice(0, 10);
}}

function jumpToTrade(entry, exit) {{
  const e = toIsoDate(entry);
  const x = toIsoDate(exit) || e;
  const span = Math.max(
    (new Date(x) - new Date(e)) / 86400000,
    1
  );
  const pad = Math.max(Math.round(span * 0.6), 10);
  const from = addDays(e, -pad);
  const to = addDays(x, pad);
  state.priceChart.timeScale().setVisibleRange({{ from, to }});
  highlightTrade(e, x);
}}

function highlightTrade(entryIso, exitIso) {{
  const base = state.baseMarkers || [];
  const hl = [];
  if (entryIso) {{
    hl.push({{ time: entryIso, position: 'aboveBar',
      color: '#2962ff', shape: 'circle', text: '◉', size: 2 }});
  }}
  if (exitIso && exitIso !== entryIso) {{
    hl.push({{ time: exitIso, position: 'aboveBar',
      color: '#ff9f1c', shape: 'circle', text: '◉', size: 2 }});
  }}
  const merged = [...base, ...hl].sort((a, b) =>
    a.time < b.time ? -1 : a.time > b.time ? 1 : 0);
  state.candleSeries.setMarkers(merged);
  if (state.hlTimer) clearTimeout(state.hlTimer);
  state.hlTimer = setTimeout(() => {{
    state.candleSeries.setMarkers(state.baseMarkers || []);
    state.hlTimer = null;
  }}, 3000);
}}

window.addEventListener('resize', () => {{
  state.priceChart.applyOptions({{ width: document.getElementById('chart').clientWidth }});
  state.eqChart.applyOptions({{ width: document.getElementById('equity').clientWidth }});
}});

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
    if (window.ipc && window.ipc.postMessage) {{
      window.ipc.postMessage(JSON.stringify({{ type: 'search', query: q }}));
    }}
  }}, 180);
}});

_searchInput.addEventListener('keydown', (e) => {{
  if (_searchItems.length === 0 && e.key !== 'Escape') return;
  if (e.key === 'ArrowDown') {{
    e.preventDefault();
    _searchIdx = Math.min(_searchIdx + 1, _searchItems.length - 1);
    highlightSearch();
  }} else if (e.key === 'ArrowUp') {{
    e.preventDefault();
    _searchIdx = Math.max(_searchIdx - 1, 0);
    highlightSearch();
  }} else if (e.key === 'Enter') {{
    e.preventDefault();
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

window.onBacktestSearchResults = function(items) {{
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
  document.getElementById('apply').disabled = true;
  const params = gatherParams();
  if (window.ipc && window.ipc.postMessage) {{
    window.ipc.postMessage(JSON.stringify({{
      type: 'select', code: code, market: market, params: params,
    }}));
  }} else {{
    onBacktestError('IPC 미지원 (브라우저 모드)');
  }}
}}

function escapeHtml(s) {{
  return String(s).replace(/[&<>"']/g, c => ({{
    '&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;','\'':'&#39;'
  }}[c]));
}}

// payload.meta 가 오면 헤더/제목 갱신
const _origOnBacktestData = window.onBacktestData;
window.onBacktestData = function(p) {{
  _origOnBacktestData(p);
  if (p && p.meta) {{
    const h = document.getElementById('symbol-title');
    if (h) h.textContent = `[${{p.meta.symbol}}] ${{p.meta.name}}`;
    document.title = `[${{p.meta.symbol}}] ${{p.meta.name}} — backtest`;
  }}
}};

init();
</script>
</body></html>"#,
        code = esc(code),
        name = esc(name),
        strat_str = strat_str,
        fast = fast,
        slow = slow,
        rsi_p = rsi_p,
        rsi_lo = rsi_lo,
        rsi_hi = rsi_hi,
        bb_p = bb_p,
        bb_s = bb_s,
        obv_p = obv_p,
        fee = params.fee_bps,
        slip = params.slippage_bps,
        lev = params.leverage,
        short_checked = short_checked,
        sl_v = sl_v,
        tp_v = tp_v,
        me_v = me_v,
        mx_v = mx_v,
        dir_v = dir_v,
        payload = initial_payload,
    )
}

fn to_iso(d: &str) -> String {
    if d.len() == 8 {
        format!("{}-{}-{}", &d[..4], &d[4..6], &d[6..8])
    } else {
        d.to_string()
    }
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn candles_json(s: &Series) -> String {
    let mut out = String::from("[");
    let mut first = true;
    for i in 0..s.closes.len() {
        if s.closes[i].is_nan() {
            continue;
        }
        if !first {
            out.push(',');
        }
        first = false;
        out.push_str(&format!(
            r#"{{"time":"{}","open":{},"high":{},"low":{},"close":{}}}"#,
            to_iso(&s.dates[i]),
            nz(s.open[i]),
            nz(s.high[i]),
            nz(s.low[i]),
            s.closes[i],
        ));
    }
    out.push(']');
    out
}

fn nz(v: f64) -> String {
    if v.is_nan() { "null".into() } else { format!("{}", v) }
}

fn markers_json(trades: &[Trade], open_position: Option<&OpenPosition>, _s: &Series) -> String {
    let mut out = String::from("[");
    let mut first = true;
    for t in trades {
        for (date, is_entry) in [(&t.entry_date, true), (&t.exit_date, false)] {
            if !first {
                out.push(',');
            }
            first = false;
            let (pos, color, shape, text) = if is_entry {
                if t.direction > 0 {
                    ("belowBar", "#089981", "arrowUp", "LONG")
                } else {
                    ("aboveBar", "#f23645", "arrowDown", "SHORT")
                }
            } else {
                let reason = match t.exit_reason.as_str() {
                    "stop-loss" => "SL",
                    "take-profit" => "TP",
                    "liquidated" => "LIQ",
                    _ => "EXIT",
                };
                if t.direction > 0 {
                    ("aboveBar", "#787b86", "square", reason)
                } else {
                    ("belowBar", "#787b86", "square", reason)
                }
            };
            out.push_str(&format!(
                r#"{{"time":"{}","position":"{}","color":"{}","shape":"{}","text":"{}"}}"#,
                to_iso(date),
                pos,
                color,
                shape,
                text,
            ));
        }
    }
    // 미청산 포지션 진입 마커 (청산 마커는 없음, 아직 열려있으므로)
    if let Some(op) = open_position {
        if !first {
            out.push(',');
        }
        let (pos, color, shape, text) = if op.direction > 0 {
            ("belowBar", "#089981", "arrowUp", "LONG ●")
        } else {
            ("aboveBar", "#f23645", "arrowDown", "SHORT ●")
        };
        out.push_str(&format!(
            r#"{{"time":"{}","position":"{}","color":"{}","shape":"{}","text":"{}"}}"#,
            to_iso(&op.entry_date),
            pos,
            color,
            shape,
            text,
        ));
    }
    out.push(']');
    out
}

fn overlays_json(s: &Series, strategy: &Strategy) -> String {
    let mut lines: Vec<(String, &'static str, Vec<f64>)> = Vec::new();
    match strategy {
        Strategy::MaCross { fast, slow } => {
            lines.push((format!("MA{fast}"), "#a678f0", sma(&s.closes, *fast)));
            lines.push((format!("MA{slow}"), "#f4a62a", sma(&s.closes, *slow)));
        }
        Strategy::Bollinger { period, sigma } => {
            let b = bollinger(&s.closes, *period, *sigma);
            lines.push(("BB Upper".into(), "#2962ff", b.upper));
            lines.push(("BB Middle".into(), "#787b86", b.middle));
            lines.push(("BB Lower".into(), "#2962ff", b.lower));
        }
        Strategy::Ichimoku => {
            let ic = ichimoku(&s.high, &s.low, &s.closes);
            lines.push(("전환선".into(), "#f23645", ic.tenkan));
            lines.push(("기준선".into(), "#2962ff", ic.kijun));
        }
        Strategy::Rsi { .. }
        | Strategy::Macd
        | Strategy::Obv { .. }
        | Strategy::Manual { .. } => {}
    }

    let mut out = String::from("[");
    let mut first = true;
    for (name, color, vals) in &lines {
        if !first {
            out.push(',');
        }
        first = false;
        let mut data = String::from("[");
        let mut inner_first = true;
        for i in 0..s.closes.len() {
            if i >= vals.len() || vals[i].is_nan() {
                continue;
            }
            if !inner_first {
                data.push(',');
            }
            inner_first = false;
            data.push_str(&format!(
                r#"{{"time":"{}","value":{}}}"#,
                to_iso(&s.dates[i]),
                vals[i]
            ));
        }
        data.push(']');
        out.push_str(&format!(
            r#"{{"name":"{}","color":"{}","data":{}}}"#,
            esc(name),
            color,
            data
        ));
    }
    out.push(']');
    out
}

fn equity_curve_json(s: &Series, equity: &[f64]) -> String {
    let mut out = String::from("[");
    let mut first = true;
    for i in 0..equity.len().min(s.dates.len()) {
        if !first {
            out.push(',');
        }
        first = false;
        out.push_str(&format!(
            r#"{{"time":"{}","value":{}}}"#,
            to_iso(&s.dates[i]),
            equity[i]
        ));
    }
    out.push(']');
    out
}
