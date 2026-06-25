//! 데이트레이드용 봉 주기 enum. 분봉(1/5/10/30/60) + 30초 polling 지원.
//!
//! KIS 분봉 API는 최소 1분봉만 제공하므로, `Sec(30)` 은 1분봉을 30초 주기로 재조회하는
//! polling fallback — 같은 봉을 두 번 관찰해도 종가(현재가)가 업데이트되므로 신호 반응이
//! 빨라진다. aggregate는 1분봉 그대로 사용.

use std::str::FromStr;

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Period {
    Min(u8),
    /// 초 단위 polling 주기. 분봉 fetch는 1분봉으로 고정, tick 주기만 초 단위로 짧아진다.
    Sec(u32),
}

impl Period {
    /// 한 주기의 총 초 수.
    pub fn seconds(self) -> u32 {
        match self {
            Period::Min(n) => n as u32 * 60,
            Period::Sec(n) => n,
        }
    }

    /// KIS 분봉 API `nmin` 파라미터 (분 단위). `Sec` 는 1분봉 fallback.
    pub fn api_nmin(self) -> u32 {
        match self {
            Period::Min(n) => n as u32,
            Period::Sec(_) => 1,
        }
    }

    /// 1분봉 → N분봉 aggregate에서 사용할 step. `Sec` 는 1 (passthrough).
    pub fn aggregate_step_min(self) -> u32 {
        match self {
            Period::Min(n) => n as u32,
            Period::Sec(_) => 1,
        }
    }

    pub fn label(self) -> String {
        match self {
            Period::Min(n) => format!("{}m", n),
            Period::Sec(n) => format!("{}s", n),
        }
    }
}

impl FromStr for Period {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim().to_lowercase();
        if let Some(rest) = s.strip_suffix('s') {
            let n: u32 = rest
                .parse()
                .map_err(|_| anyhow::anyhow!("초 주기 파싱 실패: '{s}'"))?;
            match n {
                30 => Ok(Period::Sec(n)),
                _ => bail!("지원 초 주기: 30s"),
            }
        } else if let Some(rest) = s.strip_suffix('m') {
            let n: u8 = rest
                .parse()
                .map_err(|_| anyhow::anyhow!("분봉 파싱 실패: '{s}'"))?;
            match n {
                1 | 5 | 10 | 30 | 60 => Ok(Period::Min(n)),
                _ => bail!("지원 분봉: 1m / 5m / 10m / 30m / 60m"),
            }
        } else {
            bail!("주기 포맷 필요 (예: '30s', '1m', '5m')")
        }
    }
}
