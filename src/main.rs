mod analysis;
mod api;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod symbols;
mod token;
mod viewer;
mod ws;

use std::io::{self, Write};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::client::KisClient;

#[derive(Parser)]
#[command(name = "kis", about = "한국투자증권 API CLI", version)]
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
        #[arg(long)]
        pick: Option<usize>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// 설정 파일 초기화
    Init,
    /// 현재 설정 경로 출력
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
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 차트 (D 일/ W 주/ M 월)
    Chart {
        symbol: String,
        #[arg(long, default_value = "D")]
        period: String,
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 호가
    Asking {
        symbol: String,
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
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 매도
    Sell {
        symbol: String,
        qty: u64,
        #[arg(long)]
        price: Option<u64>,
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
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 차트
    Chart {
        symbol: String,
        #[arg(long, default_value = "D")]
        period: String,
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
        #[arg(long)]
        pick: Option<usize>,
    },
    /// 매도 (지정가 필수)
    Sell {
        symbol: String,
        qty: u64,
        #[arg(long)]
        price: f64,
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
    if let Commands::Analyze { symbol, usa, chart: true, pick, .. } = &cli.command {
        return run_chart_viewer(symbol, *usa, *pick);
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    rt.block_on(async_main(cli))
}

fn run_chart_viewer(symbol: &str, usa: bool, pick: Option<usize>) -> Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let mode = if usa { symbols::ResolveMode::Overseas } else { symbols::ResolveMode::Domestic };

    let (title, html, client, sym_code, sym_name, series) = rt.block_on(async {
        let client = std::sync::Arc::new(build_client()?);
        let (sym, series, _report, html) =
            commands::analyze::prepare(&client, symbol, mode, pick).await?;
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
        },

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

    let cfg = config::AppConfig {
        credentials: config::Credentials {
            app_key: app_key.trim().to_string(),
            app_secret: app_secret.trim().to_string(),
            account_number: account.trim().to_string(),
        },
        is_mock: false,
    };

    config::save_config(&cfg)?;
    println!("\n설정 저장 완료: {}", path.display());
    Ok(())
}
