//! 데이트레이드용 봉 주기 enum. 분봉(1/5/10/30/60) 지원.

use std::str::FromStr;

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Period {
    Min(u8),
}

impl Period {
    pub fn minutes(self) -> u32 {
        let Period::Min(n) = self;
        n as u32
    }

    pub fn label(self) -> String {
        format!("{}m", self.minutes())
    }
}

impl FromStr for Period {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim().to_lowercase();
        let rest = s
            .strip_suffix('m')
            .ok_or_else(|| anyhow::anyhow!("분봉 포맷 필요 (예: '1m', '5m')"))?;
        let n: u8 = rest
            .parse()
            .map_err(|_| anyhow::anyhow!("분봉 파싱 실패: '{s}'"))?;
        match n {
            1 | 5 | 10 | 30 | 60 => Ok(Period::Min(n)),
            _ => bail!("지원 분봉: 1m / 5m / 10m / 30m / 60m"),
        }
    }
}
