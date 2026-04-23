mod analysis;
mod api;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod rate_limit;
mod symbols;
mod token;
mod viewer;
mod ws;

use std::io::{self, Write};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::client::KisClient;

#[derive(Parser)]
#[command(name = "kis", about = "한국투자증권 API CLI", disable_version_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 인증 토큰 발급 및 상태 확인
    Auth,

    /// 설정 관리
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// 종목 마스터 관리 (검색/동기화)
    Symbols {
        #[command(subcommand)]
        action: SymbolsAction,
    },

    /// 국내·해외 주식
    Stock {
        #[command(subcommand)]
        action: StockAction,
    },

    /// 장내채권 (모의투자 미지원)
    Bond {
        #[command(subcommand)]
        action: BondAction,
    },

    /// 선물옵션 (국내/해외)
    Fo {
        #[command(subcommand)]
        action: FoAction,
    },

    /// Claude Code 스킬 설치/관리
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },

    /// 최초 설치 — `cargo install` 로 바이너리 배포 + Claude 스킬 설치
    Install {
        /// 이미 설치돼 있을 때 덮어쓰기
        #[arg(long)]
        force: bool,
    },

    /// 버전 확인 — 현재 바이너리 + GitHub 최신 릴리스 비교
    Version,

    /// 업데이트 — 기본은 GitHub Release 에서 현재 플랫폼 바이너리 다운로드 → atomic 교체.
    /// `--from-source` 로 기존 `git pull + cargo install` 경로 사용 (개발 체크아웃용).
    Update {
        /// GitHub Release 대신 로컬 소스에서 재빌드 (개발 체크아웃 전제)
        #[arg(long)]
        from_source: bool,
        /// `--from-source` 와 함께 쓸 때 `git pull` 건너뛰기
        #[arg(long)]
        no_pull: bool,
    },

    /// 자동 손절 — 데몬 실행 / 상태 조회
    StopLoss {
        #[command(subcommand)]
        action: StopLossAction,
    },

    /// 시그널 감시 — 전략별 서브커맨드. cron 스케줄로 신호 체크 후 로그 (주문 없음).
    SignalWatch {
        #[command(subcommand)]
        strategy: SignalWatchStrategy,
    },

    /// 데이트레이드 — 분봉 기반 감시/페이퍼/실주문/백테스트 (국장/미장)
    Daytrade {
        #[command(subcommand)]
        action: DaytradeAction,
    },

    /// 백테스트 — 전략별 서브커맨드. 공통 옵션은 모든 전략이 공유
    Backtest {
        #[command(subcommand)]
        strategy: BacktestStrategy,
    },

    /// 기술적 분석 (MA/RSI/MACD/볼린저/일목균형표)
    Analyze {
        /// 종목명 또는 코드
        symbol: String,
        /// 해외 종목 분석 (기본: 국내)
        #[arg(long)]
        usa: bool,
        /// JSON 덤프 (LLM 해석용)
        #[arg(long)]
        json: bool,
        /// 차트 창 띄우기 (wry 네이티브 뷰어)
        #[arg(long)]
        chart: bool,
        /// HTML 파일 저장 (경로 지정)
        #[arg(long)]
        save: Option<String>,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
}

#[derive(Subcommand)]
enum BacktestStrategy {
    /// 단기·장기 이동평균 교차 (golden/dead cross)
    MaCross {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: BacktestCommonArgs,
        /// 단기 MA 기간
        #[arg(long, default_value_t = 20)]
        fast: usize,
        /// 장기 MA 기간
        #[arg(long, default_value_t = 60)]
        slow: usize,
    },
    /// RSI 과매도/과매수 반전 매매
    Rsi {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: BacktestCommonArgs,
        /// RSI 기간
        #[arg(long, default_value_t = 14)]
        rsi_period: usize,
        /// 과매도 임계 (long 진입)
        #[arg(long, default_value_t = 30.0)]
        rsi_oversold: f64,
        /// 과매수 임계 (short 진입 / long 청산)
        #[arg(long, default_value_t = 70.0)]
        rsi_overbought: f64,
    },
    /// MACD 히스토그램 부호 (12/26/9 고정)
    Macd {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: BacktestCommonArgs,
    },
    /// 볼린저 밴드 평균회귀 (하단 돌파 long, 상단 돌파 short)
    Bollinger {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: BacktestCommonArgs,
        /// 볼린저 기간
        #[arg(long, default_value_t = 20)]
        bb_period: usize,
        /// σ 배수
        #[arg(long, default_value_t = 2.0)]
        bb_sigma: f64,
    },
    /// 일목균형표 (9/26/52 고정, 구름대+전환선/기준선)
    Ichimoku {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: BacktestCommonArgs,
    },
    /// OBV(On-Balance Volume) vs SMA(OBV, N) 크로스오버
    Obv {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: BacktestCommonArgs,
        /// OBV 시그널선 SMA 기간
        #[arg(long, default_value_t = 20)]
        obv_period: usize,
    },
    /// 고정 진입/청산 (수동 백테스트)
    Manual {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: BacktestCommonArgs,
        /// 진입일 (YYYYMMDD 또는 YYYY-MM-DD)
        #[arg(long)]
        entry_date: String,
        /// 청산일 (옵션, 없으면 끝까지 보유)
        #[arg(long)]
        exit_date: Option<String>,
        /// 방향
        #[arg(long, default_value = "long")]
        direction: String,
    },
    /// 차트 뷰어 — 인터랙티브 GUI에서 전략·파라미터 자유롭게 변경
    Chart {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        seed: BacktestChartSeed,
    },
}

#[derive(Subcommand)]
enum SignalWatchStrategy {
    /// 단기·장기 이동평균 교차
    MaCross {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: SignalWatchCommonArgs,
        /// 단기 MA 기간
        #[arg(long, default_value_t = 20)]
        fast: usize,
        /// 장기 MA 기간
        #[arg(long, default_value_t = 60)]
        slow: usize,
    },
    /// RSI 과매도/과매수 반전
    Rsi {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: SignalWatchCommonArgs,
        /// RSI 기간
        #[arg(long, default_value_t = 14)]
        rsi_period: usize,
        /// 과매도 임계
        #[arg(long, default_value_t = 30.0)]
        rsi_oversold: f64,
        /// 과매수 임계
        #[arg(long, default_value_t = 70.0)]
        rsi_overbought: f64,
    },
    /// MACD 히스토그램 부호 (12/26/9 고정)
    Macd {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: SignalWatchCommonArgs,
    },
    /// 볼린저 밴드 평균회귀
    Bollinger {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: SignalWatchCommonArgs,
        /// 볼린저 기간
        #[arg(long, default_value_t = 20)]
        bb_period: usize,
        /// σ 배수
        #[arg(long, default_value_t = 2.0)]
        bb_sigma: f64,
    },
    /// 일목균형표 (9/26/52 고정)
    Ichimoku {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: SignalWatchCommonArgs,
    },
    /// OBV vs SMA(OBV, N) 크로스오버
    Obv {
        /// 종목명 또는 코드
        symbol: String,
        #[command(flatten)]
        common: SignalWatchCommonArgs,
        /// OBV 시그널선 SMA 기간
        #[arg(long, default_value_t = 20)]
        obv_period: usize,
    },
    /// 모든 전략 동시 감시 (전이 시점에만 텔레그램 알림)
    All {
        /// 종목명 또는 코드
        symbol: String,
        /// cron 표현식. 미지정 시 국장 "0 0 9-15 * * Mon-Fri", 미장 "0 0 23,0-5 * * Tue-Sat"
        #[arg(long)]
        cron: Option<String>,
        /// 해외 종목
        #[arg(long)]
        usa: bool,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
        /// /etc/systemd/system 에 unit 생성 + enable --now (루트 필요).
        /// VPS 에서는 sudo PATH 제약 때문에 반드시 `sudo $(which kis) signal-watch all ... --background` 형태로 실행 (macOS 는 unit 파일만 출력)
        #[arg(long)]
        background: bool,
    },
    /// `--background` 로 등록된 서비스 목록 출력
    List,
    /// `--background` 로 등록된 서비스 중지 + 제거 (루트 필요)
    Remove {
        /// 종목코드 (예: 005930) 또는 전체 서비스명 (예: kis-signal-watch-005930)
        target: String,
        /// 해외 서비스 (코드만 줄 때 `-usa` 접미사 추가)
        #[arg(long)]
        usa: bool,
    },
}

#[derive(Args)]
struct SignalWatchCommonArgs {
    /// cron 표현식 (6필드: 초 분 시 일 월 요일). 기본: 평일 15:20 (장 마감 10분 전)
    #[arg(long, default_value = "0 20 15 * * Mon-Fri")]
    cron: String,
    /// 봉 주기 (D/W/M)
    #[arg(long, default_value = "D")]
    period: String,
    /// 해외 종목 (기본: 국내). 미장 감시 시 cron 은 KST 기준 미국 장중 시각으로 직접 지정해야 함
    #[arg(long)]
    usa: bool,
    /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
    #[arg(long)]
    pick: Option<usize>,
}

#[derive(Subcommand)]
enum DaytradeAction {
    /// 분봉 신호 감시 — 주문 없음. Phase 1.
    SignalWatch {
        #[command(subcommand)]
        strategy: DaytradeStrategy,
    },
    /// 가상 매매 (페이퍼 트레이딩) — 실주문 없음, SQLite 매매 기록. 장 마감 10분 전 강제 청산.
    Paper {
        #[command(subcommand)]
        strategy: DaytradePaperStrategy,
    },
    /// 실주문 매매 — KIS 계좌에서 실제 주문 발주. `paper` 와 동일한 전략·청산 로직 + 지정가 체결.
    Run {
        #[command(subcommand)]
        strategy: DaytradeRunStrategy,
    },
    /// `--background` 로 등록된 daytrade 서비스 목록 출력
    List,
    /// `--background` 서비스 중지 + 제거 (루트 필요). target: 전체 서비스명 또는 고유 부분 문자열
    Remove {
        /// 서비스명 (kis-daytrade-...) 또는 일부 (예: TSLA)
        target: String,
    },
    /// 체결 기록 조회 — SQLite에 쌓인 paper/run 매매 내역 검색
    History {
        /// 특정 세션의 체결 내역
        #[arg(long)]
        session: Option<String>,
        /// 종목 필터 (코드 일치)
        #[arg(long)]
        symbol: Option<String>,
        /// 오늘 체결만
        #[arg(long)]
        today: bool,
        /// 최근 N일
        #[arg(long)]
        days: Option<u32>,
        /// 세션 요약 최대 개수 (기본 10)
        #[arg(long, default_value_t = 10)]
        limit: usize,
        /// JSON 덤프
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum DaytradePaperStrategy {
    /// 단기·장기 이동평균 교차 (분봉, 페이퍼)
    MaCross {
        symbol: String,
        #[command(flatten)]
        common: DaytradePaperCommonArgs,
        #[arg(long, default_value_t = 20)]
        fast: usize,
        #[arg(long, default_value_t = 60)]
        slow: usize,
    },
    /// RSI 과매도/과매수 반전 (분봉, 페이퍼)
    Rsi {
        symbol: String,
        #[command(flatten)]
        common: DaytradePaperCommonArgs,
        #[arg(long, default_value_t = 14)]
        rsi_period: usize,
        #[arg(long, default_value_t = 30.0)]
        rsi_oversold: f64,
        #[arg(long, default_value_t = 70.0)]
        rsi_overbought: f64,
    },
    /// MACD 히스토그램 부호 (분봉, 페이퍼)
    Macd {
        symbol: String,
        #[command(flatten)]
        common: DaytradePaperCommonArgs,
    },
    /// 볼린저 밴드 평균회귀 (분봉, 페이퍼)
    Bollinger {
        symbol: String,
        #[command(flatten)]
        common: DaytradePaperCommonArgs,
        #[arg(long, default_value_t = 20)]
        bb_period: usize,
        #[arg(long, default_value_t = 2.0)]
        bb_sigma: f64,
    },
    /// 일목균형표 (분봉, 페이퍼)
    Ichimoku {
        symbol: String,
        #[command(flatten)]
        common: DaytradePaperCommonArgs,
    },
    /// OBV vs SMA(OBV, N) 크로스오버 (분봉, 페이퍼)
    Obv {
        symbol: String,
        #[command(flatten)]
        common: DaytradePaperCommonArgs,
        #[arg(long, default_value_t = 20)]
        obv_period: usize,
    },
}

#[derive(Subcommand)]
enum DaytradeRunStrategy {
    /// 단기·장기 이동평균 교차 (분봉, 실주문)
    MaCross {
        symbol: String,
        #[command(flatten)]
        common: DaytradeRunCommonArgs,
        #[arg(long, default_value_t = 20)]
        fast: usize,
        #[arg(long, default_value_t = 60)]
        slow: usize,
    },
    /// RSI (분봉, 실주문)
    Rsi {
        symbol: String,
        #[command(flatten)]
        common: DaytradeRunCommonArgs,
        #[arg(long, default_value_t = 14)]
        rsi_period: usize,
        #[arg(long, default_value_t = 30.0)]
        rsi_oversold: f64,
        #[arg(long, default_value_t = 70.0)]
        rsi_overbought: f64,
    },
    /// MACD (분봉, 실주문)
    Macd {
        symbol: String,
        #[command(flatten)]
        common: DaytradeRunCommonArgs,
    },
    /// 볼린저 (분봉, 실주문)
    Bollinger {
        symbol: String,
        #[command(flatten)]
        common: DaytradeRunCommonArgs,
        #[arg(long, default_value_t = 20)]
        bb_period: usize,
        #[arg(long, default_value_t = 2.0)]
        bb_sigma: f64,
    },
    /// 일목균형표 (분봉, 실주문)
    Ichimoku {
        symbol: String,
        #[command(flatten)]
        common: DaytradeRunCommonArgs,
    },
    /// OBV (분봉, 실주문)
    Obv {
        symbol: String,
        #[command(flatten)]
        common: DaytradeRunCommonArgs,
        #[arg(long, default_value_t = 20)]
        obv_period: usize,
    },
}

#[derive(Args)]
struct DaytradeRunCommonArgs {
    #[command(flatten)]
    paper: DaytradePaperCommonArgs,
    /// 호가 오프셋 (틱 수). 매수 시 +, 매도 시 -. 기본 0 = 종가 그대로 지정가
    #[arg(long, default_value_t = 0)]
    tick_offset: i32,
    /// 체결 확인 폴링 타임아웃 (초)
    #[arg(long, default_value_t = 30)]
    fill_timeout_secs: u64,
    /// 체결 확인 폴링 간격 (초)
    #[arg(long, default_value_t = 2)]
    poll_interval_secs: u64,
    /// 대화형 확인 생략 (기본: 첫 실주문 진입 전 y/n 확인)
    #[arg(long)]
    yes: bool,
}

#[derive(Args)]
struct DaytradePaperCommonArgs {
    #[command(flatten)]
    base: DaytradeCommonArgs,
    /// 1회 매수 수량 (주)
    #[arg(long)]
    qty: u64,
    /// 수수료 bps (매매 한쪽당). 국내 ~15, 해외 ~5 — 왕복 기준 2배
    #[arg(long, default_value_t = 15.0)]
    fee_bps: f64,
    /// 슬리피지 bps (체결가 보정, 매매 한쪽당). 왕복 기준 2배 — 기본 10bps × 2 = 20bps 왕복, fee 30bps 포함 총 왕복 50bps
    #[arg(long, default_value_t = 10.0)]
    slippage_bps: f64,
    /// 손절 임계 % (진입가 대비 -N% 하회 시 즉시 청산). paper 미지정 시 off, run 미지정 시 2.0% 기본
    #[arg(long)]
    stop_loss_pct: Option<f64>,
    /// 익절 임계 % (진입가 대비 +N% 도달 시 즉시 청산). paper 미지정 시 off, run 미지정 시 5.0% 기본
    #[arg(long)]
    take_profit_pct: Option<f64>,
    /// ATR 배수 손절. 진입가 - ATR(N) × M 하회 시 청산. `--stop-loss-pct`와 둘 다 지정 시 더 타이트한 쪽 사용
    #[arg(long)]
    stop_loss_atr: Option<f64>,
    /// ATR 배수 익절. 진입가 + ATR(N) × M 도달 시 청산. `--take-profit-pct`와 둘 다 지정 시 더 타이트한 쪽 사용
    #[arg(long)]
    take_profit_atr: Option<f64>,
    /// ATR 계산 봉 수
    #[arg(long, default_value_t = 14)]
    atr_period: usize,
    /// 총 예산 (USA: USD, KRX: KRW). 보유 중에도 롱 신호마다 예산 한도 내에서 `qty`주씩 추가 매수(피라미딩).
    #[arg(long)]
    budget: f64,
    /// systemd unit 등록 + enable --now (Linux, 루트 필요). 다른 OS는 unit 파일만 출력.
    /// 예: `sudo $(which kis) daytrade paper rsi TSLA --usa --qty 1 --budget 5000 --background`
    #[arg(long)]
    background: bool,
}

#[derive(Subcommand)]
enum DaytradeStrategy {
    /// 단기·장기 이동평균 교차 (분봉)
    MaCross {
        symbol: String,
        #[command(flatten)]
        common: DaytradeCommonArgs,
        #[arg(long, default_value_t = 20)]
        fast: usize,
        #[arg(long, default_value_t = 60)]
        slow: usize,
    },
    /// RSI 과매도/과매수 반전 (분봉)
    Rsi {
        symbol: String,
        #[command(flatten)]
        common: DaytradeCommonArgs,
        #[arg(long, default_value_t = 14)]
        rsi_period: usize,
        #[arg(long, default_value_t = 30.0)]
        rsi_oversold: f64,
        #[arg(long, default_value_t = 70.0)]
        rsi_overbought: f64,
    },
    /// MACD 히스토그램 부호 (12/26/9 고정, 분봉)
    Macd {
        symbol: String,
        #[command(flatten)]
        common: DaytradeCommonArgs,
    },
    /// 볼린저 밴드 평균회귀 (분봉)
    Bollinger {
        symbol: String,
        #[command(flatten)]
        common: DaytradeCommonArgs,
        #[arg(long, default_value_t = 20)]
        bb_period: usize,
        #[arg(long, default_value_t = 2.0)]
        bb_sigma: f64,
    },
    /// 일목균형표 (9/26/52 고정, 분봉)
    Ichimoku {
        symbol: String,
        #[command(flatten)]
        common: DaytradeCommonArgs,
    },
    /// OBV vs SMA(OBV, N) 크로스오버 (분봉)
    Obv {
        symbol: String,
        #[command(flatten)]
        common: DaytradeCommonArgs,
        #[arg(long, default_value_t = 20)]
        obv_period: usize,
    },
}

#[derive(Args)]
struct DaytradeCommonArgs {
    /// 분봉 주기 (1m / 5m / 10m / 30m / 60m)
    #[arg(long, default_value = "5m")]
    period: String,
    /// 해외 종목 (기본: 국내)
    #[arg(long)]
    usa: bool,
    /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
    #[arg(long)]
    pick: Option<usize>,
}

#[derive(Args)]
struct BacktestChartSeed {
    /// 해외 종목 (기본: 국내)
    #[arg(long)]
    usa: bool,
    /// 봉 주기 (D/W/M)
    #[arg(long, default_value = "D")]
    period: String,
    /// 시작일 YYYYMMDD (없으면 가져온 전 구간)
    #[arg(long)]
    from: Option<String>,
    /// 종료일 YYYYMMDD (없으면 최신)
    #[arg(long)]
    to: Option<String>,
    /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
    #[arg(long)]
    pick: Option<usize>,
}

#[derive(Args)]
struct BacktestCommonArgs {
    /// 해외 종목 (기본: 국내)
    #[arg(long)]
    usa: bool,
    /// 봉 주기 (D/W/M)
    #[arg(long, default_value = "D")]
    period: String,
    /// 시작일 YYYYMMDD (없으면 가져온 전 구간)
    #[arg(long)]
    from: Option<String>,
    /// 종료일 YYYYMMDD (없으면 최신)
    #[arg(long)]
    to: Option<String>,
    /// 수수료 (bps, 진입·청산 각각). 기본 5.0 = 0.05%
    #[arg(long, default_value_t = 5.0)]
    fee_bps: f64,
    /// 슬리피지 (bps, 진입·청산 각각). 기본 0
    #[arg(long, default_value_t = 0.0)]
    slippage_bps: f64,
    /// 숏 포지션 허용 (양방향 트레이드)
    #[arg(long)]
    allow_short: bool,
    /// 레버리지 배수 (수익률·수수료에 승수). 기본 1.0
    #[arg(long, default_value_t = 1.0)]
    leverage: f64,
    /// 손절 기준 (%). 포지션 대비 손실이 이 값 이상이면 강제 청산
    #[arg(long)]
    stop_loss_pct: Option<f64>,
    /// 익절 기준 (%). 포지션 대비 수익이 이 값 이상이면 강제 청산
    #[arg(long)]
    take_profit_pct: Option<f64>,
    /// 파라미터 스윕 (내장 그리드로 탐색, Sharpe 내림차순 상위 15개 요약)
    #[arg(long)]
    sweep: bool,
    /// JSON 덤프
    #[arg(long)]
    json: bool,
    /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
    #[arg(long)]
    pick: Option<usize>,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// 설정 파일 초기화
    Init,
    /// 현재 설정 경로 출력
    Path,
    /// 텔레그램 bot 연동 — 봇이 받은 /start 메시지를 감지해 chat_id 자동 채움 + 테스트 메시지 전송
    Telegram,
}

#[derive(Subcommand)]
enum StopLossAction {
    /// 데몬 시작 — 잔고 감시 + 임계치 도달 시 매도
    Run {
        /// 손절 임계치 (%). 이 값보다 손실이 크면 매도. 기본 -5.0
        #[arg(long, default_value_t = -5.0)]
        threshold: f64,
        /// 확인 주기 (초). 기본 30
        #[arg(long, default_value_t = 30)]
        interval: u64,
        /// 감시 대상 (쉼표 구분, 코드 또는 종목명 일부). 미지정 시 전체 잔고
        #[arg(long)]
        symbols: Option<String>,
        /// 실제 매도 집행. 없으면 dry-run (로그만)
        #[arg(long)]
        execute: bool,
        /// 해외주식 지정가 스프레드 (%). 기본 1.0
        #[arg(long, default_value_t = 1.0)]
        usa_spread: f64,
        /// WebSocket 실시간 감시 (폴링 대신 tick 단위)
        #[arg(long)]
        ws: bool,
    },
    /// 현재 실행 중인 데몬 상태 조회
    Status,
    /// 상태 파일 경로 출력
    Path,
}

#[derive(Subcommand)]
enum SkillAction {
    /// ~/.claude/skills/kis/SKILL.md 에 설치
    Install {
        /// 기존 파일 덮어쓰기
        #[arg(long)]
        force: bool,
    },
    /// 설치된 스킬 삭제
    Uninstall,
    /// 설치 경로 출력
    Path,
}

#[derive(Subcommand)]
enum SymbolsAction {
    /// 마스터 파일을 다운로드해 로컬 DB 갱신
    Sync {
        /// 최근 24시간 내에 동기화했으면 건너뜀
        #[arg(long)]
        if_stale: bool,
    },
    /// 종목 검색 (한글명/영문명/코드)
    Find {
        query: String,
        #[arg(long, default_value_t = 20)]
        limit: usize,
    },
}

#[derive(Subcommand)]
enum StockAction {
    /// 국내주식
    Dome {
        #[command(subcommand)]
        action: DomeStockAction,
    },
    /// 해외주식 (미국)
    Usa {
        #[command(subcommand)]
        action: UsaStockAction,
    },
}

#[derive(Subcommand)]
enum DomeStockAction {
    /// 현재가
    Price {
        symbol: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 차트 (D 일/ W 주/ M 월)
    Chart {
        symbol: String,
        #[arg(long, default_value = "D")]
        period: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 호가
    Asking {
        symbol: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 잔고
    Balance,
    /// 매수가능 조회
    Psbl,
    /// 주문 (매수/매도/취소)
    Order {
        #[command(subcommand)]
        action: DomeOrderAction,
    },
    /// 최근 주문/체결 내역
    History,
    /// 실시간 체결가 스트리밍
    Watch {
        symbol: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
}

#[derive(Subcommand)]
enum DomeOrderAction {
    /// 매수
    Buy {
        symbol: String,
        qty: u64,
        #[arg(long)]
        price: Option<u64>,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 매도
    Sell {
        symbol: String,
        qty: u64,
        #[arg(long)]
        price: Option<u64>,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 취소 (qty=0 이면 전량 취소)
    Cancel {
        order_no: String,
        #[arg(default_value_t = 0)]
        qty: u64,
    },
}

#[derive(Subcommand)]
enum UsaStockAction {
    /// 현재가
    Price {
        symbol: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 차트
    Chart {
        symbol: String,
        #[arg(long, default_value = "D")]
        period: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 잔고
    Balance {
        /// 거래소 (NASD/NYSE/AMEX)
        #[arg(long, default_value = "NASD")]
        exchange: String,
    },
    /// 주문
    Order {
        #[command(subcommand)]
        action: UsaOrderAction,
    },
    /// 주문/체결 내역
    History {
        #[arg(long, default_value = "NASD")]
        exchange: String,
    },
    /// 실시간 체결가 스트리밍
    Watch {
        symbol: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
}

#[derive(Subcommand)]
enum UsaOrderAction {
    /// 매수 (지정가 필수)
    Buy {
        symbol: String,
        qty: u64,
        #[arg(long)]
        price: f64,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 매도 (지정가 필수)
    Sell {
        symbol: String,
        qty: u64,
        #[arg(long)]
        price: f64,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
}

#[derive(Subcommand)]
enum BondAction {
    /// 현재가
    Price { symbol: String },
    /// 일봉
    Chart { symbol: String },
    /// 잔고
    Balance,
    /// 주문
    Order {
        #[command(subcommand)]
        action: BondOrderAction,
    },
}

#[derive(Subcommand)]
enum BondOrderAction {
    Buy { symbol: String, qty: u64, #[arg(long)] price: f64 },
    Sell { symbol: String, qty: u64, #[arg(long)] price: f64 },
}

#[derive(Subcommand)]
enum FoAction {
    /// 국내선물옵션
    Dome {
        #[command(subcommand)]
        action: DomeFoAction,
    },
    /// 해외선물옵션 (모의투자 미지원)
    Usa {
        #[command(subcommand)]
        action: UsaFoAction,
    },
}

#[derive(Subcommand)]
enum DomeFoAction {
    /// 현재가
    Price {
        symbol: String,
        /// 시장구분 (F 지수선물, O 지수옵션, JF 주식선물, JO 주식옵션)
        #[arg(long)]
        market: Option<String>,
    },
    /// 잔고
    Balance,
    /// 주문
    Order {
        #[command(subcommand)]
        action: DomeFoOrderAction,
    },
    /// KRX 야간선물 실시간 체결 스트리밍 (H0MFCNT0)
    ///
    /// 심볼은 선물 마스터 이름(예: KOSPI200) 또는 직접 단축코드. 마스터는 `kis symbols sync` 후 사용.
    WatchNight {
        symbol: String,
        /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY(systemd/파이프) 필수. 예: --pick 1
        #[arg(long)]
        pick: Option<usize>,
    },
}

#[derive(Subcommand)]
enum DomeFoOrderAction {
    Buy { symbol: String, qty: u64, #[arg(long)] price: Option<f64> },
    Sell { symbol: String, qty: u64, #[arg(long)] price: Option<f64> },
}

#[derive(Subcommand)]
enum UsaFoAction {
    /// 현재가
    Price { symbol: String },
    /// 예수금/평가
    Balance {
        #[arg(long, default_value = "USD")]
        currency: String,
    },
    /// 주문
    Order {
        #[command(subcommand)]
        action: UsaFoOrderAction,
    },
}

#[derive(Subcommand)]
enum UsaFoOrderAction {
    Buy { symbol: String, qty: u64, #[arg(long)] price: f64 },
    Sell { symbol: String, qty: u64, #[arg(long)] price: f64 },
}

fn build_client() -> Result<KisClient> {
    let cfg = config::load_config()?;
    Ok(KisClient::with_mock(cfg.credentials, cfg.is_mock))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // --chart는 wry 창을 main 스레드에서 띄워야 함 (macOS AppKit 요구).
    // 비동기 prep만 런타임에서 돌리고 이벤트 루프는 main 스레드에서 블로킹.
    if let Commands::Analyze { symbol, usa, chart: true, json, pick, .. } = &cli.command {
        return run_chart_viewer(symbol, *usa, *json, *pick);
    }
    if matches!(&cli.command, Commands::Backtest { strategy: BacktestStrategy::Chart { .. } }) {
        return run_backtest_chart(cli);
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async_main(cli))
}

fn base_backtest_params(
    common: &BacktestCommonArgs,
    kind: commands::backtest::StrategyKind,
) -> commands::backtest::Params {
    commands::backtest::Params {
        strategy: kind,
        period: common.period.chars().next().unwrap_or('D'),
        from: common.from.clone(),
        to: common.to.clone(),
        fee_bps: common.fee_bps,
        slippage_bps: common.slippage_bps,
        allow_short: common.allow_short,
        leverage: common.leverage,
        stop_loss_pct: common.stop_loss_pct,
        take_profit_pct: common.take_profit_pct,
        fast: None,
        slow: None,
        rsi_period: None,
        rsi_oversold: None,
        rsi_overbought: None,
        bb_period: None,
        bb_sigma: None,
        obv_period: None,
        manual_entry_date: None,
        manual_exit_date: None,
        manual_direction: None,
    }
}

/// 비차트 서브커맨드를 (symbol, common, Params) 로 풀어낸다. Chart 는 별도 경로(run_backtest_chart).
fn unpack_backtest(
    s: BacktestStrategy,
) -> (String, BacktestCommonArgs, commands::backtest::Params) {
    use commands::backtest::{normalize_date, StrategyKind};
    match s {
        BacktestStrategy::MaCross { symbol, common, fast, slow } => {
            let mut p = base_backtest_params(&common, StrategyKind::MaCross);
            p.fast = Some(fast);
            p.slow = Some(slow);
            (symbol, common, p)
        }
        BacktestStrategy::Rsi { symbol, common, rsi_period, rsi_oversold, rsi_overbought } => {
            let mut p = base_backtest_params(&common, StrategyKind::Rsi);
            p.rsi_period = Some(rsi_period);
            p.rsi_oversold = Some(rsi_oversold);
            p.rsi_overbought = Some(rsi_overbought);
            (symbol, common, p)
        }
        BacktestStrategy::Macd { symbol, common } => {
            let p = base_backtest_params(&common, StrategyKind::Macd);
            (symbol, common, p)
        }
        BacktestStrategy::Bollinger { symbol, common, bb_period, bb_sigma } => {
            let mut p = base_backtest_params(&common, StrategyKind::Bollinger);
            p.bb_period = Some(bb_period);
            p.bb_sigma = Some(bb_sigma);
            (symbol, common, p)
        }
        BacktestStrategy::Ichimoku { symbol, common } => {
            let p = base_backtest_params(&common, StrategyKind::Ichimoku);
            (symbol, common, p)
        }
        BacktestStrategy::Obv { symbol, common, obv_period } => {
            let mut p = base_backtest_params(&common, StrategyKind::Obv);
            p.obv_period = Some(obv_period);
            (symbol, common, p)
        }
        BacktestStrategy::Manual { symbol, common, entry_date, exit_date, direction } => {
            let mut p = base_backtest_params(&common, StrategyKind::Manual);
            p.manual_entry_date = normalize_date(Some(entry_date));
            p.manual_exit_date = normalize_date(exit_date);
            p.manual_direction = Some(direction);
            (symbol, common, p)
        }
        BacktestStrategy::Chart { .. } => unreachable!("Chart 는 run_backtest_chart 가 처리"),
    }
}

fn unpack_signal_watch(s: SignalWatchStrategy) -> commands::signal_watch::Config {
    use commands::backtest::StrategyKind;
    let mut cfg = commands::signal_watch::Config {
        symbol: String::new(),
        strategy: StrategyKind::MaCross,
        cron: String::new(),
        period: 'D',
        usa: false,
        pick: None,
        fast: None,
        slow: None,
        rsi_period: None,
        rsi_oversold: None,
        rsi_overbought: None,
        bb_period: None,
        bb_sigma: None,
        obv_period: None,
    };
    let apply_common = |cfg: &mut commands::signal_watch::Config, c: SignalWatchCommonArgs| {
        cfg.cron = c.cron;
        cfg.period = c.period.chars().next().unwrap_or('D');
        cfg.usa = c.usa;
        cfg.pick = c.pick;
    };
    match s {
        SignalWatchStrategy::MaCross { symbol, common, fast, slow } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::MaCross;
            cfg.fast = Some(fast);
            cfg.slow = Some(slow);
            apply_common(&mut cfg, common);
        }
        SignalWatchStrategy::Rsi { symbol, common, rsi_period, rsi_oversold, rsi_overbought } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Rsi;
            cfg.rsi_period = Some(rsi_period);
            cfg.rsi_oversold = Some(rsi_oversold);
            cfg.rsi_overbought = Some(rsi_overbought);
            apply_common(&mut cfg, common);
        }
        SignalWatchStrategy::Macd { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Macd;
            apply_common(&mut cfg, common);
        }
        SignalWatchStrategy::Bollinger { symbol, common, bb_period, bb_sigma } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Bollinger;
            cfg.bb_period = Some(bb_period);
            cfg.bb_sigma = Some(bb_sigma);
            apply_common(&mut cfg, common);
        }
        SignalWatchStrategy::Ichimoku { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Ichimoku;
            apply_common(&mut cfg, common);
        }
        SignalWatchStrategy::Obv { symbol, common, obv_period } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Obv;
            cfg.obv_period = Some(obv_period);
            apply_common(&mut cfg, common);
        }
        SignalWatchStrategy::All { .. }
        | SignalWatchStrategy::List
        | SignalWatchStrategy::Remove { .. } => {
            unreachable!("All/List/Remove 는 dispatcher 에서 별도 처리")
        }
    }
    cfg
}

fn unpack_daytrade_signal_watch(
    s: DaytradeStrategy,
) -> Result<commands::daytrade::signal_watch::Config> {
    use commands::backtest::StrategyKind;
    use commands::daytrade::period::Period;
    use std::str::FromStr;

    let mut cfg = commands::daytrade::signal_watch::Config {
        symbol: String::new(),
        strategy: StrategyKind::MaCross,
        period: Period::Min(5),
        usa: false,
        pick: None,
        fast: None,
        slow: None,
        rsi_period: None,
        rsi_oversold: None,
        rsi_overbought: None,
        bb_period: None,
        bb_sigma: None,
        obv_period: None,
    };
    let apply_common = |cfg: &mut commands::daytrade::signal_watch::Config,
                        c: DaytradeCommonArgs|
     -> Result<()> {
        cfg.period = Period::from_str(&c.period)?;
        cfg.usa = c.usa;
        cfg.pick = c.pick;
        Ok(())
    };
    match s {
        DaytradeStrategy::MaCross { symbol, common, fast, slow } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::MaCross;
            cfg.fast = Some(fast);
            cfg.slow = Some(slow);
            apply_common(&mut cfg, common)?;
        }
        DaytradeStrategy::Rsi { symbol, common, rsi_period, rsi_oversold, rsi_overbought } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Rsi;
            cfg.rsi_period = Some(rsi_period);
            cfg.rsi_oversold = Some(rsi_oversold);
            cfg.rsi_overbought = Some(rsi_overbought);
            apply_common(&mut cfg, common)?;
        }
        DaytradeStrategy::Macd { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Macd;
            apply_common(&mut cfg, common)?;
        }
        DaytradeStrategy::Bollinger { symbol, common, bb_period, bb_sigma } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Bollinger;
            cfg.bb_period = Some(bb_period);
            cfg.bb_sigma = Some(bb_sigma);
            apply_common(&mut cfg, common)?;
        }
        DaytradeStrategy::Ichimoku { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Ichimoku;
            apply_common(&mut cfg, common)?;
        }
        DaytradeStrategy::Obv { symbol, common, obv_period } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Obv;
            cfg.obv_period = Some(obv_period);
            apply_common(&mut cfg, common)?;
        }
    }
    Ok(cfg)
}

fn unpack_daytrade_paper(
    s: DaytradePaperStrategy,
) -> Result<commands::daytrade::paper::Config> {
    use commands::backtest::StrategyKind;
    use commands::daytrade::period::Period;
    use std::str::FromStr;

    let mut cfg = commands::daytrade::paper::Config {
        symbol: String::new(),
        strategy: StrategyKind::MaCross,
        period: Period::Min(5),
        usa: false,
        pick: None,
        qty: 1,
        fee_bps: 15.0,
        slippage_bps: 10.0,
        stop_loss_pct: None,
        take_profit_pct: None,
        stop_loss_atr: None,
        take_profit_atr: None,
        atr_period: 14,
        budget: 0.0,
        background: false,
        fast: None,
        slow: None,
        rsi_period: None,
        rsi_oversold: None,
        rsi_overbought: None,
        bb_period: None,
        bb_sigma: None,
        obv_period: None,
    };
    let apply_common = |cfg: &mut commands::daytrade::paper::Config,
                        c: DaytradePaperCommonArgs|
     -> Result<()> {
        cfg.period = Period::from_str(&c.base.period)?;
        cfg.usa = c.base.usa;
        cfg.pick = c.base.pick;
        cfg.qty = c.qty;
        cfg.fee_bps = c.fee_bps;
        cfg.slippage_bps = c.slippage_bps;
        cfg.stop_loss_pct = c.stop_loss_pct;
        cfg.take_profit_pct = c.take_profit_pct;
        cfg.stop_loss_atr = c.stop_loss_atr;
        cfg.take_profit_atr = c.take_profit_atr;
        cfg.atr_period = c.atr_period;
        cfg.budget = c.budget;
        cfg.background = c.background;
        Ok(())
    };
    match s {
        DaytradePaperStrategy::MaCross { symbol, common, fast, slow } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::MaCross;
            cfg.fast = Some(fast);
            cfg.slow = Some(slow);
            apply_common(&mut cfg, common)?;
        }
        DaytradePaperStrategy::Rsi { symbol, common, rsi_period, rsi_oversold, rsi_overbought } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Rsi;
            cfg.rsi_period = Some(rsi_period);
            cfg.rsi_oversold = Some(rsi_oversold);
            cfg.rsi_overbought = Some(rsi_overbought);
            apply_common(&mut cfg, common)?;
        }
        DaytradePaperStrategy::Macd { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Macd;
            apply_common(&mut cfg, common)?;
        }
        DaytradePaperStrategy::Bollinger { symbol, common, bb_period, bb_sigma } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Bollinger;
            cfg.bb_period = Some(bb_period);
            cfg.bb_sigma = Some(bb_sigma);
            apply_common(&mut cfg, common)?;
        }
        DaytradePaperStrategy::Ichimoku { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Ichimoku;
            apply_common(&mut cfg, common)?;
        }
        DaytradePaperStrategy::Obv { symbol, common, obv_period } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Obv;
            cfg.obv_period = Some(obv_period);
            apply_common(&mut cfg, common)?;
        }
    }
    Ok(cfg)
}

fn unpack_daytrade_run(
    s: DaytradeRunStrategy,
) -> Result<commands::daytrade::run::Config> {
    use commands::backtest::StrategyKind;
    use commands::daytrade::period::Period;
    use std::str::FromStr;

    let mut cfg = commands::daytrade::run::Config {
        symbol: String::new(),
        strategy: StrategyKind::MaCross,
        period: Period::Min(5),
        usa: false,
        pick: None,
        qty: 1,
        fee_bps: 15.0,
        stop_loss_pct: None,
        take_profit_pct: None,
        stop_loss_atr: None,
        take_profit_atr: None,
        atr_period: 14,
        budget: 0.0,
        tick_offset: 0,
        fill_timeout_secs: 30,
        poll_interval_secs: 2,
        yes: false,
        background: false,
        fast: None,
        slow: None,
        rsi_period: None,
        rsi_oversold: None,
        rsi_overbought: None,
        bb_period: None,
        bb_sigma: None,
        obv_period: None,
    };
    let apply_common = |cfg: &mut commands::daytrade::run::Config,
                        c: DaytradeRunCommonArgs|
     -> Result<()> {
        cfg.period = Period::from_str(&c.paper.base.period)?;
        cfg.usa = c.paper.base.usa;
        cfg.pick = c.paper.base.pick;
        cfg.qty = c.paper.qty;
        cfg.fee_bps = c.paper.fee_bps;
        // 실주문 기본 안전장치: SL 2%, TP 5% (사용자가 명시하면 그 값 사용)
        cfg.stop_loss_pct = Some(c.paper.stop_loss_pct.unwrap_or(2.0));
        cfg.take_profit_pct = Some(c.paper.take_profit_pct.unwrap_or(5.0));
        cfg.stop_loss_atr = c.paper.stop_loss_atr;
        cfg.take_profit_atr = c.paper.take_profit_atr;
        cfg.atr_period = c.paper.atr_period;
        cfg.budget = c.paper.budget;
        cfg.tick_offset = c.tick_offset;
        cfg.fill_timeout_secs = c.fill_timeout_secs;
        cfg.poll_interval_secs = c.poll_interval_secs;
        cfg.yes = c.yes;
        cfg.background = c.paper.background;
        Ok(())
    };
    match s {
        DaytradeRunStrategy::MaCross { symbol, common, fast, slow } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::MaCross;
            cfg.fast = Some(fast);
            cfg.slow = Some(slow);
            apply_common(&mut cfg, common)?;
        }
        DaytradeRunStrategy::Rsi { symbol, common, rsi_period, rsi_oversold, rsi_overbought } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Rsi;
            cfg.rsi_period = Some(rsi_period);
            cfg.rsi_oversold = Some(rsi_oversold);
            cfg.rsi_overbought = Some(rsi_overbought);
            apply_common(&mut cfg, common)?;
        }
        DaytradeRunStrategy::Macd { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Macd;
            apply_common(&mut cfg, common)?;
        }
        DaytradeRunStrategy::Bollinger { symbol, common, bb_period, bb_sigma } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Bollinger;
            cfg.bb_period = Some(bb_period);
            cfg.bb_sigma = Some(bb_sigma);
            apply_common(&mut cfg, common)?;
        }
        DaytradeRunStrategy::Ichimoku { symbol, common } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Ichimoku;
            apply_common(&mut cfg, common)?;
        }
        DaytradeRunStrategy::Obv { symbol, common, obv_period } => {
            cfg.symbol = symbol;
            cfg.strategy = StrategyKind::Obv;
            cfg.obv_period = Some(obv_period);
            apply_common(&mut cfg, common)?;
        }
    }
    Ok(cfg)
}

fn run_backtest_chart(cli: Cli) -> Result<()> {
    let Commands::Backtest { strategy: BacktestStrategy::Chart { symbol, seed } } = cli.command
    else {
        unreachable!("gated by matches! in main")
    };
    let mode = if seed.usa { symbols::ResolveMode::Overseas } else { symbols::ResolveMode::Domestic };
    let pick = seed.pick;
    // 초기 시드는 ma-cross(20/60) 기본값. GUI 폼에서 사용자가 변경.
    let params = commands::backtest::Params {
        strategy: commands::backtest::StrategyKind::MaCross,
        period: seed.period.chars().next().unwrap_or('D'),
        from: seed.from,
        to: seed.to,
        fee_bps: 5.0,
        slippage_bps: 0.0,
        allow_short: false,
        leverage: 1.0,
        stop_loss_pct: None,
        take_profit_pct: None,
        fast: Some(20),
        slow: Some(60),
        rsi_period: None,
        rsi_oversold: None,
        rsi_overbought: None,
        bb_period: None,
        bb_sigma: None,
        obv_period: None,
        manual_entry_date: None,
        manual_exit_date: None,
        manual_direction: None,
    };

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;

    let (prep, client) = rt.block_on(async {
        let client = std::sync::Arc::new(build_client()?);
        let prep = commands::backtest::prepare_chart(&client, &symbol, mode, params, pick).await?;
        Ok::<_, anyhow::Error>((prep, client))
    })?;

    let store = symbols::Store::open(&config::symbols_db_path()?)?;

    let ctx = viewer::BacktestCtx {
        rt: rt.handle().clone(),
        client,
        store: std::sync::Arc::new(std::sync::Mutex::new(store)),
        code: std::sync::Arc::new(std::sync::Mutex::new(prep.code)),
        name: std::sync::Arc::new(std::sync::Mutex::new(prep.name)),
        mode: std::sync::Arc::new(std::sync::Mutex::new(mode)),
        series: std::sync::Arc::new(std::sync::Mutex::new(prep.series)),
        period: std::sync::Arc::new(std::sync::Mutex::new(prep.period)),
        from: std::sync::Arc::new(std::sync::Mutex::new(prep.from)),
        to: std::sync::Arc::new(std::sync::Mutex::new(prep.to)),
    };
    let _rt_guard = rt;
    viewer::launch_backtest(&prep.title, &prep.html, ctx)
}

fn run_chart_viewer(symbol: &str, usa: bool, json: bool, pick: Option<usize>) -> Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let mode = if usa { symbols::ResolveMode::Overseas } else { symbols::ResolveMode::Domestic };

    let (title, html, client, sym_code, sym_name, series) = rt.block_on(async {
        let client = std::sync::Arc::new(build_client()?);
        let (sym, series, report, html) =
            commands::analyze::prepare(&client, symbol, mode, pick).await?;
        // --json이 같이 지정됐으면 차트 창 띄우기 전에 stdout으로 JSON 리포트 출력
        if json {
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        let title = format!("[{}] {} — kis-cli", sym.code, sym.name_kr);
        let name = if !sym.name_kr.is_empty() { sym.name_kr.clone() } else { sym.name_en.clone() };
        Ok::<_, anyhow::Error>((title, html, client, sym.code, name, series))
    })?;

    // 심볼 DB 공유용 (검색 IPC에서 사용)
    let store = symbols::Store::open(&config::symbols_db_path()?)?;

    let ctx = viewer::ViewerCtx {
        rt: rt.handle().clone(),
        client,
        store: std::sync::Arc::new(std::sync::Mutex::new(store)),
        state: std::sync::Arc::new(std::sync::Mutex::new(viewer::ViewerState {
            series,
            period: 'D',
            symbol_code: sym_code,
            symbol_name: sym_name,
            mode,
        })),
    };
    // 런타임은 프로세스 종료까지 유지 (IPC 핸들러가 spawn).
    let _rt_guard = rt;
    viewer::launch(&title, &html, ctx)
}

async fn async_main(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Auth => commands::auth::run(&build_client()?).await,

        Commands::Config { action } => match action {
            ConfigAction::Init => config_init(),
            ConfigAction::Path => {
                println!("{}", config::config_path()?.display());
                Ok(())
            }
            ConfigAction::Telegram => config_telegram().await,
        },

        Commands::Skill { action } => match action {
            SkillAction::Install { force } => commands::skill::run_install(force),
            SkillAction::Uninstall => commands::skill::run_uninstall(),
            SkillAction::Path => commands::skill::run_path(),
        },

        Commands::Install { force } => commands::installer::run_install(force),
        Commands::Update { from_source, no_pull } => {
            commands::installer::run_update(from_source, no_pull).await
        }
        Commands::Version => commands::installer::run_version().await,

        Commands::Symbols { action } => match action {
            SymbolsAction::Sync { if_stale } => commands::symbols::run_sync(if_stale).await,
            SymbolsAction::Find { query, limit } => commands::symbols::run_find(&query, limit),
        },

        Commands::Stock { action } => match action {
            StockAction::Dome { action } => dispatch_dome_stock(action).await,
            StockAction::Usa { action } => dispatch_usa_stock(action).await,
        },

        Commands::Bond { action } => dispatch_bond(action).await,

        Commands::Fo { action } => match action {
            FoAction::Dome { action } => dispatch_dome_fo(action).await,
            FoAction::Usa { action } => dispatch_usa_fo(action).await,
        },

        Commands::StopLoss { action } => match action {
            StopLossAction::Run { threshold, interval, symbols, execute, usa_spread, ws } => {
                let client = build_client()?;
                let syms = symbols.map(|s| {
                    s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect::<Vec<_>>()
                });
                let cfg = commands::stop_loss::Config {
                    threshold_pct: threshold,
                    interval_secs: interval,
                    symbols: syms,
                    execute,
                    usa_spread_pct: usa_spread,
                    use_ws: ws,
                };
                commands::stop_loss::run(&client, cfg).await
            }
            StopLossAction::Status => commands::stop_loss::run_status(),
            StopLossAction::Path => commands::stop_loss::run_path(),
        },

        Commands::SignalWatch { strategy } => match strategy {
            SignalWatchStrategy::All { symbol, cron, usa, pick, background } => {
                let client = std::sync::Arc::new(build_client()?);
                let cfg = commands::signal_watch::AllConfig {
                    symbol, cron, usa, pick, background,
                };
                commands::signal_watch::run_all(client, cfg).await
            }
            SignalWatchStrategy::List => commands::signal_watch::list_services(),
            SignalWatchStrategy::Remove { target, usa } => {
                commands::signal_watch::remove_service(&target, usa)
            }
            other => {
                let client = std::sync::Arc::new(build_client()?);
                let cfg = unpack_signal_watch(other);
                commands::signal_watch::run(client, cfg).await
            }
        },

        Commands::Daytrade { action } => match action {
            DaytradeAction::SignalWatch { strategy } => {
                let client = std::sync::Arc::new(build_client()?);
                let cfg = unpack_daytrade_signal_watch(strategy)?;
                commands::daytrade::signal_watch::run(client, cfg).await
            }
            DaytradeAction::Paper { strategy } => {
                let client = std::sync::Arc::new(build_client()?);
                let cfg = unpack_daytrade_paper(strategy)?;
                commands::daytrade::paper::run(client, cfg).await
            }
            DaytradeAction::Run { strategy } => {
                let client = std::sync::Arc::new(build_client()?);
                let cfg = unpack_daytrade_run(strategy)?;
                commands::daytrade::run::run(client, cfg).await
            }
            DaytradeAction::List => commands::daytrade::background::list_services(),
            DaytradeAction::Remove { target } => {
                commands::daytrade::background::remove_service(&target)
            }
            DaytradeAction::History { session, symbol, today, days, limit, json } => {
                let opts = commands::daytrade::history::Opts {
                    session, symbol, today, days, limit, json,
                };
                commands::daytrade::history::run(opts)
            }
        },

        Commands::Backtest { strategy } => {
            // Chart 는 main() 에서 먼저 가로채기 때문에 여긴 닿지 않음
            debug_assert!(!matches!(strategy, BacktestStrategy::Chart { .. }));
            let (symbol, common, params) = unpack_backtest(strategy);
            let client = build_client()?;
            let mode = if common.usa { symbols::ResolveMode::Overseas } else { symbols::ResolveMode::Domestic };
            let opts = commands::backtest::RunOpts {
                json: common.json,
                sweep: common.sweep,
            };
            commands::backtest::run(&client, &symbol, mode, params, opts, common.pick).await
        }

        Commands::Analyze { symbol, usa, json, chart: _, save, pick } => {
            // chart=true 경로는 main()에서 이미 가로챘다 (wry 창). 여기선 json/save만.
            let client = build_client()?;
            let mode = if usa { symbols::ResolveMode::Overseas } else { symbols::ResolveMode::Domestic };
            commands::analyze::run(&client, &symbol, mode, json, save, pick).await
        }
    }
}

async fn dispatch_dome_stock(action: DomeStockAction) -> Result<()> {
    use commands::stock::dome as d;
    let client = build_client()?;
    match action {
        DomeStockAction::Price { symbol, pick } => d::run_price(&client, &symbol, pick).await,
        DomeStockAction::Chart { symbol, period, pick } => {
            let p = period.chars().next().unwrap_or('D');
            d::run_chart(&client, &symbol, p, pick).await
        }
        DomeStockAction::Asking { symbol, pick } => d::run_asking(&client, &symbol, pick).await,
        DomeStockAction::Balance => d::run_balance(&client).await,
        DomeStockAction::Psbl => d::run_psbl(&client).await,
        DomeStockAction::Order { action } => match action {
            DomeOrderAction::Buy { symbol, qty, price, pick } => {
                d::run_order(&client, crate::api::domestic_stock::order_account::order_cash::Side::Buy, &symbol, qty, price, pick).await
            }
            DomeOrderAction::Sell { symbol, qty, price, pick } => {
                d::run_order(&client, crate::api::domestic_stock::order_account::order_cash::Side::Sell, &symbol, qty, price, pick).await
            }
            DomeOrderAction::Cancel { order_no, qty } => d::run_order_cancel(&client, &order_no, qty).await,
        },
        DomeStockAction::History => d::run_history(&client).await,
        DomeStockAction::Watch { symbol, pick } => d::run_watch(&client, &symbol, pick).await,
    }
}

async fn dispatch_usa_stock(action: UsaStockAction) -> Result<()> {
    use commands::stock::usa as u;
    use crate::api::overseas_stock::order_account::order::Side;
    let client = build_client()?;
    match action {
        UsaStockAction::Price { symbol, pick } => u::run_price(&client, &symbol, pick).await,
        UsaStockAction::Chart { symbol, period, pick } => {
            let p = period.chars().next().unwrap_or('D');
            u::run_chart(&client, &symbol, p, pick).await
        }
        UsaStockAction::Balance { exchange } => u::run_balance(&client, &exchange).await,
        UsaStockAction::Order { action } => match action {
            UsaOrderAction::Buy { symbol, qty, price, pick } => {
                u::run_order(&client, Side::Buy, &symbol, qty, price, pick).await
            }
            UsaOrderAction::Sell { symbol, qty, price, pick } => {
                u::run_order(&client, Side::Sell, &symbol, qty, price, pick).await
            }
        },
        UsaStockAction::History { exchange } => u::run_history(&client, &exchange).await,
        UsaStockAction::Watch { symbol, pick } => u::run_watch(&client, &symbol, pick).await,
    }
}

async fn dispatch_bond(action: BondAction) -> Result<()> {
    use commands::bond as b;
    let client = build_client()?;
    match action {
        BondAction::Price { symbol } => b::run_price(&client, &symbol).await,
        BondAction::Chart { symbol } => b::run_chart(&client, &symbol).await,
        BondAction::Balance => b::run_balance(&client).await,
        BondAction::Order { action } => match action {
            BondOrderAction::Buy { symbol, qty, price } => b::run_buy(&client, &symbol, qty, price).await,
            BondOrderAction::Sell { symbol, qty, price } => b::run_sell(&client, &symbol, qty, price).await,
        },
    }
}

async fn dispatch_dome_fo(action: DomeFoAction) -> Result<()> {
    use commands::fo::dome as f;
    let client = build_client()?;
    match action {
        DomeFoAction::Price { symbol, market } => f::run_price(&client, &symbol, market.as_deref()).await,
        DomeFoAction::Balance => f::run_balance(&client).await,
        DomeFoAction::Order { action } => match action {
            DomeFoOrderAction::Buy { symbol, qty, price } => {
                f::run_order(&client, f::Side::Buy, &symbol, qty, price).await
            }
            DomeFoOrderAction::Sell { symbol, qty, price } => {
                f::run_order(&client, f::Side::Sell, &symbol, qty, price).await
            }
        },
        DomeFoAction::WatchNight { symbol, pick } => f::run_watch_night(&client, &symbol, pick).await,
    }
}

async fn dispatch_usa_fo(action: UsaFoAction) -> Result<()> {
    use commands::fo::usa as f;
    let client = build_client()?;
    match action {
        UsaFoAction::Price { symbol } => f::run_price(&client, &symbol).await,
        UsaFoAction::Balance { currency } => f::run_balance(&client, &currency).await,
        UsaFoAction::Order { action } => match action {
            UsaFoOrderAction::Buy { symbol, qty, price } => {
                f::run_order(&client, f::Side::Buy, &symbol, qty, price).await
            }
            UsaFoOrderAction::Sell { symbol, qty, price } => {
                f::run_order(&client, f::Side::Sell, &symbol, qty, price).await
            }
        },
    }
}

fn config_init() -> Result<()> {
    let path = config::config_path()?;
    if path.exists() {
        print!("설정 파일이 이미 존재합니다. 덮어쓸까요? (y/N): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("취소됨");
            return Ok(());
        }
    }

    print!("APP_KEY: ");
    io::stdout().flush()?;
    let mut app_key = String::new();
    io::stdin().read_line(&mut app_key)?;

    print!("APP_SECRET: ");
    io::stdout().flush()?;
    let mut app_secret = String::new();
    io::stdin().read_line(&mut app_secret)?;

    print!("계좌번호 (예: 00000000-01): ");
    io::stdout().flush()?;
    let mut account = String::new();
    io::stdin().read_line(&mut account)?;

    print!("텔레그램 BOT_TOKEN (signal-watch 알림용, 건너뛰려면 Enter): ");
    io::stdout().flush()?;
    let mut bot_token = String::new();
    io::stdin().read_line(&mut bot_token)?;
    let bot_token = bot_token.trim().to_string();
    let telegram = if bot_token.is_empty() {
        None
    } else {
        Some(config::TelegramConfig { bot_token, chat_id: String::new() })
    };
    let has_telegram = telegram.is_some();

    let cfg = config::AppConfig {
        credentials: config::Credentials {
            app_key: app_key.trim().to_string(),
            app_secret: app_secret.trim().to_string(),
            account_number: account.trim().to_string(),
        },
        is_mock: false,
        telegram,
    };

    config::save_config(&cfg)?;
    token::clear_cache_files();
    println!("\n설정 저장 완료: {}", path.display());
    if has_telegram {
        println!("\n텔레그램 chat_id 연동을 이어서 진행하세요:");
        println!("  kis config telegram");
    }
    Ok(())
}

async fn config_telegram() -> Result<()> {
    let mut cfg = config::load_config()?;
    let token = cfg
        .telegram
        .as_ref()
        .map(|t| t.bot_token.clone())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "[telegram] bot_token 이 설정돼 있지 않습니다.\nconfig.toml 에 아래 섹션을 먼저 추가하세요:\n\n[telegram]\nbot_token = \"<BotFather 에게 받은 토큰>\"\nchat_id = \"\""
            )
        })?;
    let http = reqwest::Client::new();

    // 1) getMe 로 봇 username 확인
    let me: serde_json::Value = http
        .get(format!("https://api.telegram.org/bot{token}/getMe"))
        .send()
        .await?
        .json()
        .await?;
    if me.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(anyhow::anyhow!(
            "getMe 실패 — bot_token 이 유효한지 확인하세요.\n응답: {}",
            me
        ));
    }
    let username = me
        .pointer("/result/username")
        .and_then(|v| v.as_str())
        .unwrap_or("(unknown)");

    println!("봇: @{}", username);
    println!("아래 링크를 열어 봇에게 /start (또는 아무 메시지) 를 보내세요:");
    println!("  https://t.me/{}", username);
    println!();
    println!("메시지 대기 중... (최대 60초)");

    // 2) getUpdates long polling
    let updates: serde_json::Value = http
        .get(format!("https://api.telegram.org/bot{token}/getUpdates"))
        .query(&[("timeout", "60")])
        .send()
        .await?
        .json()
        .await?;
    if updates.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err(anyhow::anyhow!("getUpdates 실패: {}", updates));
    }
    let results = updates
        .get("result")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("getUpdates 응답 파싱 실패"))?;

    // 가장 최근의 message.chat.id 를 채택
    let (chat_id, from_name) = results
        .iter()
        .rev()
        .find_map(|u| {
            let chat_id = u.pointer("/message/chat/id").and_then(|v| v.as_i64())?;
            let first = u
                .pointer("/message/from/first_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let user = u
                .pointer("/message/from/username")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let label = match (first.is_empty(), user.is_empty()) {
                (false, false) => format!("{first} (@{user})"),
                (false, true) => first.to_string(),
                (true, false) => format!("@{user}"),
                _ => "(unknown)".into(),
            };
            Some((chat_id.to_string(), label))
        })
        .ok_or_else(|| {
            anyhow::anyhow!(
                "60초 안에 메시지를 감지하지 못했습니다.\n봇에게 /start 를 먼저 보낸 뒤 다시 시도하세요."
            )
        })?;

    println!("감지됨: chat_id={} (보낸 사람: {})", chat_id, from_name);

    // 3) config.toml 업데이트
    cfg.telegram = Some(config::TelegramConfig {
        bot_token: token.clone(),
        chat_id: chat_id.clone(),
    });
    config::save_config(&cfg)?;
    println!("config.toml 업데이트 완료: {}", config::config_path()?.display());

    // 4) 테스트 메시지
    let resp = http
        .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": "✓ kis-cli 텔레그램 연동 완료. signal-watch 전이 신호가 여기로 전송됩니다.",
        }))
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "테스트 메시지 전송 실패 ({}): {}",
            status,
            body
        ));
    }
    println!("✓ 테스트 메시지 전송 완료. 텔레그램을 확인하세요.");
    Ok(())
}
