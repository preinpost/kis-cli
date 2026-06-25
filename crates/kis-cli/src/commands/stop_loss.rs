//! 자동 손절 데몬.
//!
//! 주기적으로 잔고를 조회해서 평가손익률이 임계치 이하인 종목을 시장가(국내)
//! 또는 공격적 지정가(해외)로 매도한다. 같은 세션에서 이미 매도한 종목은 재트리거
//! 하지 않는다.
//!
//! 주의: `--execute` 없으면 dry-run (주문 안 나감, 로그만).

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{error, info};

use crate::api::domestic_stock::order_account::{inquire_balance as dome_bal, order_cash};
use crate::api::overseas_stock::order_account::{inquire_balance as usa_bal, order as usa_order};
use crate::client::KisClient;
use crate::commands::helpers::format_number;
use crate::config;
use crate::ws::aes_cbc_decrypt;

pub struct Config {
    pub threshold_pct: f64,
    pub interval_secs: u64,
    pub symbols: Option<Vec<String>>,
    pub execute: bool,
    pub usa_spread_pct: f64,
    pub use_ws: bool,
}

pub async fn run(client: &KisClient, cfg: Config) -> Result<()> {
    let _log_guard = crate::logging::init_daemon("stop-loss")?;
    print_banner(&cfg);
    if cfg.use_ws {
        run_ws(client, &cfg).await
    } else {
        run_polling(client, cfg).await
    }
}

async fn run_polling(client: &KisClient, cfg: Config) -> Result<()> {
    let mut sold: HashSet<String> = HashSet::new();
    let mut iter = 0u64;
    let started_at = chrono::Local::now().to_rfc3339();

    loop {
        iter += 1;
        let now = chrono::Local::now().format("%H:%M:%S").to_string();
        let mut aggregated: Vec<(String, SnapshotRow)> = Vec::new();

        match check_domestic(client, &cfg, &mut sold, &now).await {
            Ok(snap) => {
                print_snapshot(&now, "국내", &snap, cfg.threshold_pct);
                for r in snap.holdings { aggregated.push(("국내".into(), r)); }
            }
            Err(e) => error!("국내 조회 실패: {e}"),
        }

        for excg in ["NASD", "NYSE", "AMEX"] {
            match check_overseas(client, &cfg, &mut sold, &now, excg).await {
                Ok(snap) => {
                    if !snap.holdings.is_empty() {
                        print_snapshot(&now, excg, &snap, cfg.threshold_pct);
                    }
                    for r in snap.holdings { aggregated.push((excg.into(), r)); }
                }
                Err(e) => error!("{} 조회 실패: {e}", excg),
            }
        }

        // status 파일 — 폴링 iteration마다 덤프
        write_polling_status(&cfg, &started_at, &aggregated);

        if iter == 1 {
            println!();
            println!("감시 중... (Ctrl+C 로 중단)");
        }

        tokio::time::sleep(Duration::from_secs(cfg.interval_secs)).await;
    }
}

fn print_banner(cfg: &Config) {
    println!("═══════════════════════════════════════════════");
    println!("  자동 손절 데몬");
    println!("═══════════════════════════════════════════════");
    println!("  임계치:     {:+.2}%", cfg.threshold_pct);
    println!("  감시:       {}", if cfg.use_ws { "WebSocket (tick 단위)".to_string() }
        else { format!("폴링 {}초 주기", cfg.interval_secs) });
    println!("  대상:       {}", cfg.symbols.as_ref()
        .map(|s| s.join(", ")).unwrap_or("전체".into()));
    println!("  모드:       {}", if cfg.execute { "\x1b[31m실거래\x1b[0m" } else { "DRY-RUN" });
    if !cfg.execute {
        println!();
        println!("  (실제 매도를 원하면 --execute 추가)");
    } else {
        println!("  해외 스프레드: {:.2}% (현재가에서 낮춰 지정가 주문)", cfg.usa_spread_pct);
    }
    println!("═══════════════════════════════════════════════");
    println!();
}

#[derive(Default)]
struct Snapshot {
    holdings: Vec<SnapshotRow>,
}

struct SnapshotRow {
    code: String,
    name: String,
    qty: u64,
    pnl_rate: f64,
    triggered: bool,
    sold: bool,
    error: Option<String>,
}

fn print_snapshot(now: &str, label: &str, snap: &Snapshot, threshold: f64) {
    if snap.holdings.is_empty() { return; }
    for r in &snap.holdings {
        let badge = if r.sold { "\x1b[31m✓매도\x1b[0m" }
            else if r.error.is_some() { "\x1b[33m!실패\x1b[0m" }
            else if r.triggered { "\x1b[33m→트리거\x1b[0m" }
            else if r.pnl_rate <= threshold + 1.0 { "\x1b[33m⚠근접\x1b[0m" }
            else { "" };
        println!(
            "[{now}] {:<5} {:<12} {:<14} {:>6}주 {:>+7.2}% {}",
            label, r.code, truncate(&r.name, 14), r.qty, r.pnl_rate, badge
        );
        if let Some(e) = &r.error {
            println!("         ↳ {}", e);
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= max { s.to_string() } else { c[..max].iter().collect() }
}

async fn check_domestic(
    client: &KisClient,
    cfg: &Config,
    sold: &mut HashSet<String>,
    _now: &str,
) -> Result<Snapshot> {
    let req = dome_bal::Request {
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
    let r = dome_bal::call(client, &req).await?;
    let mut snap = Snapshot::default();
    for h in r.holdings {
        if !symbol_match(&cfg.symbols, &h.pdno, &h.prdt_name) { continue; }
        let qty: u64 = h.ord_psbl_qty.trim().parse().unwrap_or(0);
        if qty == 0 { continue; }
        let rate: f64 = h.evlu_pfls_rt.trim().parse().unwrap_or(0.0);
        let mut row = SnapshotRow {
            code: h.pdno.clone(),
            name: h.prdt_name.clone(),
            qty,
            pnl_rate: rate,
            triggered: false,
            sold: false,
            error: None,
        };
        if rate <= cfg.threshold_pct && !sold.contains(&h.pdno) {
            row.triggered = true;
            if cfg.execute {
                match sell_domestic(client, &h.pdno, qty).await {
                    Ok(odno) => {
                        row.sold = true;
                        sold.insert(h.pdno.clone());
                        info!(
                            "▶ 매도 국내 {} ({}) {}주 시장가 / 손익률 {:+.2}% / 주문번호 {}",
                            h.pdno, h.prdt_name, qty, rate, odno
                        );
                    }
                    Err(e) => row.error = Some(e.to_string()),
                }
            } else {
                info!(
                    "◇ DRY-RUN 국내 {} ({}) {}주 시장가 매도 예정 (손익률 {:+.2}%)",
                    h.pdno, h.prdt_name, qty, rate
                );
            }
        }
        snap.holdings.push(row);
    }
    Ok(snap)
}

async fn check_overseas(
    client: &KisClient,
    cfg: &Config,
    sold: &mut HashSet<String>,
    _now: &str,
    excg: &str,
) -> Result<Snapshot> {
    let req = usa_bal::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: excg.into(),
        tr_crcy_cd: "USD".into(),
        ctx_area_fk200: "".into(),
        ctx_area_nk200: "".into(),
    };
    let r = usa_bal::call(client, &req).await?;
    let mut snap = Snapshot::default();
    for h in r.holdings {
        if !symbol_match(&cfg.symbols, &h.ovrs_pdno, &h.ovrs_item_name) { continue; }
        let qty: u64 = h.ord_psbl_qty.trim().parse().unwrap_or(0);
        if qty == 0 { continue; }
        let rate: f64 = h.evlu_pfls_rt.trim().parse().unwrap_or(0.0);
        let now_price: f64 = h.now_pric2.trim().parse().unwrap_or(0.0);
        let mut row = SnapshotRow {
            code: h.ovrs_pdno.clone(),
            name: h.ovrs_item_name.clone(),
            qty,
            pnl_rate: rate,
            triggered: false,
            sold: false,
            error: None,
        };
        if rate <= cfg.threshold_pct && !sold.contains(&h.ovrs_pdno) {
            row.triggered = true;
            if now_price <= 0.0 {
                row.error = Some("현재가 알 수 없음 — 해외 지정가 산출 불가".into());
            } else {
                let limit = now_price * (1.0 - cfg.usa_spread_pct / 100.0);
                if cfg.execute {
                    match sell_overseas(client, &h.ovrs_pdno, excg, qty, limit).await {
                        Ok(odno) => {
                            row.sold = true;
                            sold.insert(h.ovrs_pdno.clone());
                            info!(
                                "▶ 매도 {} {} ({}) {}주 지정가 ${:.4} / 손익률 {:+.2}% / 주문번호 {}",
                                excg, h.ovrs_pdno, h.ovrs_item_name, qty, limit, rate, odno
                            );
                        }
                        Err(e) => row.error = Some(e.to_string()),
                    }
                } else {
                    info!(
                        "◇ DRY-RUN {} {} ({}) {}주 지정가 ${:.4} 매도 예정 (손익률 {:+.2}%)",
                        excg, h.ovrs_pdno, h.ovrs_item_name, qty, limit, rate
                    );
                }
            }
        }
        snap.holdings.push(row);
    }
    Ok(snap)
}

async fn sell_domestic(client: &KisClient, code: &str, qty: u64) -> Result<String> {
    let req = order_cash::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        pdno: code.into(),
        sll_type: Some("01".into()),
        ord_dvsn: "01".into(), // 시장가
        ord_qty: qty.to_string(),
        ord_unpr: "0".into(),
        cndt_pric: None,
        excg_id_dvsn_cd: None,
    };
    let r = order_cash::call(client, order_cash::Side::Sell, &req).await?;
    Ok(r.odno)
}

async fn sell_overseas(
    client: &KisClient,
    code: &str,
    excg: &str,
    qty: u64,
    limit: f64,
) -> Result<String> {
    let req = usa_order::Request {
        cano: client.cano().into(),
        acnt_prdt_cd: client.product_code().into(),
        ovrs_excg_cd: excg.into(),
        pdno: code.into(),
        ord_qty: qty.to_string(),
        ovrs_ord_unpr: format!("{:.4}", limit),
        ord_svr_dvsn_cd: "0".into(),
        ord_dvsn: "00".into(),
    };
    let r = usa_order::call(client, usa_order::Market::Usa, usa_order::Side::Sell, &req).await?;
    Ok(r.odno)
}

fn symbol_match(filter: &Option<Vec<String>>, code: &str, name: &str) -> bool {
    match filter {
        None => true,
        Some(list) => list.iter().any(|s| {
            let s = s.trim();
            s.eq_ignore_ascii_case(code) || s == name || name.contains(s)
        }),
    }
}

// format_number는 import되어 있지만 현재 사용 안 함 — 추후 금액 표시에 사용 가능.
#[allow(dead_code)]
fn _unused() { let _ = format_number; }

// ============================================================================
// WebSocket 기반 실시간 감시 (V1)
// ============================================================================

const WS_URL: &str = "ws://ops.koreainvestment.com:21000/tryitout";

#[derive(Debug, Clone)]
struct Position {
    code: String,
    name: String,
    qty: u64,
    avg_price: f64,
    mode: PosMode,
    /// 해외만: 주문용 거래소 코드 (NASD/NYSE/AMEX). 국내는 빈 문자열.
    excg_order: String,
    triggered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PosMode {
    Domestic,
    /// Overseas — (excd: NAS/NYS/AMS)
    Overseas,
}

struct DecryptInfo { key: String, iv: String }

const SUB_LIMIT: usize = 40;
const REFRESH_INTERVAL_SECS: u64 = 600; // 10분
const RECONNECT_BASE_SECS: u64 = 2;

/// 전역 공유 상태 — reconnect·refresh 사이에 보존.
struct SharedState {
    positions: Vec<Position>,
    /// tr_key → position 인덱스 (fast lookup)
    key_map: HashMap<String, usize>,
    /// 이미 구독 메시지 보낸 tr_key (중복 구독 방지)
    subscribed: HashSet<String>,
    /// tr_key → 마지막 체결가 (status 덤프용)
    last_prices: HashMap<String, f64>,
    /// 데몬 시작 시각 (RFC3339)
    started_at: String,
    /// 마지막 status 파일 쓰기 시각 (throttle)
    last_status_write: Instant,
}

async fn run_ws(client: &KisClient, cfg: &Config) -> Result<()> {
    // 1. 초기 잔고 → 포지션 리스트
    let positions = fetch_initial_positions(client, &cfg.symbols).await?;
    if positions.is_empty() {
        println!("감시할 보유 종목이 없습니다.");
        return Ok(());
    }
    print_positions_header(&positions);

    // 2. SharedState 초기화
    let mut key_map = HashMap::new();
    for (i, p) in positions.iter().enumerate() {
        key_map.insert(position_tr_key(p), i);
    }
    let state = std::sync::Arc::new(std::sync::Mutex::new(SharedState {
        positions,
        key_map,
        subscribed: HashSet::new(),
        last_prices: HashMap::new(),
        started_at: chrono::Local::now().to_rfc3339(),
        last_status_write: Instant::now() - Duration::from_secs(10),
    }));

    // 시작 직후 status 한 번 기록
    maybe_write_status(&state, cfg, true);

    // 3. reconnect 루프 (exp backoff 2,4,8,16,32 최대)
    let mut attempt: u32 = 0;
    loop {
        match run_ws_session(client, cfg, state.clone()).await {
            Ok(()) => {
                info!("[WS] 정상 종료");
                break;
            }
            Err(e) => {
                attempt = attempt.saturating_add(1);
                let delay = (RECONNECT_BASE_SECS << (attempt.min(5) - 1)).min(60);
                error!(
                    "[WS] 연결 끊김 (시도 {}회): {e} — {delay}초 후 재연결",
                    attempt
                );
                state.lock().unwrap().subscribed.clear();
                tokio::time::sleep(Duration::from_secs(delay)).await;

                if let Err(re) = refresh_positions_in_state(client, &cfg.symbols, &state).await {
                    error!("  잔고 갱신 실패 (이전 상태로 계속): {re}");
                }
            }
        }
    }
    Ok(())
}

/// 한 번의 WS 연결 생명주기. 연결 끊기거나 Close 오면 Err/Ok 반환.
async fn run_ws_session(
    client: &KisClient,
    cfg: &Config,
    state: std::sync::Arc<std::sync::Mutex<SharedState>>,
) -> Result<()> {
    let approval_key = client
        .token_manager
        .get_ws_approval_key_string()
        .await
        .context("WebSocket approval key 발급 실패")?;

    let (ws_stream, _) = connect_async(WS_URL).await.context("WebSocket 연결 실패")?;
    let (mut write, mut read) = ws_stream.split();

    // 초기 구독 (최대 SUB_LIMIT개)
    subscribe_pending(&mut write, &approval_key, &state).await?;

    let mut decrypt_info: Option<DecryptInfo> = None;
    let mut last_log: HashMap<usize, Instant> = HashMap::new();
    let log_throttle = Duration::from_secs(3);
    let mut refresh_ticker = tokio::time::interval(Duration::from_secs(REFRESH_INTERVAL_SECS));
    refresh_ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    refresh_ticker.tick().await; // 첫 tick은 즉시 — 스킵

    loop {
        tokio::select! {
            msg = read.next() => {
                let Some(msg) = msg else {
                    return Err(anyhow::anyhow!("WS 스트림 종료"));
                };
                let msg = msg.context("메시지 수신 오류")?;
                let text = match msg {
                    Message::Text(t) => t.to_string(),
                    Message::Ping(d) => { write.send(Message::Pong(d)).await.ok(); continue; }
                    Message::Close(_) => return Ok(()),
                    _ => continue,
                };
                handle_frame(
                    &text, &mut write, &mut decrypt_info, &mut last_log, log_throttle,
                    client, cfg, &state,
                ).await?;
            }
            _ = refresh_ticker.tick() => {
                if let Err(e) = refresh_positions_in_state(client, &cfg.symbols, &state).await {
                    error!("잔고 갱신 실패: {e}");
                } else {
                    if let Err(e) = subscribe_pending(&mut write, &approval_key, &state).await {
                        error!("  신규 구독 실패: {e}");
                    }
                    maybe_write_status(&state, cfg, true);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn handle_frame(
    text: &str,
    write: &mut futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    decrypt_info: &mut Option<DecryptInfo>,
    last_log: &mut HashMap<usize, Instant>,
    log_throttle: Duration,
    client: &KisClient,
    cfg: &Config,
    state: &std::sync::Arc<std::sync::Mutex<SharedState>>,
) -> Result<()> {
    let first = text.chars().next().unwrap_or(' ');
    if first == '0' || first == '1' {
        // 실시간 데이터
        let parts: Vec<&str> = text.splitn(4, '|').collect();
        if parts.len() < 4 { return Ok(()); }
        let encrypt_flag = parts[0];
        let tr_id = parts[1];
        let mut data_str = parts[3].to_string();
        if encrypt_flag == "1" {
            if let Some(info) = decrypt_info.as_ref() {
                if let Ok(d) = aes_cbc_decrypt(&info.key, &info.iv, &data_str) {
                    data_str = d;
                }
            }
        }
        let fields: Vec<&str> = data_str.split('^').collect();
        let Some((tr_key, price)) = parse_tick(tr_id, &fields) else { return Ok(()); };

        // 스냅샷 얻기 + 현재가 기록 (Mutex 빠르게 놓음)
        let snapshot = {
            let mut st = state.lock().unwrap();
            let Some(&idx) = st.key_map.get(&tr_key) else { return Ok(()); };
            st.last_prices.insert(tr_key.clone(), price);
            let pos = st.positions[idx].clone();
            (idx, pos)
        };
        let (idx, pos) = snapshot;
        if pos.triggered { return Ok(()); }
        if price <= 0.0 || pos.avg_price <= 0.0 { return Ok(()); }
        let pnl_rate = (price - pos.avg_price) / pos.avg_price * 100.0;

        let now = Instant::now();
        let should_log = last_log.get(&idx)
            .map(|t| now.duration_since(*t) >= log_throttle)
            .unwrap_or(true);
        if should_log {
            last_log.insert(idx, now);
            print_tick(&pos, price, pnl_rate, cfg.threshold_pct);
        }

        // 가격 갱신 → status 파일 (throttle 1초)
        maybe_write_status(state, cfg, false);

        if pnl_rate <= cfg.threshold_pct {
            // triggered 선점 (같은 연결 내 중복 트리거 방지)
            {
                let mut st = state.lock().unwrap();
                if let Some(p) = st.positions.get_mut(idx) {
                    if p.triggered { return Ok(()); }
                    p.triggered = true;
                }
            }
            if cfg.execute {
                let result = match pos.mode {
                    PosMode::Domestic => sell_domestic(client, &pos.code, pos.qty).await,
                    PosMode::Overseas => {
                        let limit = price * (1.0 - cfg.usa_spread_pct / 100.0);
                        sell_overseas(client, &pos.code, &pos.excg_order, pos.qty, limit).await
                    }
                };
                match result {
                    Ok(odno) => {
                        println!(
                            "[{}] \x1b[31m▶ 매도\x1b[0m {} ({}) {}주 @ {:.4} 손익률 {:+.2}% / 주문번호 {}",
                            chrono::Local::now().format("%H:%M:%S"),
                            pos.code, pos.name, pos.qty, price, pnl_rate, odno
                        );
                        maybe_write_status(state, cfg, true);
                    }
                    Err(e) => {
                        let mut st = state.lock().unwrap();
                        if let Some(p) = st.positions.get_mut(idx) { p.triggered = false; }
                        error!("  ✗ 매도 실패: {e} — 다음 tick에서 재시도");
                    }
                }
            } else {
                info!(
                    "◇ DRY-RUN {} ({}) {}주 @ {:.4} 손익률 {:+.2}% (매도 조건 충족)",
                    pos.code, pos.name, pos.qty, price, pnl_rate
                );
            }
        }
    } else {
        // 시스템 메시지
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(text) {
            let tr_id = val["header"]["tr_id"].as_str().unwrap_or("");
            if tr_id == "PINGPONG" {
                write.send(Message::Pong(text.as_bytes().to_vec().into())).await.ok();
                return Ok(());
            }
            if let Some(output) = val["body"]["output"].as_object() {
                if let (Some(k), Some(iv)) = (
                    output.get("key").and_then(|v| v.as_str()),
                    output.get("iv").and_then(|v| v.as_str()),
                ) {
                    *decrypt_info = Some(DecryptInfo { key: k.into(), iv: iv.into() });
                }
            }
            let rt_cd = val["body"]["rt_cd"].as_str().unwrap_or("");
            let msg1 = val["body"]["msg1"].as_str().unwrap_or("");
            if rt_cd == "0" {
                info!("  ✓ 구독 성공 {tr_id}: {msg1}");
            } else if !rt_cd.is_empty() {
                error!("  ✗ 구독 실패 {tr_id}: {msg1}");
            }
        }
    }
    Ok(())
}

/// state에 있는 포지션 중 아직 구독 안 된 것들을 subscribe.
async fn subscribe_pending(
    write: &mut futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    approval_key: &str,
    state: &std::sync::Arc<std::sync::Mutex<SharedState>>,
) -> Result<()> {
    // 스냅샷
    let to_subscribe: Vec<(String, String)> = {
        let st = state.lock().unwrap();
        let existing = st.subscribed.clone();
        let mut list: Vec<(String, String)> = Vec::new();
        for p in &st.positions {
            let (tr_id, tr_key) = tr_spec(p);
            if existing.contains(&tr_key) { continue; }
            if existing.len() + list.len() >= SUB_LIMIT {
                break;
            }
            list.push((tr_id.to_string(), tr_key));
        }
        list
    };

    if to_subscribe.is_empty() { return Ok(()); }

    for (tr_id, tr_key) in &to_subscribe {
        let msg = serde_json::json!({
            "header": {
                "approval_key": approval_key,
                "custtype": "P",
                "tr_type": "1",
                "content-type": "utf-8",
            },
            "body": { "input": { "tr_id": tr_id, "tr_key": tr_key } },
        });
        write.send(Message::Text(msg.to_string().into())).await
            .context("구독 전송 실패")?;
    }

    // 성공한 것들 subscribed에 추가
    {
        let mut st = state.lock().unwrap();
        for (_, tr_key) in to_subscribe.iter() {
            st.subscribed.insert(tr_key.clone());
        }
        let total = st.positions.len();
        let limited = st.subscribed.len();
        println!(
            "[{}] 구독 +{} (총 {}/{})",
            chrono::Local::now().format("%H:%M:%S"),
            to_subscribe.len(), limited, total
        );
        if total > SUB_LIMIT {
            tracing::warn!(
                "  ⚠ 구독 한도 {SUB_LIMIT} 초과 — {}개 종목은 감시 대상에서 제외됨",
                total - SUB_LIMIT
            );
        }
    }
    Ok(())
}

/// 잔고 재조회 → state 갱신. 신규 포지션은 subscribed에 없어 다음 subscribe_pending에서 구독됨.
async fn refresh_positions_in_state(
    client: &KisClient,
    filter: &Option<Vec<String>>,
    state: &std::sync::Arc<std::sync::Mutex<SharedState>>,
) -> Result<()> {
    let new_positions = fetch_initial_positions(client, filter).await?;
    let new_keys: HashSet<String> = new_positions.iter().map(position_tr_key).collect();

    let mut st = state.lock().unwrap();
    let old_keys: HashSet<String> = st.positions.iter().map(position_tr_key).collect();
    let added: usize = new_keys.difference(&old_keys).count();
    let removed: usize = old_keys.difference(&new_keys).count();

    // triggered 상태 보존: 동일 key가 new에 있으면 old의 triggered 유지
    let mut merged: Vec<Position> = Vec::with_capacity(new_positions.len());
    for np in new_positions {
        let np_key = position_tr_key(&np);
        let triggered = st
            .positions
            .iter()
            .find(|op| position_tr_key(op) == np_key)
            .map(|op| op.triggered)
            .unwrap_or(false);
        merged.push(Position { triggered, ..np });
    }

    // 제거된 포지션의 subscribed 항목 정리 (서버에 unsubscribe 보내지는 않음 — 연결 종료 시 자동 해제)
    st.subscribed.retain(|k| new_keys.contains(k));

    st.positions = merged;
    let new_key_map: HashMap<String, usize> = st
        .positions
        .iter()
        .enumerate()
        .map(|(i, p)| (position_tr_key(p), i))
        .collect();
    st.key_map = new_key_map;

    if added > 0 || removed > 0 {
        println!(
            "[{}] 포지션 갱신: +{added} / -{removed} (총 {})",
            chrono::Local::now().format("%H:%M:%S"),
            st.positions.len()
        );
    }
    Ok(())
}

fn position_tr_key(p: &Position) -> String {
    match p.mode {
        PosMode::Domestic => p.code.clone(),
        PosMode::Overseas => format!("D{}{}", order_excg_to_ws(&p.excg_order), p.code),
    }
}

// ============================================================================
// 상태 파일 (데몬이 쓰고, `kis stop-loss status`가 읽음)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatusSnapshot {
    pid: u32,
    started_at: String,
    updated_at: String,
    mode: String, // "ws" | "polling"
    execute: bool,
    threshold: f64,
    symbols_filter: Option<Vec<String>>,
    positions: Vec<StatusPosition>,
    triggered_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatusPosition {
    code: String,
    name: String,
    market: String,
    qty: u64,
    avg_price: f64,
    last_price: Option<f64>,
    pnl_rate: Option<f64>,
    triggered: bool,
}

fn write_status_atomic(snap: &StatusSnapshot) -> Result<()> {
    let path = config::stoploss_status_path()?;
    if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }
    let tmp: PathBuf = path.with_extension("json.tmp");
    let json = serde_json::to_string_pretty(snap)?;
    std::fs::write(&tmp, json)?;
    std::fs::rename(&tmp, &path)?;
    Ok(())
}

fn build_snapshot(
    cfg: &Config,
    started_at: &str,
    positions: &[Position],
    last_prices: &HashMap<String, f64>,
) -> StatusSnapshot {
    let pid = std::process::id();
    let updated_at = chrono::Local::now().to_rfc3339();
    let mut triggered_count = 0usize;
    let positions: Vec<StatusPosition> = positions.iter().map(|p| {
        if p.triggered { triggered_count += 1; }
        let key = position_tr_key(p);
        let last_price = last_prices.get(&key).copied();
        let pnl_rate = last_price.map(|lp| {
            if p.avg_price > 0.0 { (lp - p.avg_price) / p.avg_price * 100.0 } else { 0.0 }
        });
        StatusPosition {
            code: p.code.clone(),
            name: p.name.clone(),
            market: match p.mode {
                PosMode::Domestic => "국내".into(),
                PosMode::Overseas => p.excg_order.clone(),
            },
            qty: p.qty,
            avg_price: p.avg_price,
            last_price,
            pnl_rate,
            triggered: p.triggered,
        }
    }).collect();
    StatusSnapshot {
        pid,
        started_at: started_at.into(),
        updated_at,
        mode: if cfg.use_ws { "ws".into() } else { "polling".into() },
        execute: cfg.execute,
        threshold: cfg.threshold_pct,
        symbols_filter: cfg.symbols.clone(),
        positions,
        triggered_count,
    }
}

/// `kis stop-loss status` — 상태 파일 읽어 출력.
pub fn run_status() -> Result<()> {
    let path = config::stoploss_status_path()?;
    if !path.exists() {
        println!("실행 중인 데몬 없음 ({} 파일 없음).", path.display());
        println!("`kis stop-loss run --ws --execute` 등으로 시작.");
        return Ok(());
    }
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("상태 파일 읽기 실패: {}", path.display()))?;
    let snap: StatusSnapshot = serde_json::from_str(&text)
        .with_context(|| "상태 파일 JSON 파싱 실패")?;

    let alive = is_pid_alive(snap.pid);
    let updated = chrono::DateTime::parse_from_rfc3339(&snap.updated_at).ok();
    let staleness = updated.map(|u| {
        let now = chrono::Local::now();
        let diff = now.signed_duration_since(u.with_timezone(&chrono::Local));
        diff.num_seconds()
    });

    println!("═══════════════════════════════════════════════");
    println!("  자동 손절 데몬 상태");
    println!("═══════════════════════════════════════════════");
    println!("  PID:        {} ({})", snap.pid,
        if alive { "\x1b[32m실행 중\x1b[0m" } else { "\x1b[31m종료됨/응답 없음\x1b[0m" });
    println!("  시작:       {}", snap.started_at);
    println!("  마지막 갱신: {}{}", snap.updated_at,
        staleness.map(|s| format!(" ({}초 전)", s)).unwrap_or_default());
    println!("  감시 방식:   {}", snap.mode);
    println!("  실거래:     {}", if snap.execute { "\x1b[31mYES\x1b[0m" } else { "dry-run" });
    println!("  임계치:     {:+.2}%", snap.threshold);
    if let Some(f) = &snap.symbols_filter {
        println!("  대상 필터:  {}", f.join(", "));
    }
    println!("  종목 수:    {} (트리거됨: {})", snap.positions.len(), snap.triggered_count);
    println!();
    if snap.positions.is_empty() {
        println!("  보유 종목 없음");
    } else {
        println!(
            "  {:<5} {:<10} {:<14} {:>6} {:>12} {:>12} {:>9} {}",
            "시장", "코드", "종목명", "수량", "평균단가", "현재가", "손익률", "상태"
        );
        println!("  {}", "─".repeat(85));
        for p in &snap.positions {
            let price_str = p.last_price.map(|v|
                if v >= 1000.0 { format_number(&format!("{:.0}", v)) } else { format!("{:.4}", v) }
            ).unwrap_or_else(|| "-".into());
            let pnl_str = p.pnl_rate.map(|r| format!("{:+.2}%", r)).unwrap_or_else(|| "-".into());
            let badge = if p.triggered { "\x1b[31m매도\x1b[0m" }
                else if p.pnl_rate.map(|r| r <= snap.threshold).unwrap_or(false) { "\x1b[33m→트리거\x1b[0m" }
                else if p.pnl_rate.map(|r| r <= snap.threshold + 1.0).unwrap_or(false) { "\x1b[33m⚠\x1b[0m" }
                else { "" };
            let avg_str = if p.avg_price >= 1000.0 { format_number(&format!("{:.0}", p.avg_price)) }
                else { format!("{:.4}", p.avg_price) };
            println!(
                "  {:<5} {:<10} {:<14} {:>6} {:>12} {:>12} {:>9} {}",
                p.market,
                p.code,
                truncate(&p.name, 14),
                p.qty,
                avg_str,
                price_str,
                pnl_str,
                badge,
            );
        }
    }
    println!();
    if !alive {
        println!("\x1b[33m⚠ 프로세스가 살아있지 않음 — 위 정보는 마지막 갱신 시점 기준.\x1b[0m");
    }
    Ok(())
}

pub fn run_path() -> Result<()> {
    let path = config::stoploss_status_path()?;
    println!("{}", path.display());
    if !path.exists() {
        println!("(파일 없음 — 데몬이 한 번도 실행되지 않았거나 경로 변경됨)");
    }
    Ok(())
}

fn write_polling_status(cfg: &Config, started_at: &str, rows: &[(String, SnapshotRow)]) {
    let mut triggered_count = 0usize;
    let positions: Vec<StatusPosition> = rows.iter().map(|(market, r)| {
        if r.triggered || r.sold { triggered_count += 1; }
        StatusPosition {
            code: r.code.clone(),
            name: r.name.clone(),
            market: market.clone(),
            qty: r.qty,
            avg_price: 0.0, // 폴링 모드에선 평균단가 추적 안 함 (잔고 API에 손익률만 사용)
            last_price: None,
            pnl_rate: Some(r.pnl_rate),
            triggered: r.triggered || r.sold,
        }
    }).collect();
    let snap = StatusSnapshot {
        pid: std::process::id(),
        started_at: started_at.into(),
        updated_at: chrono::Local::now().to_rfc3339(),
        mode: "polling".into(),
        execute: cfg.execute,
        threshold: cfg.threshold_pct,
        symbols_filter: cfg.symbols.clone(),
        positions,
        triggered_count,
    };
    if let Err(e) = write_status_atomic(&snap) {
        error!("status 파일 쓰기 실패: {e}");
    }
}

/// 1초 throttle로 상태 파일 기록. force=true면 throttle 무시.
fn maybe_write_status(
    state: &std::sync::Arc<std::sync::Mutex<SharedState>>,
    cfg: &Config,
    force: bool,
) {
    let snapshot_result = {
        let mut st = state.lock().unwrap();
        let now = Instant::now();
        if !force && now.duration_since(st.last_status_write) < Duration::from_secs(1) {
            return;
        }
        st.last_status_write = now;
        Some(build_snapshot(cfg, &st.started_at, &st.positions, &st.last_prices))
    };
    if let Some(snap) = snapshot_result {
        if let Err(e) = write_status_atomic(&snap) {
            error!("status 파일 쓰기 실패: {e}");
        }
    }
}

fn is_pid_alive(pid: u32) -> bool {
    unsafe extern "C" {
        fn kill(pid: i32, sig: i32) -> i32;
    }
    // signal 0: 실제 신호는 안 보내고 프로세스 존재·권한만 확인
    unsafe { kill(pid as i32, 0) == 0 }
}

fn tr_spec(p: &Position) -> (&'static str, String) {
    match p.mode {
        PosMode::Domestic => ("H0STCNT0", p.code.clone()),
        PosMode::Overseas => {
            let excd = order_excg_to_ws(&p.excg_order);
            ("HDFSCNT0", format!("D{}{}", excd, p.code))
        }
    }
}

/// 주문용 거래소 코드(NASD/NYSE/AMEX) → WS tr_key용 3자리(NAS/NYS/AMS)
fn order_excg_to_ws(excg: &str) -> &'static str {
    match excg {
        "NASD" => "NAS",
        "NYSE" => "NYS",
        "AMEX" => "AMS",
        _ => "NAS",
    }
}

fn parse_tick(tr_id: &str, fields: &[&str]) -> Option<(String, f64)> {
    match tr_id {
        "H0STCNT0" => {
            if fields.len() < 3 { return None; }
            let key = fields[0].to_string(); // 종목코드
            let price: f64 = fields[2].parse().ok()?;
            Some((key, price))
        }
        "HDFSCNT0" => {
            if fields.len() < 12 { return None; }
            let key = fields[0].to_string(); // RSYM = "D" + EXCD + SYMB
            let price: f64 = fields[11].parse().ok()?;
            Some((key, price))
        }
        _ => None,
    }
}

fn print_positions_header(positions: &[Position]) {
    println!("감시 대상 포지션:");
    for p in positions {
        let label = match p.mode { PosMode::Domestic => "국내", PosMode::Overseas => "해외" };
        println!(
            "  {:<5} {:<10} {:<14} 수량 {:>6}  평균가 {}",
            label, p.code, truncate(&p.name, 14), p.qty,
            if p.avg_price >= 1000.0 { format_number(&format!("{:.0}", p.avg_price)) }
            else { format!("{:.4}", p.avg_price) }
        );
    }
    println!();
}

fn print_tick(p: &Position, price: f64, pnl_rate: f64, threshold: f64) {
    let now = chrono::Local::now().format("%H:%M:%S");
    let badge = if pnl_rate <= threshold { "\x1b[31m→트리거\x1b[0m" }
        else if pnl_rate <= threshold + 1.0 { "\x1b[33m⚠근접\x1b[0m" }
        else { "" };
    let label = match p.mode { PosMode::Domestic => "국내", PosMode::Overseas => "해외" };
    let price_str = if price >= 1000.0 { format_number(&format!("{:.0}", price)) }
        else { format!("{:.4}", price) };
    println!(
        "[{now}] {:<5} {:<10} {:<14} @ {:>12} {:>+7.2}% {}",
        label, p.code, truncate(&p.name, 14), price_str, pnl_rate, badge
    );
}

async fn fetch_initial_positions(
    client: &KisClient,
    filter: &Option<Vec<String>>,
) -> Result<Vec<Position>> {
    let mut out = Vec::new();

    // 국내
    let req = dome_bal::Request {
        cano: client.cano().into(), acnt_prdt_cd: client.product_code().into(),
        afhr_flpr_yn: "N".into(), ofl_yn: "".into(),
        inqr_dvsn: "02".into(), unpr_dvsn: "01".into(),
        fund_sttl_icld_yn: "N".into(), fncg_amt_auto_rdpt_yn: "N".into(),
        prcs_dvsn: "01".into(), ctx_area_fk100: "".into(), ctx_area_nk100: "".into(),
    };
    let dome = dome_bal::call(client, &req).await.unwrap_or_else(|_| dome_bal::Response { holdings: vec![], summary: None });
    for h in dome.holdings {
        if !symbol_match(filter, &h.pdno, &h.prdt_name) { continue; }
        let qty: u64 = h.ord_psbl_qty.trim().parse().unwrap_or(0);
        if qty == 0 { continue; }
        let avg: f64 = h.pchs_avg_pric.trim().parse().unwrap_or(0.0);
        out.push(Position {
            code: h.pdno, name: h.prdt_name,
            qty, avg_price: avg,
            mode: PosMode::Domestic,
            excg_order: String::new(),
            triggered: false,
        });
    }

    // 해외 (NASD/NYSE/AMEX)
    for excg in ["NASD", "NYSE", "AMEX"] {
        let req = usa_bal::Request {
            cano: client.cano().into(), acnt_prdt_cd: client.product_code().into(),
            ovrs_excg_cd: excg.into(), tr_crcy_cd: "USD".into(),
            ctx_area_fk200: "".into(), ctx_area_nk200: "".into(),
        };
        let resp = usa_bal::call(client, &req).await;
        let Ok(resp) = resp else { continue };
        for h in resp.holdings {
            if !symbol_match(filter, &h.ovrs_pdno, &h.ovrs_item_name) { continue; }
            let qty: u64 = h.ord_psbl_qty.trim().parse().unwrap_or(0);
            if qty == 0 { continue; }
            let avg: f64 = h.pchs_avg_pric.trim().parse().unwrap_or(0.0);
            out.push(Position {
                code: h.ovrs_pdno, name: h.ovrs_item_name,
                qty, avg_price: avg,
                mode: PosMode::Overseas,
                excg_order: excg.to_string(),
                triggered: false,
            });
        }
    }
    Ok(out)
}
