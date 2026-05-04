//! `daytrade.toml` 스키마 — 데몬이 읽는 strategy 리스트.
//!
//! 진입(`add`)할 때 1회 resolve해서 `code`/`market` 을 박아두므로 데몬은 재해석 안 함.
//! 파일은 `~/.config/kis-cli/daytrade.toml`. 없으면 빈 설정.

use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::commands::backtest::StrategyKind;
use crate::config;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DaytradeConfig {
    #[serde(default = "default_schema")]
    pub schema: u32,
    #[serde(default, rename = "strategy")]
    pub strategies: Vec<StrategyEntry>,
}

fn default_schema() -> u32 { 1 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyEntry {
    /// ULID 문자열 (시간순 정렬 가능)
    pub id: String,
    /// "paper" | "run"
    pub mode: ExecMode,
    /// 전략 종류 — `composite` 면 `combinator` + `children` 필수.
    pub kind: StrategyKind,
    /// 사전 resolve 된 KIS 코드 (재해석 우회의 핵심)
    pub code: String,
    /// 사전 resolve 된 시장 ("KOSPI"/"KOSDAQ"/"NASD"/"NYSE"/"AMEX")
    pub market: String,
    #[serde(default)]
    pub display_name: String,
    /// 분봉 주기 ("1m"/"5m"/...)
    pub period: String,
    pub qty: u64,
    pub budget: f64,
    #[serde(default = "default_fee_bps")]
    pub fee_bps: f64,
    /// paper 전용 슬리피지
    #[serde(default)]
    pub slippage_bps: f64,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,
    pub stop_loss_atr: Option<f64>,
    pub take_profit_atr: Option<f64>,
    #[serde(default = "default_atr_period")]
    pub atr_period: usize,
    // 전략별 파라미터 (kind 에 맞는 것만 사용 — composite면 child가 따로 보유)
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
    // composite 전용 — `kind == "composite"` 일 때만 의미.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combinator: Option<Combinator>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "children")]
    pub children: Vec<ChildStrategyEntry>,
    // run 전용
    #[serde(default)]
    pub tick_offset: i32,
    #[serde(default = "default_fill_timeout")]
    pub fill_timeout_secs: u64,
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
}

/// composite의 child — code/qty/budget 같은 종목·자금 메타는 부모가 들고, 여긴 신호 파라미터만.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChildStrategyEntry {
    pub kind: StrategyKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fast: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slow: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsi_period: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsi_oversold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsi_overbought: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bb_period: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bb_sigma: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obv_period: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Combinator {
    /// 모든 child가 +1 → +1 (보수적 진입). 어느 하나가 ≤0 이면 청산.
    And,
    /// 하나라도 +1 → +1. 모두 ≤0 이면 청산.
    Or,
}

impl Combinator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Combinator::And => "and",
            Combinator::Or => "or",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecMode {
    Paper,
    Run,
}

impl ExecMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExecMode::Paper => "paper",
            ExecMode::Run => "run",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "paper" => Some(Self::Paper),
            "run" => Some(Self::Run),
            _ => None,
        }
    }
}

fn default_fee_bps() -> f64 { 5.0 }
fn default_atr_period() -> usize { 14 }
fn default_fill_timeout() -> u64 { 30 }
fn default_poll_interval() -> u64 { 2 }

impl StrategyEntry {
    /// 시장이 해외(NASD/NYSE/AMEX)면 true.
    pub fn is_usa(&self) -> bool {
        matches!(self.market.as_str(), "NASD" | "NASDAQ" | "NYSE" | "AMEX")
    }
}

pub fn config_path() -> Result<PathBuf> {
    config::daytrade_config_path()
}

impl DaytradeConfig {
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("daytrade.toml 읽기 실패: {}", path.display()))?;
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        let cfg: Self = toml::from_str(&text)
            .with_context(|| format!("daytrade.toml 파싱 실패: {}", path.display()))?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // schema 필드를 항상 채워서 저장
        let mut out = self.clone();
        if out.schema == 0 {
            out.schema = default_schema();
        }
        let text = toml::to_string_pretty(&out)
            .context("daytrade.toml 직렬화 실패")?;
        std::fs::write(&path, text)
            .with_context(|| format!("daytrade.toml 쓰기 실패: {}", path.display()))?;
        Ok(())
    }

    pub fn add(&mut self, entry: StrategyEntry) {
        self.strategies.push(entry);
    }

    /// id (전체 또는 substring) 일치 항목을 제거. 매칭 0건 → Err, 다중 → Err.
    /// substring 매칭이라 `short_id` 의 timestamp prefix뿐 아니라 random 끝자락으로도 식별 가능.
    pub fn remove(&mut self, id_or_substring: &str) -> Result<StrategyEntry> {
        let matches: Vec<usize> = self
            .strategies
            .iter()
            .enumerate()
            .filter(|(_, s)| s.id == id_or_substring || s.id.contains(id_or_substring))
            .map(|(i, _)| i)
            .collect();
        match matches.len() {
            0 => Err(anyhow!("'{}' 일치하는 strategy 없음", id_or_substring)),
            1 => Ok(self.strategies.remove(matches[0])),
            n => Err(anyhow!(
                "'{}' 가 {}개 strategy와 일치 — 더 길게/구체적으로 지정 (예: short_id 끝 4자)",
                id_or_substring, n
            )),
        }
    }
}

/// 새 ULID 생성 (소문자 26자).
pub fn new_id() -> String {
    Ulid::new().to_string().to_lowercase()
}

/// (mode, code, market, kind) 조합이 같은 항목이 이미 있는지.
/// 같은 종목+전략+모드를 파라미터만 다르게 둘 수도 있으므로 _경고_용으로만 사용.
pub fn duplicate_summary(cfg: &DaytradeConfig, entry: &StrategyEntry) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    cfg.strategies
        .iter()
        .filter(|s| {
            s.mode == entry.mode
                && s.code == entry.code
                && s.market == entry.market
                && std::mem::discriminant(&s.kind) == std::mem::discriminant(&entry.kind)
                && s.id != entry.id
        })
        .filter_map(|s| {
            let id_short = short_id(&s.id);
            if seen.insert(id_short.clone()) { Some(id_short) } else { None }
        })
        .collect()
}

/// 단일 id 표시용 — ULID 첫 12자 (`add`/`rm` 메시지 등에서 사용).
/// 표 출력엔 [`min_distinguishing_prefix`] 로 git 처럼 충돌 없는 최소 길이를 계산하는 게 좋음.
pub fn short_id(id: &str) -> String {
    id.chars().take(12).collect()
}

/// git short-hash 스타일 — 주어진 id 목록에서 모두 unique 해지는 최소 prefix 길이.
/// 최소 6자(시각적 일관성), 충돌 시 최대 26자(ULID 전체)까지 확장.
pub fn min_distinguishing_prefix(ids: &[&str]) -> usize {
    use std::collections::HashSet;
    let max_len = ids.iter().map(|s| s.chars().count()).max().unwrap_or(0);
    for len in 6..=max_len {
        let mut seen = HashSet::with_capacity(ids.len());
        let mut all_unique = true;
        for id in ids {
            let prefix: String = id.chars().take(len).collect();
            if !seen.insert(prefix) {
                all_unique = false;
                break;
            }
        }
        if all_unique {
            return len;
        }
    }
    max_len.max(6)
}
