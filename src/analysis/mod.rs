//! 기술적 지표 계산 — 캔들 시계열(오래된→최신 순서)을 받아 지표값 Vec 반환.
//!
//! 모든 함수는 순수 — 같은 입력은 같은 결과. 입력 길이가 부족하면 빈 Vec.

pub mod indicators;

pub use indicators::{
    bollinger, ema, ichimoku, macd, rsi, sma, Bollinger, Ichimoku, Macd,
};
