//! 전략 → 매수/매도 신호 계층 (순수 계산).
//!
//! 지표(`analysis::indicators`) 위에 얹히는 "전략 식별 + 신호 생성" 레이어.
//! backtest(과거 시뮬레이션)·signal-watch·daytrade 엔진이 공유한다. 네트워크/표현 없음.
//!
//! - [`StrategyKind`] : 전략 종류(직렬화 가능, clap 은 호출 측 CLI 가 담당)
//! - [`Params`]       : 전략 파라미터 묶음
//! - [`Strategy`]     : 파라미터가 적용된 실행 단위
//! - [`compute_signals`] : 각 봉의 보유 신호(-1/0/+1) 벡터
//! - [`latest_signal`]   : 마지막 봉의 신호만 (signal-watch/daytrade 용)

use crate::analysis::{bollinger, ichimoku, macd, obv, rsi, sma};

/// OHLCV 시계열 (오래된→최신 순). backtest·analyze·daytrade 공용 데이터 컨테이너.
pub struct Series {
    pub dates: Vec<String>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub closes: Vec<f64>,
    pub volume: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StrategyKind {
    MaCross,
    Rsi,
    Macd,
    Bollinger,
    Ichimoku,
    Obv,
    Manual,
    /// 복합 전략 — daytrade.toml 의 `children`+`combinator` 로 정의. backtest 미지원.
    Composite,
}

impl StrategyKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            StrategyKind::MaCross => "ma-cross",
            StrategyKind::Rsi => "rsi",
            StrategyKind::Macd => "macd",
            StrategyKind::Bollinger => "bollinger",
            StrategyKind::Ichimoku => "ichimoku",
            StrategyKind::Obv => "obv",
            StrategyKind::Manual => "manual",
            StrategyKind::Composite => "composite",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "ma-cross" | "macross" => Some(Self::MaCross),
            "rsi" => Some(Self::Rsi),
            "macd" => Some(Self::Macd),
            "bollinger" => Some(Self::Bollinger),
            "ichimoku" => Some(Self::Ichimoku),
            "obv" => Some(Self::Obv),
            "manual" => Some(Self::Manual),
            "composite" => Some(Self::Composite),
            _ => None,
        }
    }

    pub fn is_composite(&self) -> bool {
        matches!(self, StrategyKind::Composite)
    }
}

#[derive(Clone, Debug)]
pub enum Strategy {
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
    pub fn label(&self) -> String {
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

/// 주어진 시리즈·파라미터로 **마지막 봉의 신호** 만 반환. signal-watch / daytrade 가 사용.
/// 반환값: +1 (long 유지 신호), 0 (flat), -1 (short 유지 신호).
pub fn latest_signal(series: &Series, params: &Params) -> i8 {
    let strategy = build_strategy(params);
    let signals = compute_signals(&strategy, series);
    signals.last().copied().unwrap_or(0)
}

pub fn build_strategy(p: &Params) -> Strategy {
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
        StrategyKind::Composite => {
            // Composite는 daytrade 데몬 전용 — backtest에선 해당 entry를 macross 디폴트로 폴백.
            // 실수로 호출되지 않도록 호출처(daytrade.toml/engine)는 별도 분기를 두고 있음.
            Strategy::MaCross { fast: 20, slow: 60 }
        }
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

/// 각 바에서 "바 종가 이후 유지하고 싶은 포지션": -1 short, 0 flat, +1 long.
pub fn compute_signals(strategy: &Strategy, s: &Series) -> Vec<i8> {
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
