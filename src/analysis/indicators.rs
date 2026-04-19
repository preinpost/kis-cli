//! 기술적 지표 수식.
//!
//! 입력 시계열은 **시간 오름차순**(오래된 → 최신)이어야 한다.
//! 반환 Vec의 인덱스도 동일 정렬. 초기 N-1 개는 NaN (계산 불가 구간).

/// 단순이동평균 (SMA).
pub fn sma(values: &[f64], period: usize) -> Vec<f64> {
    if period == 0 || values.len() < period {
        return vec![f64::NAN; values.len()];
    }
    let mut out = vec![f64::NAN; values.len()];
    let mut sum: f64 = values[..period].iter().sum();
    out[period - 1] = sum / period as f64;
    for i in period..values.len() {
        sum += values[i] - values[i - period];
        out[i] = sum / period as f64;
    }
    out
}

/// 지수이동평균 (EMA). 초기값은 처음 `period`개의 SMA로 seed.
pub fn ema(values: &[f64], period: usize) -> Vec<f64> {
    if period == 0 || values.len() < period {
        return vec![f64::NAN; values.len()];
    }
    let k = 2.0 / (period as f64 + 1.0);
    let mut out = vec![f64::NAN; values.len()];
    let seed: f64 = values[..period].iter().sum::<f64>() / period as f64;
    out[period - 1] = seed;
    for i in period..values.len() {
        out[i] = values[i] * k + out[i - 1] * (1.0 - k);
    }
    out
}

/// RSI — Wilder's smoothing (SMMA).
pub fn rsi(closes: &[f64], period: usize) -> Vec<f64> {
    let n = closes.len();
    let mut out = vec![f64::NAN; n];
    if n <= period {
        return out;
    }
    let mut gains = 0.0;
    let mut losses = 0.0;
    for i in 1..=period {
        let d = closes[i] - closes[i - 1];
        if d >= 0.0 {
            gains += d;
        } else {
            losses += -d;
        }
    }
    let mut avg_gain = gains / period as f64;
    let mut avg_loss = losses / period as f64;
    out[period] = rsi_value(avg_gain, avg_loss);
    for i in (period + 1)..n {
        let d = closes[i] - closes[i - 1];
        let (g, l) = if d >= 0.0 { (d, 0.0) } else { (0.0, -d) };
        avg_gain = (avg_gain * (period as f64 - 1.0) + g) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + l) / period as f64;
        out[i] = rsi_value(avg_gain, avg_loss);
    }
    out
}

fn rsi_value(gain: f64, loss: f64) -> f64 {
    if loss < f64::EPSILON {
        return 100.0;
    }
    let rs = gain / loss;
    100.0 - 100.0 / (1.0 + rs)
}

#[derive(Debug, Clone)]
pub struct Macd {
    pub macd: Vec<f64>,
    pub signal: Vec<f64>,
    pub histogram: Vec<f64>,
}

/// MACD (fast=12, slow=26, signal=9).
pub fn macd(closes: &[f64], fast: usize, slow: usize, signal: usize) -> Macd {
    let ema_fast = ema(closes, fast);
    let ema_slow = ema(closes, slow);
    let macd_line: Vec<f64> = ema_fast
        .iter()
        .zip(ema_slow.iter())
        .map(|(f, s)| if f.is_nan() || s.is_nan() { f64::NAN } else { f - s })
        .collect();
    // signal은 MACD line의 EMA — NaN 뒷부분만 골라 EMA 먹인다.
    let valid_start = macd_line.iter().position(|v| !v.is_nan()).unwrap_or(macd_line.len());
    let macd_valid: Vec<f64> = macd_line[valid_start..].to_vec();
    let sig_valid = ema(&macd_valid, signal);
    let mut signal_line = vec![f64::NAN; macd_line.len()];
    for (i, v) in sig_valid.iter().enumerate() {
        signal_line[valid_start + i] = *v;
    }
    let hist: Vec<f64> = macd_line
        .iter()
        .zip(signal_line.iter())
        .map(|(m, s)| if m.is_nan() || s.is_nan() { f64::NAN } else { m - s })
        .collect();
    Macd { macd: macd_line, signal: signal_line, histogram: hist }
}

#[derive(Debug, Clone)]
pub struct Bollinger {
    pub upper: Vec<f64>,
    pub middle: Vec<f64>,
    pub lower: Vec<f64>,
}

/// 볼린저밴드 (기본 20, 2σ).
pub fn bollinger(closes: &[f64], period: usize, sigma: f64) -> Bollinger {
    let middle = sma(closes, period);
    let n = closes.len();
    let mut upper = vec![f64::NAN; n];
    let mut lower = vec![f64::NAN; n];
    for i in (period - 1)..n {
        if i + 1 < period { continue; }
        let slice = &closes[i + 1 - period..=i];
        let mean = middle[i];
        if mean.is_nan() { continue; }
        let var: f64 = slice.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / period as f64;
        let std = var.sqrt();
        upper[i] = mean + sigma * std;
        lower[i] = mean - sigma * std;
    }
    Bollinger { upper, middle, lower }
}

#[derive(Debug, Clone)]
pub struct Ichimoku {
    /// 전환선 (9기간 고저 중간)
    pub tenkan: Vec<f64>,
    /// 기준선 (26기간 고저 중간)
    pub kijun: Vec<f64>,
    /// 선행스팬1 ((전환+기준)/2, 26 앞으로 이동)
    pub senkou_a: Vec<f64>,
    /// 선행스팬2 (52기간 고저 중간, 26 앞으로 이동)
    pub senkou_b: Vec<f64>,
    /// 후행스팬 (종가, 26 뒤로 이동)
    pub chikou: Vec<f64>,
}

/// 일목균형표 (전환 9, 기준 26, 선행스팬2 52, shift 26).
///
/// 선행스팬 1/2는 인덱스 `i+26` 위치에 저장됨 (미래 구름). 길이가 `closes.len()+26`까지 확장.
pub fn ichimoku(high: &[f64], low: &[f64], close: &[f64]) -> Ichimoku {
    let tenkan_p = 9usize;
    let kijun_p = 26usize;
    let senkou_b_p = 52usize;
    let shift = 26usize;
    let n = close.len();
    let tenkan = hl_midpoint(high, low, tenkan_p);
    let kijun = hl_midpoint(high, low, kijun_p);
    let senkou_b_raw = hl_midpoint(high, low, senkou_b_p);

    // senkou_a/b를 앞으로 26 shift → 벡터 길이 n+shift
    let mut senkou_a = vec![f64::NAN; n + shift];
    let mut senkou_b = vec![f64::NAN; n + shift];
    for i in 0..n {
        let a = (tenkan[i] + kijun[i]) / 2.0;
        senkou_a[i + shift] = a;
        senkou_b[i + shift] = senkou_b_raw[i];
    }

    // chikou = close를 뒤로 26 shift → 인덱스 i-shift에 종가 저장
    let mut chikou = vec![f64::NAN; n];
    for i in shift..n {
        chikou[i - shift] = close[i];
    }

    Ichimoku { tenkan, kijun, senkou_a, senkou_b, chikou }
}

/// OBV (On-Balance Volume) — 누적 거래량 지표.
///
/// 초기값 0, `close[i] > close[i-1]` 이면 volume[i] 더하고, 하락이면 빼고, 동일이면 유지.
/// `close.len() != volume.len()` 이거나 둘 중 길이 <2 면 전부 NaN.
pub fn obv(close: &[f64], volume: &[f64]) -> Vec<f64> {
    let n = close.len();
    if n < 2 || volume.len() != n {
        return vec![f64::NAN; n];
    }
    let mut out = vec![f64::NAN; n];
    out[0] = 0.0;
    for i in 1..n {
        let prev = out[i - 1];
        let v = volume[i];
        let d = close[i] - close[i - 1];
        out[i] = if d > 0.0 {
            prev + v
        } else if d < 0.0 {
            prev - v
        } else {
            prev
        };
    }
    out
}

fn hl_midpoint(high: &[f64], low: &[f64], period: usize) -> Vec<f64> {
    let n = high.len().min(low.len());
    let mut out = vec![f64::NAN; n];
    if n < period { return out; }
    for i in (period - 1)..n {
        let lo = low[i + 1 - period..=i].iter().cloned().fold(f64::INFINITY, f64::min);
        let hi = high[i + 1 - period..=i].iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        out[i] = (hi + lo) / 2.0;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn sma_basic() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let r = sma(&v, 3);
        assert!(r[0].is_nan());
        assert!(r[1].is_nan());
        assert!(approx_eq(r[2], 2.0, 1e-9));
        assert!(approx_eq(r[3], 3.0, 1e-9));
        assert!(approx_eq(r[4], 4.0, 1e-9));
    }

    #[test]
    fn rsi_increasing_only() {
        // 지속 상승이면 RSI=100
        let v: Vec<f64> = (1..=30).map(|i| i as f64).collect();
        let r = rsi(&v, 14);
        assert!(approx_eq(r[29], 100.0, 1e-6));
    }

    #[test]
    fn bollinger_sanity() {
        let v: Vec<f64> = (1..=25).map(|i| i as f64).collect();
        let b = bollinger(&v, 20, 2.0);
        let last = v.len() - 1;
        assert!(!b.middle[last].is_nan());
        assert!(b.upper[last] > b.middle[last]);
        assert!(b.lower[last] < b.middle[last]);
    }

    #[test]
    fn obv_basic() {
        //            i=0   1    2    3    4
        // close:    10,  11,  11,  10,  12
        // volume:  100, 200, 300, 400, 500
        // delta:    -,   +,   0,   -,   +
        // obv:      0, 200, 200,-200, 300
        let c = vec![10.0, 11.0, 11.0, 10.0, 12.0];
        let v = vec![100.0, 200.0, 300.0, 400.0, 500.0];
        let o = obv(&c, &v);
        assert!(approx_eq(o[0], 0.0, 1e-9));
        assert!(approx_eq(o[1], 200.0, 1e-9));
        assert!(approx_eq(o[2], 200.0, 1e-9));
        assert!(approx_eq(o[3], -200.0, 1e-9));
        assert!(approx_eq(o[4], 300.0, 1e-9));
    }

    #[test]
    fn ichimoku_shape() {
        let n = 60;
        let h: Vec<f64> = (1..=n).map(|i| i as f64 + 1.0).collect();
        let l: Vec<f64> = (1..=n).map(|i| i as f64 - 1.0).collect();
        let c: Vec<f64> = (1..=n).map(|i| i as f64).collect();
        let ic = ichimoku(&h, &l, &c);
        assert_eq!(ic.tenkan.len(), n);
        assert_eq!(ic.senkou_a.len(), n + 26);
        assert!(!ic.tenkan[n - 1].is_nan());
        assert!(!ic.kijun[n - 1].is_nan());
    }
}
