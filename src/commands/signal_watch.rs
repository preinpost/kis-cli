//! `kis signal-watch` — cron 스케줄에 맞춰 전략 신호를 주기적으로 로그로 남기는 감시 도구.
//!
//! **설계**
//! - `tokio-cron-scheduler` 로 cron 표현식에 따라 주기 실행
//! - 매 실행마다: 캔들 fetch → 최신 신호 → 현재 보유 수량 → **전이 판단** → **로그**
//! - 주문 실행 없음 (의도적). 자동 매매는 별도 커맨드로 분리 예정.
//!
//! **출력 예시**
//! ```
//! [2026-04-19 15:45:03] tick [005930] 삼성전자
//!   최신봉 20260419 / 종가 89,000원 / 신호 +1 (long)
//!   현재 보유: 0주
//!   → ▲ 진입 신호 (BUY 0주 권장)
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use chrono::Local;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::api::domestic_stock::order_account::inquire_balance as inquire_balance_domestic;
use crate::api::overseas_stock::order_account::inquire_balance as inquire_balance_overseas;
use crate::client::KisClient;
use crate::commands::backtest::{self, Params, StrategyKind};
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::config::{load_config, TelegramConfig};
use crate::symbols::{Market, ResolveMode};

pub struct Config {
    pub symbol: String,
    pub strategy: StrategyKind,
    pub cron: String,
    pub period: char,
    pub usa: bool,
    pub pick: Option<usize>,
    pub fast: Option<usize>,
    pub slow: Option<usize>,
    pub rsi_period: Option<usize>,
    pub rsi_oversold: Option<f64>,
    pub rsi_overbought: Option<f64>,
    pub bb_period: Option<usize>,
    pub bb_sigma: Option<f64>,
    pub obv_period: Option<usize>,
}

pub async fn run(client: Arc<KisClient>, cfg: Config) -> Result<()> {
    let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, mode, cfg.pick)?;
    let display_name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };

    let telegram = load_config().ok().and_then(|c| c.telegram).map(Arc::new);
    log_info(&format!(
        "signal-watch 시작: [{}] {} ({}) · 전략 {} · cron \"{}\" (감시 전용, 주문 없음){}",
        sym.code,
        display_name,
        sym.market.as_str(),
        strategy_label(&cfg),
        cfg.cron,
        if telegram.is_some() { " · 텔레그램 알림 활성" } else { "" },
    ));

    let cfg = Arc::new(cfg);
    let sym_code = sym.code.clone();
    let sym_market = sym.market;
    let sym_name = display_name;

    let mut sched = JobScheduler::new().await?;
    let client_cl = client.clone();
    let cfg_cl = cfg.clone();
    let sym_code_cl = sym_code.clone();
    let sym_name_cl = sym_name.clone();
    let telegram_cl = telegram.clone();

    let job = Job::new_async(cfg.cron.as_str(), move |_uuid, _l| {
        let client = client_cl.clone();
        let cfg = cfg_cl.clone();
        let code = sym_code_cl.clone();
        let name = sym_name_cl.clone();
        let telegram = telegram_cl.clone();
        Box::pin(async move {
            if let Err(e) = tick(&client, &cfg, &code, &name, sym_market, telegram.as_deref()).await {
                log_error(&format!("tick 실패: {e}"));
            }
        })
    })?;
    sched.add(job).await?;
    sched.start().await?;

    log_info("스케줄러 시작됨. 종료: Ctrl+C");

    tokio::signal::ctrl_c().await?;
    log_info("종료 신호 수신, 스케줄러 정리…");
    sched.shutdown().await.ok();
    Ok(())
}

async fn tick(
    client: &KisClient,
    cfg: &Config,
    code: &str,
    name: &str,
    market: Market,
    telegram: Option<&TelegramConfig>,
) -> Result<()> {
    log_info(&format!("── tick [{}] {} ──", code, name));

    let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let params = build_params(cfg);
    let series = backtest::fetch_series_range(
        client,
        code,
        mode,
        cfg.period,
        None,
        None,
    )
    .await?;
    if series.closes.len() < 30 {
        return Err(anyhow!("데이터 부족 ({}봉)", series.closes.len()));
    }
    let signal = backtest::latest_signal(&series, &params);
    let last_price = series.closes.last().copied().unwrap_or(f64::NAN);
    let last_date = series.dates.last().cloned().unwrap_or_default();
    let (price_str, unit) = if cfg.usa {
        (format!("{:.4}", last_price), "USD")
    } else {
        (format_number(&format!("{:.0}", last_price)), "원")
    };
    log_info(&format!(
        "  최신봉 {} / 종가 {}{} / 신호 {}",
        last_date,
        price_str,
        unit,
        signal_label(signal)
    ));

    let held = if cfg.usa {
        current_holding_qty_overseas(client, code, market).await?
    } else {
        current_holding_qty_domestic(client, code).await?
    };
    log_info(&format!("  현재 보유: {}주", held));

    let alert = classify(signal, held);
    match &alert {
        Alert::None => log_info("  → 변화 없음"),
        Alert::EntryRecommended => log_info("  → ▲ 진입 신호 (미보유 → long 전략)"),
        Alert::ExitRecommended => log_info(&format!(
            "  → ▼ 청산 신호 (보유 {}주 → flat 전략)", held
        )),
    }

    if let (Some(tg), Some(body)) = (
        telegram,
        build_alert_message(&alert, code, name, cfg, &last_date, &price_str, unit, held),
    ) {
        if let Err(e) = send_telegram(tg, &body).await {
            log_error(&format!("텔레그램 전송 실패: {e}"));
        } else {
            log_info("  → 텔레그램 전송 완료");
        }
    }
    Ok(())
}

fn build_alert_message(
    alert: &Alert,
    code: &str,
    name: &str,
    cfg: &Config,
    last_date: &str,
    price_str: &str,
    unit: &str,
    held: u64,
) -> Option<String> {
    let headline = match alert {
        Alert::None => return None,
        Alert::EntryRecommended => "🔔 진입 신호 (BUY 권장)",
        Alert::ExitRecommended => "🔔 청산 신호 (SELL 권장)",
    };
    Some(format!(
        "{}\n[{}] {}\n전략: {}\n최신봉: {} / {}{}\n보유: {}주\n({})",
        headline,
        code,
        name,
        strategy_label(cfg),
        last_date,
        price_str,
        unit,
        held,
        Local::now().format("%Y-%m-%d %H:%M:%S"),
    ))
}

async fn send_telegram(cfg: &TelegramConfig, text: &str) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", cfg.bot_token);
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": cfg.chat_id,
            "text": text,
        }))
        .send()
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!("HTTP {}: {}", status, body));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Alert {
    None,
    EntryRecommended,
    ExitRecommended,
}

fn classify(signal: i8, held: u64) -> Alert {
    if signal > 0 && held == 0 {
        Alert::EntryRecommended
    } else if signal <= 0 && held > 0 {
        Alert::ExitRecommended
    } else {
        Alert::None
    }
}

fn build_params(cfg: &Config) -> Params {
    Params {
        strategy: cfg.strategy,
        period: cfg.period,
        from: None,
        to: None,
        fee_bps: 5.0,
        slippage_bps: 0.0,
        allow_short: false,
        leverage: 1.0,
        stop_loss_pct: None,
        take_profit_pct: None,
        fast: cfg.fast,
        slow: cfg.slow,
        rsi_period: cfg.rsi_period,
        rsi_oversold: cfg.rsi_oversold,
        rsi_overbought: cfg.rsi_overbought,
        bb_period: cfg.bb_period,
        bb_sigma: cfg.bb_sigma,
        obv_period: cfg.obv_period,
        manual_entry_date: None,
        manual_exit_date: None,
        manual_direction: None,
    }
}

async fn current_holding_qty_domestic(client: &KisClient, code: &str) -> Result<u64> {
    let req = inquire_balance_domestic::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        afhr_flpr_yn: "N".into(),
        ofl_yn: "".into(),
        inqr_dvsn: "02".into(),
        unpr_dvsn: "01".into(),
        fund_sttl_icld_yn: "N".into(),
        fncg_amt_auto_rdpt_yn: "N".into(),
        prcs_dvsn: "01".into(),
        ctx_area_fk100: "".into(),
        ctx_area_nk100: "".into(),
    };
    let r = inquire_balance_domestic::call(client, &req).await?;
    for h in &r.holdings {
        if h.pdno == code {
            return Ok(h.hldg_qty.parse::<u64>().unwrap_or(0));
        }
    }
    Ok(0)
}

async fn current_holding_qty_overseas(client: &KisClient, code: &str, market: Market) -> Result<u64> {
    let excg = match market {
        Market::Nasdaq => "NASD",
        Market::Nyse => "NYSE",
        Market::Amex => "AMEX",
        _ => "NASD",
    };
    let req = inquire_balance_overseas::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: excg.into(),
        tr_crcy_cd: "USD".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    let r = inquire_balance_overseas::call(client, &req).await?;
    for h in &r.holdings {
        if h.ovrs_pdno.eq_ignore_ascii_case(code) {
            return Ok(h.ovrs_cblc_qty.parse::<u64>().unwrap_or(0));
        }
    }
    Ok(0)
}

fn signal_label(s: i8) -> &'static str {
    match s {
        1 => "+1 (long)",
        -1 => "-1 (short, long-only 에선 flat 취급)",
        _ => "0 (flat)",
    }
}

fn strategy_label(cfg: &Config) -> String {
    match cfg.strategy {
        StrategyKind::MaCross => format!(
            "ma-cross({}/{})",
            cfg.fast.unwrap_or(20),
            cfg.slow.unwrap_or(60)
        ),
        StrategyKind::Rsi => format!(
            "rsi({}, {:.0}/{:.0})",
            cfg.rsi_period.unwrap_or(14),
            cfg.rsi_oversold.unwrap_or(30.0),
            cfg.rsi_overbought.unwrap_or(70.0),
        ),
        StrategyKind::Macd => "macd(12/26/9)".into(),
        StrategyKind::Bollinger => format!(
            "bollinger({}, {}σ)",
            cfg.bb_period.unwrap_or(20),
            cfg.bb_sigma.unwrap_or(2.0),
        ),
        StrategyKind::Ichimoku => "ichimoku(9/26/52)".into(),
        StrategyKind::Obv => format!("obv({})", cfg.obv_period.unwrap_or(20)),
        StrategyKind::Manual => "manual (signal-watch 부적합)".into(),
        StrategyKind::Composite => "composite (signal-watch 부적합)".into(),
    }
}

fn log_info(msg: &str) {
    eprintln!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
}

fn log_error(msg: &str) {
    eprintln!(
        "[{}] ERROR: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    );
}

// ─────────────────────────────────────────────────────────────
// `kis signal-watch all` — 모든 전략을 한 스케줄러에서 감시, 전이 시점만 텔레그램 전송.
// ─────────────────────────────────────────────────────────────

pub struct AllConfig {
    pub symbol: String,
    pub cron: Option<String>,
    pub usa: bool,
    pub pick: Option<usize>,
    pub background: bool,
}

pub async fn run_all(client: Arc<KisClient>, cfg: AllConfig) -> Result<()> {
    let mode = if cfg.usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let sym = resolve_symbol(&cfg.symbol, mode, cfg.pick)?;
    let display_name = if !sym.name_kr.is_empty() { sym.name_kr.clone() }
        else if !sym.name_en.is_empty() { sym.name_en.clone() }
        else { sym.code.clone() };
    let cron = cfg.cron.clone().unwrap_or_else(|| default_cron(cfg.usa).into());

    if cfg.background {
        return install_systemd_unit(&sym.code, cfg.usa, &cron, &display_name);
    }

    let telegram = load_config()
        .ok()
        .and_then(|c| c.telegram)
        .filter(|t| !t.bot_token.is_empty() && !t.chat_id.is_empty())
        .map(Arc::new);
    log_info(&format!(
        "signal-watch all 시작: [{}] {} ({}) · cron \"{}\"{}",
        sym.code,
        display_name,
        sym.market.as_str(),
        cron,
        if telegram.is_some() { " · 텔레그램 알림 활성 (전이 시점만)" } else { "" },
    ));

    let strategies = all_strategy_configs();
    let state: Arc<Mutex<HashMap<&'static str, Alert>>> = Arc::new(Mutex::new(HashMap::new()));
    let sym_code = sym.code.clone();
    let sym_market = sym.market;
    let sym_name = display_name;
    let usa = cfg.usa;

    let mut sched = JobScheduler::new().await?;
    let client_cl = client.clone();
    let code_cl = sym_code.clone();
    let name_cl = sym_name.clone();
    let state_cl = state.clone();
    let tg_cl = telegram.clone();
    let strategies_cl = Arc::new(strategies);

    let job = Job::new_async(cron.as_str(), move |_uuid, _l| {
        let client = client_cl.clone();
        let code = code_cl.clone();
        let name = name_cl.clone();
        let state = state_cl.clone();
        let tg = tg_cl.clone();
        let strategies = strategies_cl.clone();
        Box::pin(async move {
            if let Err(e) = tick_all(
                &client, &code, &name, usa, sym_market, &strategies, &state, tg.as_deref(),
            ).await {
                log_error(&format!("tick_all 실패: {e}"));
            }
        })
    })?;
    sched.add(job).await?;
    sched.start().await?;

    log_info("스케줄러 시작됨. 종료: Ctrl+C (systemd 는 SIGTERM)");
    if let Some(tg) = telegram.as_deref() {
        let body = format!(
            "▶ signal-watch all 시작\n[{}] {} ({})\ncron: {}\nhost: {}",
            sym_code, sym_name, sym_market.as_str(), cron, get_hostname(),
        );
        if let Err(e) = send_telegram(tg, &body).await {
            log_error(&format!("시작 알림 실패: {e}"));
        }
    }

    wait_for_shutdown_signal().await;
    log_info("종료 신호 수신, 스케줄러 정리…");

    if let Some(tg) = telegram.as_deref() {
        let body = format!(
            "■ signal-watch all 종료\n[{}] {} ({})\nhost: {}",
            sym_code, sym_name, sym_market.as_str(), get_hostname(),
        );
        if let Err(e) = send_telegram(tg, &body).await {
            log_error(&format!("종료 알림 실패: {e}"));
        }
    }

    sched.shutdown().await.ok();
    Ok(())
}

async fn wait_for_shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                log_error(&format!("SIGTERM 핸들러 등록 실패: {e} — Ctrl+C 만 대기"));
                let _ = tokio::signal::ctrl_c().await;
                return;
            }
        };
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = sigterm.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

fn get_hostname() -> String {
    std::process::Command::new("hostname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn default_cron(usa: bool) -> &'static str {
    if usa {
        // KST 기준 미장 — 서머타임은 사용자가 --cron 으로 조정
        "0 0 23,0-5 * * Tue-Sat"
    } else {
        "0 0 9-15 * * Mon-Fri"
    }
}

struct StrategyEntry {
    name: &'static str,
    params: Params,
}

fn all_strategy_configs() -> Vec<StrategyEntry> {
    let base = |strategy: StrategyKind| Params {
        strategy,
        period: 'D',
        from: None,
        to: None,
        fee_bps: 5.0,
        slippage_bps: 0.0,
        allow_short: false,
        leverage: 1.0,
        stop_loss_pct: None,
        take_profit_pct: None,
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
    };
    vec![
        StrategyEntry {
            name: "ma-cross(20/60)",
            params: Params { fast: Some(20), slow: Some(60), ..base(StrategyKind::MaCross) },
        },
        StrategyEntry {
            name: "rsi(14, 30/70)",
            params: Params {
                rsi_period: Some(14),
                rsi_oversold: Some(30.0),
                rsi_overbought: Some(70.0),
                ..base(StrategyKind::Rsi)
            },
        },
        StrategyEntry {
            name: "macd(12/26/9)",
            params: base(StrategyKind::Macd),
        },
        StrategyEntry {
            name: "bollinger(20, 2σ)",
            params: Params { bb_period: Some(20), bb_sigma: Some(2.0), ..base(StrategyKind::Bollinger) },
        },
        StrategyEntry {
            name: "ichimoku(9/26/52)",
            params: base(StrategyKind::Ichimoku),
        },
        StrategyEntry {
            name: "obv(20)",
            params: Params { obv_period: Some(20), ..base(StrategyKind::Obv) },
        },
    ]
}

async fn tick_all(
    client: &KisClient,
    code: &str,
    name: &str,
    usa: bool,
    market: Market,
    strategies: &[StrategyEntry],
    state: &Mutex<HashMap<&'static str, Alert>>,
    telegram: Option<&TelegramConfig>,
) -> Result<()> {
    log_info(&format!("── tick-all [{}] {} ──", code, name));

    let mode = if usa { ResolveMode::Overseas } else { ResolveMode::Domestic };
    let series = backtest::fetch_series_range(client, code, mode, 'D', None, None).await?;
    if series.closes.len() < 30 {
        return Err(anyhow!("데이터 부족 ({}봉)", series.closes.len()));
    }
    let last_price = series.closes.last().copied().unwrap_or(f64::NAN);
    let last_date = series.dates.last().cloned().unwrap_or_default();
    let (price_str, unit) = if usa {
        (format!("{:.4}", last_price), "USD")
    } else {
        (format_number(&format!("{:.0}", last_price)), "원")
    };
    log_info(&format!("  최신봉 {} / 종가 {}{}", last_date, price_str, unit));

    let held = if usa {
        current_holding_qty_overseas(client, code, market).await?
    } else {
        current_holding_qty_domestic(client, code).await?
    };
    log_info(&format!("  현재 보유: {}주", held));

    for entry in strategies {
        let signal = backtest::latest_signal(&series, &entry.params);
        let alert = classify(signal, held);
        let prev = state.lock().unwrap().get(entry.name).cloned();
        let changed = prev.as_ref() != Some(&alert);

        let tag = match alert {
            Alert::None => "변화 없음",
            Alert::EntryRecommended => "▲ 진입 신호",
            Alert::ExitRecommended => "▼ 청산 신호",
        };
        log_info(&format!(
            "  [{}] 신호 {} → {}{}",
            entry.name,
            signal_label(signal),
            tag,
            if changed && prev.is_some() { " (전이)" } else { "" },
        ));

        let should_notify = changed
            && matches!(alert, Alert::EntryRecommended | Alert::ExitRecommended);
        if should_notify {
            if let Some(tg) = telegram {
                let body = format!(
                    "🔔 [{}] {}\n[{}] {}\n최신봉: {} / {}{}\n보유: {}주\n({})",
                    entry.name,
                    tag,
                    code,
                    name,
                    last_date,
                    price_str,
                    unit,
                    held,
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                );
                if let Err(e) = send_telegram(tg, &body).await {
                    log_error(&format!("[{}] 텔레그램 전송 실패: {e}", entry.name));
                } else {
                    log_info(&format!("  [{}] 텔레그램 전송 완료", entry.name));
                }
            }
        }
        state.lock().unwrap().insert(entry.name, alert);
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────
// `--background` — systemd user unit 생성/등록 (Linux 전용)
// ─────────────────────────────────────────────────────────────

fn install_systemd_unit(code: &str, usa: bool, cron: &str, display_name: &str) -> Result<()> {
    // sudo 로 실행됐으면 SUDO_USER 가 원래 유저. 아니면 USER 그대로.
    let run_user = std::env::var("SUDO_USER")
        .ok()
        .or_else(|| std::env::var("USER").ok())
        .ok_or_else(|| anyhow!("$USER 를 읽을 수 없습니다"))?;

    let exe = std::env::current_exe()?;
    let mut args = vec!["signal-watch".to_string(), "all".to_string(), code.to_string()];
    if usa { args.push("--usa".to_string()); }
    args.push("--cron".to_string());
    args.push(format!("\"{}\"", cron));
    let exec_start = format!("{} {}", shell_escape(&exe.to_string_lossy()), args.join(" "));

    let service_name = format!(
        "kis-signal-watch-{}{}",
        code.to_lowercase(),
        if usa { "-usa" } else { "" }
    );
    let unit = format!(
        "[Unit]\n\
         Description=kis-cli signal-watch all — {display_name} ({code})\n\
         After=network-online.target\n\
         Wants=network-online.target\n\
         \n\
         [Service]\n\
         Type=simple\n\
         User={run_user}\n\
         Group={run_user}\n\
         ExecStart={exec_start}\n\
         Restart=on-failure\n\
         RestartSec=30\n\
         \n\
         [Install]\n\
         WantedBy=multi-user.target\n"
    );

    let unit_path = format!("/etc/systemd/system/{}.service", service_name);

    if !cfg!(target_os = "linux") {
        println!("─────────────────────────────────────────────");
        println!("⚠ systemd는 Linux 전용입니다. 아래 unit 파일을 VPS에 복사하세요.");
        println!("파일 경로: {}", unit_path);
        println!("─────────────────────────────────────────────");
        print!("{}", unit);
        println!("─────────────────────────────────────────────");
        println!("⚠ 위 ExecStart 에 박힌 경로는 *현재 맥 로컬 바이너리 경로* 입니다.");
        println!("  VPS 에서 `which kis` 결과로 교체한 뒤 설치하세요.");
        println!();
        println!("설치 절차 (VPS 에서, root 또는 sudo):");
        println!("  # 1) VPS 상 kis 바이너리 경로 확인");
        println!("  which kis");
        println!("  # 2) unit 파일 작성 (ExecStart 의 경로를 1) 결과로 교체)");
        println!("  sudo tee {} > /dev/null <<'EOF'", unit_path);
        print!("{}", unit);
        println!("EOF");
        println!("  # 3) 활성화");
        println!("  sudo systemctl daemon-reload");
        println!("  sudo systemctl enable --now {}", service_name);
        println!("  sudo journalctl -u {} -f     # 로그 확인", service_name);
        println!();
        println!("💡 VPS 에서 직접 `--background` 실행하는 쪽이 더 편합니다:");
        println!("  sudo $(which kis) signal-watch all {} {}--background",
            code, if usa { "--usa " } else { "" });
        println!("  ($(which kis) 로 감싸야 sudo 의 secure_path 제약을 우회)");
        return Ok(());
    }

    match std::fs::write(&unit_path, &unit) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!(
                "{} 에 쓰기 권한이 없습니다. 시스템 unit 설치는 루트 권한이 필요합니다.\n\
                 재실행: sudo $(which kis) signal-watch all {} {}--background\n\
                 ($(which kis) 로 감싸지 않으면 sudo 의 secure_path 가 ~/.cargo/bin 같은 유저 경로를 가려서 `kis` 를 못 찾거나 엉뚱한 바이너리를 집을 수 있음)",
                unit_path,
                code,
                if usa { "--usa " } else { "" },
            ));
        }
        Err(e) => return Err(anyhow!("{} 쓰기 실패: {e}", unit_path)),
    }
    log_info(&format!("systemd unit 생성: {}", unit_path));

    run_systemctl(&["daemon-reload"])?;
    run_systemctl(&["enable", "--now", &service_name])?;
    log_info(&format!("✓ {}.service 활성화 및 시작됨 (실행 유저: {})", service_name, run_user));
    println!();
    println!("ExecStart: {}", exec_start);
    warn_nonstandard_exe_path(&exe.to_string_lossy());
    println!();
    println!("로그 확인: sudo journalctl -u {} -f", service_name);
    println!("상태 확인: sudo systemctl status {}", service_name);
    println!("목록 확인: kis signal-watch list");
    println!("제거:      sudo $(which kis) signal-watch remove {} {}", code, if usa { "--usa" } else { "" });
    println!();
    println!("※ 이 바이너리 경로가 유지되어야 서비스가 뜹니다. 바이너리를 옮기거나 재빌드로 경로가 바뀌면");
    println!("   `sudo $(which kis) signal-watch all {} {}--background` 를 다시 실행해 unit 을 재생성하세요.",
        code, if usa { "--usa " } else { "" });
    Ok(())
}

fn warn_nonstandard_exe_path(path: &str) {
    let suspicious = path.contains("/target/debug/")
        || path.contains("/target/release/")
        || path.starts_with("/tmp/")
        || path.starts_with("/home/") && path.contains("/.cargo/");
    if suspicious {
        println!();
        println!("⚠ 바이너리 경로가 비표준 위치입니다 ({}).", path);
        println!("  프로덕션에선 `/usr/local/bin/kis` 같은 고정 경로로 설치한 뒤 `--background` 를 돌리길 권장합니다.");
    }
}

fn run_systemctl(args: &[&str]) -> Result<()> {
    let status = std::process::Command::new("systemctl")
        .args(args)
        .status()
        .map_err(|e| anyhow!("systemctl 실행 실패: {e}"))?;
    if !status.success() {
        return Err(anyhow!("systemctl {:?} 실패 (exit {:?})", args, status.code()));
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────
// `kis signal-watch list` / `kis signal-watch remove <target>`
// ─────────────────────────────────────────────────────────────

const UNIT_DIR: &str = "/etc/systemd/system";
const UNIT_PREFIX: &str = "kis-signal-watch-";

pub fn list_services() -> Result<()> {
    let dir = std::path::Path::new(UNIT_DIR);
    if !dir.exists() {
        println!("(등록된 서비스 없음 — {} 가 없습니다. Linux 전용 기능)", UNIT_DIR);
        return Ok(());
    }
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(UNIT_PREFIX) && n.ends_with(".service"))
                .unwrap_or(false)
        })
        .collect();
    if files.is_empty() {
        println!("(등록된 kis-signal-watch 서비스 없음)");
        return Ok(());
    }
    files.sort();

    for path in files {
        let service_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let description = extract_field(&content, "Description=").unwrap_or_default();
        let exec_start = extract_field(&content, "ExecStart=").unwrap_or_default();
        let run_user = extract_field(&content, "User=").unwrap_or_default();
        let active = systemctl_query(&["is-active", &service_name]);
        let enabled = systemctl_query(&["is-enabled", &service_name]);

        println!("● {}.service", service_name);
        if !description.is_empty() {
            println!("    Description: {}", description);
        }
        println!("    Status:      active={} / enabled={} / user={}", active, enabled, run_user);
        if !exec_start.is_empty() {
            println!("    ExecStart:   {}", exec_start);
        }
        println!("    Unit:        {}", path.display());
        println!();
    }
    println!("제거: sudo $(which kis) signal-watch remove <code> [--usa]");
    println!("로그: sudo journalctl -u <service-name> -f");
    Ok(())
}

fn extract_field(content: &str, prefix: &str) -> Option<String> {
    content
        .lines()
        .find(|l| l.starts_with(prefix))
        .map(|l| l[prefix.len()..].trim().to_string())
}

fn systemctl_query(args: &[&str]) -> String {
    std::process::Command::new("systemctl")
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

pub fn remove_service(target: &str, usa: bool) -> Result<()> {
    let service_name = resolve_service_name(target, usa);
    let unit_path = format!("{}/{}.service", UNIT_DIR, service_name);

    if !std::path::Path::new(&unit_path).exists() {
        return Err(anyhow!(
            "서비스 파일이 없습니다: {}\n`kis signal-watch list` 로 등록된 서비스를 먼저 확인하세요.",
            unit_path
        ));
    }

    // stop + disable (실패해도 파일 삭제까지는 진행)
    if let Err(e) = run_systemctl(&["disable", "--now", &service_name]) {
        log_error(&format!("disable --now 실패 (무시하고 파일 삭제 시도): {e}"));
    }

    match std::fs::remove_file(&unit_path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!(
                "{} 삭제 권한이 없습니다. 재실행: sudo $(which kis) signal-watch remove {} {}",
                unit_path,
                target,
                if usa { "--usa" } else { "" }
            ));
        }
        Err(e) => return Err(anyhow!("{} 삭제 실패: {e}", unit_path)),
    }

    run_systemctl(&["daemon-reload"])?;
    log_info(&format!("✓ {}.service 제거됨 ({} 삭제)", service_name, unit_path));
    Ok(())
}

fn resolve_service_name(target: &str, usa: bool) -> String {
    // 전체 서비스명을 직접 넘긴 경우: `kis-signal-watch-xxx` 또는 `.service` 포함
    let stripped = target.strip_suffix(".service").unwrap_or(target);
    if stripped.starts_with(UNIT_PREFIX) {
        return stripped.to_string();
    }
    // 코드만 넘긴 경우: prefix + 소문자 + (옵션) -usa
    format!(
        "{}{}{}",
        UNIT_PREFIX,
        stripped.to_lowercase(),
        if usa { "-usa" } else { "" }
    )
}

fn shell_escape(s: &str) -> String {
    if s.chars().all(|c| c.is_ascii_alphanumeric() || "/._-".contains(c)) {
        s.to_string()
    } else {
        format!("\"{}\"", s.replace('"', "\\\""))
    }
}
