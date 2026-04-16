mod api;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod token;
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
    /// 국내주식 현재가 조회
    Price {
        /// 종목코드 (예: 005930)
        symbol: String,
    },
    /// 계좌 잔고 조회
    Balance,
    /// 실시간 체결가 스트리밍 (WebSocket)
    Watch {
        /// 종목코드 (예: 005930)
        symbol: String,
    },
    /// 설정 관리
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// 설정 파일 초기화
    Init,
    /// 현재 설정 경로 출력
    Path,
}

fn build_client() -> Result<KisClient> {
    let cfg = config::load_config()?;
    Ok(KisClient::with_mock(cfg.credentials, cfg.is_mock))
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Auth => {
            let client = build_client()?;
            commands::auth::run(&client).await
        }
        Commands::Price { symbol } => {
            let client = build_client()?;
            commands::price::run(&client, &symbol).await
        }
        Commands::Balance => {
            let client = build_client()?;
            commands::balance::run(&client).await
        }
        Commands::Watch { symbol } => {
            let client = build_client()?;
            commands::watch::run(&client, &symbol).await
        }
        Commands::Config { action } => match action {
            ConfigAction::Init => config_init(),
            ConfigAction::Path => {
                let path = config::config_path()?;
                println!("{}", path.display());
                Ok(())
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
