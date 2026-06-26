//! `kis telegram stream` — 관심 종목 시세를 텔레그램 메시지 1건에 **in-place** 로 실시간 갱신.
//!
//! **설계**
//! - 매 주기(기본 1초) 관심 종목마다 `inquire_price` REST 조회 → 표 1장으로 렌더.
//! - 장 시작 시 `sendMessage` 로 **그날 첫 메시지 1건** 발행 → 이후 `editMessageText` 로 같은 메시지 갱신.
//! - 날짜가 바뀌어 다음 세션이 열리면 자동으로 **새 메시지** 발행 (다음날 새 채팅).
//! - 세션 판정은 `daytrade::session`(한국 공휴일 `chk-holiday` 반영) 재사용. 장 마감/주말/공휴일엔 다음 개장까지 대기.
//! - 알림 폭탄 방지: 직전 렌더 텍스트와 동일하면 edit 생략, `429` 는 `retry_after` 만큼 백오프.

use std::sync::Arc;

use anyhow::{anyhow, Result};
use chrono::{Datelike, NaiveDate};
use serde_json::Value;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::api::domestic_stock::quotations::inquire_price;
use crate::api::overseas_stock::quotations::price as overseas_price;
use crate::client::KisClient;
use crate::commands::daytrade::session::{self, HolidayCache, Market};
use crate::commands::helpers::{format_number, resolve_symbol};
use crate::config::{load_config, TelegramConfig};
use crate::symbols::{self, ResolveMode};

pub struct StreamConfig {
    /// 관심 종목 (이름 또는 코드). 비어 있으면 영속 파일(telegram-stream.toml)에서 로드.
    pub symbols: Vec<String>,
    /// 갱신 주기 (초). 기본 1.
    pub interval_secs: u64,
    /// 세션 무시하고 즉시 1회만 전송 후 종료 (포맷 확인용).
    pub once: bool,
    /// systemd unit 설치 (Linux). macOS 는 unit 내용만 출력.
    pub background: bool,
    /// 텔레그램 `/add` `/rm` 등 명령 수신(getUpdates) 활성. 기본 ON.
    pub listen: bool,
    /// 복수 매칭 시 N번째 후보 선택 (1-indexed). 비-TTY 필수.
    pub pick: Option<usize>,
}

/// 해석된 관심 종목 1건.
#[derive(Clone)]
struct Watch {
    code: String,
    name: String,
    /// 국내(Kospi/Kosdaq) vs 해외(Nasdaq/Nyse/Amex) — 시세 API·통화·세션 분기에 사용.
    market: symbols::Market,
}

/// 렌더 루프와 명령 폴러가 공유하는 관심종목 리스트.
type Shared = Arc<Mutex<Vec<Watch>>>;

pub async fn run(client: Arc<KisClient>, cfg: StreamConfig) -> Result<()> {
    // 1) 관심종목 확정: CLI 인자가 있으면 그 목록으로 영속 파일을 덮어쓰고(seed),
    //    없으면 영속 파일(telegram-stream.toml)에서 로드.
    let watches = if !cfg.symbols.is_empty() {
        let resolved = resolve_watches(&cfg.symbols, cfg.pick)?;
        save_watchlist(&codes_of(&resolved))?;
        resolved
    } else {
        resolve_watches(&load_watchlist(), cfg.pick).unwrap_or_default()
    };

    // 2) --background: systemd unit 설치 (종목은 영속 파일에서 읽으므로 ExecStart 에 안 박음).
    if cfg.background {
        return install_systemd_unit(cfg.interval_secs);
    }

    let _log_guard = crate::logging::init_daemon("telegram-stream")?;

    // 3) 텔레그램 자격증명 필수.
    let tg = load_config()
        .ok()
        .and_then(|c| c.telegram)
        .filter(|t| !t.bot_token.is_empty() && !t.chat_id.is_empty())
        .ok_or_else(|| anyhow!("텔레그램 봇이 설정되지 않았습니다. 먼저 `kis config telegram` 을 실행하세요."))?;

    // 4) --once: 세션 무시, 즉시 1회 전송 후 종료.
    if cfg.once {
        let text = render(&client, &watches, cfg.interval_secs).await;
        let id = send_message(&tg, &text).await?;
        info!("--once 전송 완료 (message_id={id}, {}종목)", watches.len());
        return Ok(());
    }

    info!(
        "telegram stream 시작: {}종목 · 주기 {}초 · 명령수신 {}",
        watches.len(),
        cfg.interval_secs,
        if cfg.listen { "ON (/add /rm /list)" } else { "OFF" },
    );

    // 5) 공유 상태 + 종료 시그널.
    let shared: Shared = Arc::new(Mutex::new(watches));
    let cancel = CancellationToken::new();
    kis_daemon::shutdown::spawn_signal_listener(cancel.clone());

    // 6) 명령 폴러 (getUpdates) 기동.
    if cfg.listen {
        let client_cl = client.clone();
        let tg_cl = tg.clone();
        let shared_cl = shared.clone();
        let cancel_cl = cancel.clone();
        let pick = cfg.pick;
        tokio::spawn(async move {
            run_command_poller(client_cl, tg_cl, shared_cl, pick, cancel_cl).await;
        });
    }

    let cache = HolidayCache::new();
    // (메시지 날짜, message_id, 마지막 렌더 텍스트)
    let mut current: Option<(NaiveDate, i64, String)> = None;

    loop {
        if cancel.is_cancelled() {
            break;
        }
        let now = session::now_kst();
        let snapshot = shared.lock().await.clone();
        let markets = session_markets(&snapshot);
        let mut in_session = false;
        for m in &markets {
            if session::is_in_session_async(*m, now, &client, &cache).await {
                in_session = true;
                break;
            }
        }

        if in_session {
            let today = now.date_naive();
            let text = render(&client, &snapshot, cfg.interval_secs).await;

            match &current {
                // 같은 날 → 기존 메시지 갱신
                Some((d, id, last)) if *d == today => {
                    let id = *id;
                    if *last != text {
                        match edit_message_text(&tg, id, &text).await {
                            Ok(EditOutcome::Ok) | Ok(EditOutcome::NotModified) => {
                                current = Some((today, id, text));
                            }
                            Ok(EditOutcome::RateLimited(secs)) => {
                                warn!("텔레그램 rate-limit, {secs}초 백오프");
                                sleep_or_cancel(&cancel, secs).await;
                            }
                            Ok(EditOutcome::NotFound) => {
                                warn!("메시지를 찾을 수 없음 (삭제됨?) — 새 메시지 발행");
                                current = send_fresh(&tg, today, &text).await;
                            }
                            Err(e) => error!("editMessageText 실패: {e}"),
                        }
                    }
                }
                // 첫 실행 또는 날짜 변경 → 새 메시지 발행 (다음날 새 채팅)
                _ => {
                    current = send_fresh(&tg, today, &text).await;
                }
            }

            sleep_or_cancel(&cancel, cfg.interval_secs).await;
        } else {
            // 장 마감/주말/공휴일: 직전 세션 메시지를 1회 "장 마감" 표시로 마무리.
            if let Some((d, id, last)) = current.clone() {
                let closed = format!("{last}\n\n■ 장 마감 — 다음 개장 시 새 메시지로 갱신");
                if last != closed {
                    if let Err(e) = edit_message_text(&tg, id, &closed).await {
                        error!("장 마감 표시 실패: {e}");
                    }
                    current = Some((d, id, closed));
                }
            }
            // 모든 관심 시장이 닫힘 → 가장 가까운 개장까지 대기.
            let mut dur: Option<chrono::Duration> = None;
            for m in &markets {
                let d = session::time_until_open_async(*m, now, &client, &cache).await;
                if dur.is_none_or(|cur| d < cur) {
                    dur = Some(d);
                }
            }
            let secs = dur.map(|d| d.num_seconds().max(1) as u64).unwrap_or(60);
            info!("세션 밖 — 다음 개장까지 약 {}분 대기", secs / 60);
            sleep_or_cancel(&cancel, secs).await;
        }
    }

    info!("종료 신호 수신 — telegram stream 정리");
    Ok(())
}

/// 새 메시지 발행 후 상태 튜플 생성. 실패하면 상태 유지(None).
async fn send_fresh(tg: &TelegramConfig, today: NaiveDate, text: &str) -> Option<(NaiveDate, i64, String)> {
    match send_message(tg, text).await {
        Ok(id) => {
            info!("새 메시지 발행 (message_id={id})");
            Some((today, id, text.to_string()))
        }
        Err(e) => {
            error!("sendMessage 실패: {e}");
            None
        }
    }
}

/// 취소 신호와 함께 슬립. 취소되면 즉시 반환.
async fn sleep_or_cancel(cancel: &CancellationToken, secs: u64) {
    tokio::select! {
        _ = cancel.cancelled() => {}
        _ = tokio::time::sleep(std::time::Duration::from_secs(secs)) => {}
    }
}

fn resolve_watches(symbols: &[String], pick: Option<usize>) -> Result<Vec<Watch>> {
    symbols
        .iter()
        .map(|s| resolve_one(s, pick))
        .collect()
}

fn resolve_one(symbol: &str, pick: Option<usize>) -> Result<Watch> {
    let sym = resolve_symbol(symbol, ResolveMode::Any, pick)?;
    if sym.market.is_futureoption() {
        return Err(anyhow!("주식만 지원 (선물/옵션 미지원)"));
    }
    let name = if !sym.name_kr.is_empty() {
        sym.name_kr.clone()
    } else if !sym.name_en.is_empty() {
        sym.name_en.clone()
    } else {
        sym.code.clone()
    };
    Ok(Watch { code: sym.code, name, market: sym.market })
}

fn codes_of(watches: &[Watch]) -> Vec<String> {
    watches.iter().map(|w| w.code.clone()).collect()
}

/// 관심종목이 속한 세션 시장의 합집합(국내 있으면 KRX, 해외 있으면 USA).
/// 둘 중 하나라도 열려 있으면 갱신, 모두 닫히면 가장 가까운 개장까지 대기한다.
/// 비어 있으면 KRX 기본(기존 동작 유지).
fn session_markets(watches: &[Watch]) -> Vec<Market> {
    let mut out = Vec::new();
    if watches.iter().any(|w| w.market.is_domestic()) {
        out.push(Market::Krx);
    }
    if watches.iter().any(|w| w.market.is_overseas()) {
        out.push(Market::Usa);
    }
    if out.is_empty() {
        out.push(Market::Krx);
    }
    out
}

// ─────────────────────────────────────────────────────────────
// 관심종목 영속화 (telegram-stream.toml)
// ─────────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct WatchlistFile {
    #[serde(default)]
    symbols: Vec<String>,
}

/// 영속 파일에서 종목 코드 목록 로드. 없거나 깨졌으면 빈 목록.
fn load_watchlist() -> Vec<String> {
    let path = match crate::config::telegram_stream_path() {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    toml::from_str::<WatchlistFile>(&content)
        .map(|f| f.symbols)
        .unwrap_or_default()
}

/// 종목 코드 목록을 영속 파일에 저장.
fn save_watchlist(codes: &[String]) -> Result<()> {
    let path = crate::config::telegram_stream_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = WatchlistFile { symbols: codes.to_vec() };
    std::fs::write(&path, toml::to_string_pretty(&file)?)?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────
// 텔레그램 명령 수신 (getUpdates long-poll)
// ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Add(Vec<String>),
    Rm(Vec<String>),
    List,
    Clear,
    Help,
}

/// 메시지 텍스트 → 명령. 명령이 아니면 None.
/// `/add@botname` 처럼 봇 멘션이 붙어도 인식.
fn parse_command(text: &str) -> Option<Command> {
    let text = text.trim();
    let mut it = text.split_whitespace();
    let head = it.next()?;
    if !head.starts_with('/') {
        return None;
    }
    let cmd = head[1..].split('@').next().unwrap_or("").to_lowercase();
    let args: Vec<String> = it.map(|s| s.to_string()).collect();
    match cmd.as_str() {
        "add" => Some(Command::Add(args)),
        "rm" | "remove" | "del" | "delete" => Some(Command::Rm(args)),
        "list" | "ls" => Some(Command::List),
        "clear" => Some(Command::Clear),
        "help" | "start" => Some(Command::Help),
        _ => None,
    }
}

/// 중복(코드 기준) 제거하며 추가. 추가된 종목 라벨 반환.
fn apply_add(list: &mut Vec<Watch>, new: Vec<Watch>) -> Vec<String> {
    let mut added = Vec::new();
    for w in new {
        if list.iter().any(|e| e.code == w.code) {
            continue;
        }
        added.push(format!("{} ({})", w.name, w.code));
        list.push(w);
    }
    added
}

/// 코드 또는 이름 일치 항목 제거. 제거된 종목 라벨 반환.
fn apply_rm(list: &mut Vec<Watch>, queries: &[String]) -> Vec<String> {
    let mut removed = Vec::new();
    for q in queries {
        let q = q.trim();
        if let Some(pos) = list
            .iter()
            .position(|e| e.code == q || e.name == q)
        {
            let w = list.remove(pos);
            removed.push(format!("{} ({})", w.name, w.code));
        }
    }
    removed
}

const HELP_TEXT: &str = "🤖 관심종목 명령\n\
    /add 삼성전자 TSLA — 종목 추가 (국내·미국 주식, 이름 또는 코드, 여러 개)\n\
    /rm 삼성전자 — 종목 삭제\n\
    /list — 현재 관심종목\n\
    /clear — 전체 비우기\n\
    /help — 이 도움말";

async fn run_command_poller(
    client: Arc<KisClient>,
    tg: TelegramConfig,
    shared: Shared,
    pick: Option<usize>,
    cancel: CancellationToken,
) {
    let _ = &client; // 향후 확장 여지(현재 resolve 는 동기). 시그니처 일관성 유지.
    let http = reqwest::Client::new();
    let mut offset: i64 = 0;
    info!("명령 폴러 시작 (getUpdates)");

    loop {
        if cancel.is_cancelled() {
            break;
        }
        let url = format!("https://api.telegram.org/bot{}/getUpdates", tg.bot_token);
        let req = http
            .get(&url)
            .query(&[
                ("timeout", "30".to_string()),
                ("offset", offset.to_string()),
                ("allowed_updates", "[\"message\"]".to_string()),
            ])
            .timeout(std::time::Duration::from_secs(40))
            .send();

        let resp = tokio::select! {
            _ = cancel.cancelled() => break,
            r = req => r,
        };

        let updates: Value = match resp {
            Ok(r) => match r.json().await {
                Ok(v) => v,
                Err(e) => {
                    warn!("getUpdates 파싱 실패: {e}");
                    sleep_or_cancel(&cancel, 5).await;
                    continue;
                }
            },
            Err(e) => {
                warn!("getUpdates 요청 실패: {e} — 5초 후 재시도");
                sleep_or_cancel(&cancel, 5).await;
                continue;
            }
        };

        if updates.get("ok").and_then(Value::as_bool) != Some(true) {
            // 409 Conflict (다중 폴러) 포함.
            warn!("getUpdates 오류: {} — 10초 백오프", updates);
            sleep_or_cancel(&cancel, 10).await;
            continue;
        }

        let results = match updates.get("result").and_then(Value::as_array) {
            Some(a) => a,
            None => continue,
        };

        for u in results {
            if let Some(id) = u.get("update_id").and_then(Value::as_i64) {
                offset = offset.max(id + 1);
            }
            let chat_id = u.pointer("/message/chat/id").and_then(Value::as_i64);
            // 설정된 chat_id 만 처리 (다른 사용자 무시).
            if chat_id.map(|c| c.to_string()) != Some(tg.chat_id.clone()) {
                continue;
            }
            let text = match u.pointer("/message/text").and_then(Value::as_str) {
                Some(t) => t,
                None => continue,
            };
            if let Some(cmd) = parse_command(text) {
                let reply = handle_command(cmd, &shared, pick).await;
                if let Err(e) = send_message(&tg, &reply).await {
                    error!("명령 회신 전송 실패: {e}");
                }
            }
        }
    }
    info!("명령 폴러 종료");
}

/// 명령 처리 → 공유 리스트 갱신 + 영속화 → 회신 텍스트.
async fn handle_command(cmd: Command, shared: &Shared, pick: Option<usize>) -> String {
    match cmd {
        Command::Help => HELP_TEXT.to_string(),
        Command::List => {
            let list = shared.lock().await;
            if list.is_empty() {
                "관심종목이 비어 있습니다. /add 005930 으로 등록하세요.".into()
            } else {
                let body = list
                    .iter()
                    .map(|w| format!("• {} ({})", w.name, w.code))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("📋 관심종목 {}개\n{}", list.len(), body)
            }
        }
        Command::Clear => {
            let mut list = shared.lock().await;
            list.clear();
            persist(&list);
            "🧹 관심종목을 모두 비웠습니다.".into()
        }
        Command::Add(queries) => {
            if queries.is_empty() {
                return "사용법: /add 삼성전자 000660".into();
            }
            // resolve 는 동기 — lock 밖에서 수행.
            let mut resolved = Vec::new();
            let mut failed = Vec::new();
            for q in &queries {
                match resolve_one(q, pick) {
                    Ok(w) => resolved.push(w),
                    Err(e) => failed.push(format!("{q}: {e}")),
                }
            }
            let mut list = shared.lock().await;
            let added = apply_add(&mut list, resolved);
            persist(&list);
            let mut msg = String::new();
            if !added.is_empty() {
                msg.push_str(&format!("✅ 추가: {}\n", added.join(", ")));
            }
            if !failed.is_empty() {
                msg.push_str(&format!("⚠️ 실패: {}\n", failed.join(" / ")));
            }
            if msg.is_empty() {
                msg.push_str("이미 등록된 종목입니다.");
            }
            msg.push_str(&format!("(현재 {}종목)", list.len()));
            msg
        }
        Command::Rm(queries) => {
            if queries.is_empty() {
                return "사용법: /rm 삼성전자 (또는 코드)".into();
            }
            let mut list = shared.lock().await;
            let removed = apply_rm(&mut list, &queries);
            persist(&list);
            if removed.is_empty() {
                format!("해당 종목을 찾지 못했습니다. (현재 {}종목)", list.len())
            } else {
                format!("🗑 삭제: {} (현재 {}종목)", removed.join(", "), list.len())
            }
        }
    }
}

/// 공유 리스트를 영속 파일에 반영 (실패는 로그만).
fn persist(list: &[Watch]) {
    if let Err(e) = save_watchlist(&codes_of(list)) {
        error!("관심종목 저장 실패: {e}");
    }
}

// ─────────────────────────────────────────────────────────────
// 시세 렌더
// ─────────────────────────────────────────────────────────────

async fn render(client: &KisClient, watches: &[Watch], interval_secs: u64) -> String {
    let now = session::now_kst();
    let header = format!(
        "📊 관심종목  {}({}) {}",
        now.format("%m/%d"),
        kor_weekday(now.weekday()),
        now.format("%H:%M:%S"),
    );

    if watches.is_empty() {
        return format!(
            "{header}\n\n관심종목이 비어 있습니다.\n텔레그램에서 <code>/add 005930 TSLA</code> (국내·미국, 이름 또는 코드) 로 등록하세요."
        );
    }

    // 각 종목 행 데이터 수집.
    struct Row {
        name: String,
        price: String,
        change: String,
    }
    let mut rows: Vec<Row> = Vec::with_capacity(watches.len());
    for w in watches {
        match fetch_quote(client, w).await {
            Ok(q) => {
                rows.push(Row {
                    name: w.name.clone(),
                    price: q.price,
                    change: q.change,
                });
            }
            Err(e) => {
                warn!("[{}] {} 시세 조회 실패: {e}", w.code, w.name);
                rows.push(Row {
                    name: w.name.clone(),
                    price: "조회 실패".into(),
                    change: String::new(),
                });
            }
        }
    }

    // 컬럼 폭 산출 (한글 2칸 폭 고려).
    let name_w = rows.iter().map(|r| display_width(&r.name)).max().unwrap_or(0);
    let price_w = rows.iter().map(|r| r.price.chars().count()).max().unwrap_or(0);

    let mut body = String::new();
    for r in &rows {
        body.push_str(&pad_to(&esc(&r.name), name_w));
        body.push_str("  ");
        body.push_str(&" ".repeat(price_w.saturating_sub(r.price.chars().count())));
        body.push_str(&esc(&r.price));
        if !r.change.is_empty() {
            body.push_str("  ");
            body.push_str(&r.change);
        }
        body.push('\n');
    }

    format!(
        "{header}\n<pre>{body}</pre>🔄 {interval_secs}초마다 갱신"
    )
}

/// 시장에 맞는 현재가를 조회해 통화·부호까지 포맷한 표시용 값으로 정규화.
struct QuoteView {
    price: String,
    change: String,
}

async fn fetch_quote(client: &KisClient, w: &Watch) -> Result<QuoteView> {
    if w.market.is_overseas() {
        let req = overseas_price::Request {
            auth: String::new(),
            excd: w.market.excd().into(),
            symb: w.code.clone(),
        };
        let q = overseas_price::call(client, &req).await?;
        Ok(QuoteView {
            price: fmt_usd(&q.last),
            change: fmt_change_usd(&q.sign, &q.diff, &q.rate),
        })
    } else {
        let req = inquire_price::Request {
            fid_cond_mrkt_div_code: "J".into(),
            fid_input_iscd: w.code.clone(),
        };
        let q = inquire_price::call(client, &req).await?;
        Ok(QuoteView {
            price: format!("{}원", format_number(&q.stck_prpr)),
            change: fmt_change(&q.prdy_vrss_sign, &q.prdy_vrss, &q.prdy_ctrt),
        })
    }
}

/// "242.8400" → "$242.84". 파싱 실패 시 원문에 $ 만.
fn fmt_usd(last: &str) -> String {
    match last.trim().parse::<f64>() {
        Ok(v) => format!("${v:.2}"),
        Err(_) => format!("${}", last.trim()),
    }
}

/// 전일대비 부호/등락 포맷. KIS sign: 1 상한, 2 상승, 3 보합, 4 하한, 5 하락.
/// KIS 대비기호(1상한·2상승·3보합·4하한·5하락) → (화살표, 부호). 국내·해외 공통.
fn sign_arrow(sign: &str) -> (&'static str, &'static str) {
    match sign {
        "1" | "2" => ("▲", "+"),
        "4" | "5" => ("▼", "-"),
        _ => ("─", ""),
    }
}

fn fmt_change(sign: &str, vrss: &str, ctrt: &str) -> String {
    let (arrow, pol) = sign_arrow(sign);
    let vrss_abs = vrss.trim().trim_start_matches('-');
    let ctrt_abs = ctrt.trim().trim_start_matches('-');
    format!("{arrow} {pol}{}  {pol}{ctrt_abs}%", format_number(vrss_abs))
}

/// 해외 시세용 — diff/rate 가 소수(달러·%)라 2자리로 포맷. 부호는 sign 으로.
fn fmt_change_usd(sign: &str, diff: &str, rate: &str) -> String {
    let (arrow, pol) = sign_arrow(sign);
    format!("{arrow} {pol}{}  {pol}{}%", fmt_dec2(diff), fmt_dec2(rate))
}

/// "2.3400"/"-2.3400" → "2.34"(절댓값, 부호는 호출부 pol 담당). 파싱 실패 시 원문 절댓값.
fn fmt_dec2(s: &str) -> String {
    match s.trim().parse::<f64>() {
        Ok(v) => format!("{:.2}", v.abs()),
        Err(_) => s.trim().trim_start_matches('-').to_string(),
    }
}

fn kor_weekday(w: chrono::Weekday) -> &'static str {
    use chrono::Weekday::*;
    match w {
        Mon => "월",
        Tue => "화",
        Wed => "수",
        Thu => "목",
        Fri => "금",
        Sat => "토",
        Sun => "일",
    }
}

/// 텔레그램 HTML parse_mode 에서 의미를 갖는 문자 이스케이프.
fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// 표시 폭 (한글/CJK/전각 = 2칸).
fn display_width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            let u = c as u32;
            let wide = (0x1100..=0x115F).contains(&u)   // Hangul Jamo
                || (0x2E80..=0xA4CF).contains(&u)        // CJK 부수 ~ Hangul 호환 자모
                || (0xAC00..=0xD7A3).contains(&u)        // Hangul 음절
                || (0xF900..=0xFAFF).contains(&u)        // CJK 호환 한자
                || (0xFE30..=0xFE4F).contains(&u)        // CJK 호환 형태
                || (0xFF00..=0xFF60).contains(&u)        // 전각 형태
                || (0xFFE0..=0xFFE6).contains(&u);
            if wide { 2 } else { 1 }
        })
        .sum()
}

fn pad_to(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - w))
    }
}

// ─────────────────────────────────────────────────────────────
// 텔레그램 Bot API (sendMessage / editMessageText)
// ─────────────────────────────────────────────────────────────

enum EditOutcome {
    Ok,
    NotModified,
    NotFound,
    /// retry_after (초)
    RateLimited(u64),
}

async fn send_message(tg: &TelegramConfig, text: &str) -> Result<i64> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", tg.bot_token);
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": tg.chat_id,
            "text": text,
            "parse_mode": "HTML",
            "disable_web_page_preview": true,
        }))
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await?;
    if body.get("ok").and_then(Value::as_bool) != Some(true) {
        return Err(anyhow!("sendMessage HTTP {}: {}", status, body));
    }
    body["result"]["message_id"]
        .as_i64()
        .ok_or_else(|| anyhow!("message_id 파싱 실패: {body}"))
}

async fn edit_message_text(tg: &TelegramConfig, message_id: i64, text: &str) -> Result<EditOutcome> {
    let url = format!("https://api.telegram.org/bot{}/editMessageText", tg.bot_token);
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": tg.chat_id,
            "message_id": message_id,
            "text": text,
            "parse_mode": "HTML",
            "disable_web_page_preview": true,
        }))
        .send()
        .await?;
    let status = resp.status();
    let body: Value = resp.json().await?;
    if body.get("ok").and_then(Value::as_bool) == Some(true) {
        return Ok(EditOutcome::Ok);
    }
    let desc = body["description"].as_str().unwrap_or_default();
    if desc.contains("message is not modified") {
        return Ok(EditOutcome::NotModified);
    }
    if desc.contains("message to edit not found") || desc.contains("MESSAGE_ID_INVALID") {
        return Ok(EditOutcome::NotFound);
    }
    if status.as_u16() == 429 {
        let retry = body["parameters"]["retry_after"].as_u64().unwrap_or(1);
        return Ok(EditOutcome::RateLimited(retry));
    }
    Err(anyhow!("editMessageText HTTP {}: {}", status, body))
}

// ─────────────────────────────────────────────────────────────
// `--background` — systemd user unit (Linux 전용, 단일 서비스)
// ─────────────────────────────────────────────────────────────

const SERVICE_NAME: &str = "kis-telegram-stream";

fn unit_path() -> String {
    format!("/etc/systemd/system/{SERVICE_NAME}.service")
}

fn install_systemd_unit(interval_secs: u64) -> Result<()> {
    let run_user = std::env::var("SUDO_USER")
        .ok()
        .or_else(|| std::env::var("USER").ok())
        .ok_or_else(|| anyhow!("$USER 를 읽을 수 없습니다"))?;

    let exe = std::env::current_exe()?;
    // 종목은 telegram-stream.toml 에서 읽으므로 ExecStart 에 박지 않는다.
    let exec_start = format!(
        "{} telegram stream --interval {interval_secs}",
        shell_escape(&exe.to_string_lossy())
    );

    let unit = format!(
        "[Unit]\n\
         Description=kis-cli telegram stream (관심종목 라이브)\n\
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
         WantedBy=multi-user.target\n",
    );
    let path = unit_path();

    if !cfg!(target_os = "linux") {
        println!("─────────────────────────────────────────────");
        println!("⚠ systemd는 Linux 전용입니다. 아래 unit 파일을 VPS에 복사하세요.");
        println!("파일 경로: {path}");
        println!("─────────────────────────────────────────────");
        print!("{unit}");
        println!("─────────────────────────────────────────────");
        println!("⚠ ExecStart 의 경로는 현재 맥 로컬 바이너리입니다. VPS 에선 `which kis` 결과로 교체하세요.");
        println!();
        println!("VPS 에서 직접 실행이 더 편합니다:");
        println!("  sudo $(which kis) telegram stream --interval {interval_secs} --background");
        println!("종목은 텔레그램에서 /add 로 등록하거나 telegram-stream.toml 을 편집하세요.");
        return Ok(());
    }

    match std::fs::write(&path, &unit) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!(
                "{path} 에 쓰기 권한이 없습니다. 재실행: sudo $(which kis) telegram stream --interval {interval_secs} --background"
            ));
        }
        Err(e) => return Err(anyhow!("{path} 쓰기 실패: {e}")),
    }
    info!("systemd unit 생성: {path}");

    run_systemctl(&["daemon-reload"])?;
    run_systemctl(&["enable", "--now", SERVICE_NAME])?;
    info!("✓ {SERVICE_NAME}.service 활성화 및 시작됨 (실행 유저: {run_user})");
    println!("ExecStart: {exec_start}");
    println!("로그 확인: sudo journalctl -u {SERVICE_NAME} -f");
    println!("제거:      sudo $(which kis) telegram remove");
    Ok(())
}

pub fn list_service() -> Result<()> {
    let path = unit_path();
    if !std::path::Path::new(&path).exists() {
        println!("(등록된 {SERVICE_NAME} 서비스 없음 — Linux 전용 기능)");
        return Ok(());
    }
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let description = extract_field(&content, "Description=").unwrap_or_default();
    let exec_start = extract_field(&content, "ExecStart=").unwrap_or_default();
    println!("● {SERVICE_NAME}.service");
    if !description.is_empty() {
        println!("    Description: {description}");
    }
    println!(
        "    Status:      active={} / enabled={}",
        systemctl_query(&["is-active", SERVICE_NAME]),
        systemctl_query(&["is-enabled", SERVICE_NAME]),
    );
    if !exec_start.is_empty() {
        println!("    ExecStart:   {exec_start}");
    }
    println!("    Unit:        {path}");
    println!();
    println!("로그: sudo journalctl -u {SERVICE_NAME} -f");
    println!("제거: sudo $(which kis) telegram remove");
    Ok(())
}

pub fn remove_service() -> Result<()> {
    let path = unit_path();
    if !std::path::Path::new(&path).exists() {
        return Err(anyhow!("서비스 파일이 없습니다: {path}"));
    }
    if let Err(e) = run_systemctl(&["disable", "--now", SERVICE_NAME]) {
        error!("disable --now 실패 (무시하고 파일 삭제 시도): {e}");
    }
    match std::fs::remove_file(&path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!("{path} 삭제 권한이 없습니다. 재실행: sudo $(which kis) telegram remove"));
        }
        Err(e) => return Err(anyhow!("{path} 삭제 실패: {e}")),
    }
    run_systemctl(&["daemon-reload"])?;
    info!("✓ {SERVICE_NAME}.service 제거됨 ({path} 삭제)");
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

fn shell_escape(s: &str) -> String {
    if s.chars().all(|c| c.is_ascii_alphanumeric() || "/._-".contains(c)) {
        s.to_string()
    } else {
        format!("\"{}\"", s.replace('"', "\\\""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_up_down_flat() {
        assert_eq!(fmt_change("2", "100", "0.13"), "▲ +100  +0.13%");
        assert_eq!(fmt_change("1", "29800", "29.95"), "▲ +29,800  +29.95%"); // 상한
        assert_eq!(fmt_change("5", "-2000", "1.11"), "▼ -2,000  -1.11%");
        assert_eq!(fmt_change("3", "0", "0.00"), "─ 0  0.00%"); // 보합
    }

    #[test]
    fn hangul_is_double_width() {
        assert_eq!(display_width("삼성전자"), 8);
        assert_eq!(display_width("ABC"), 3);
        assert_eq!(display_width("SK하이닉스"), 10); // S,K = 2, 하이닉스 = 8
    }

    #[test]
    fn pad_aligns_by_display_width() {
        // 다른 글자수라도 표시 폭은 동일하게 정렬돼야 한다.
        let a = pad_to("삼성전자", 12); // 폭 8 → +4 공백
        let b = pad_to("SK하이닉스", 12); // 폭 10 → +2 공백
        assert_eq!(display_width(&a), 12);
        assert_eq!(display_width(&b), 12);
    }

    #[test]
    fn esc_handles_ampersand_in_name() {
        // F&F 같은 종목명이 HTML parse_mode 를 깨지 않아야 한다.
        assert_eq!(esc("F&F"), "F&amp;F");
        assert_eq!(esc("<b>"), "&lt;b&gt;");
    }

    fn w(code: &str, name: &str) -> Watch {
        Watch { code: code.into(), name: name.into(), market: symbols::Market::Kospi }
    }

    fn w_mkt(code: &str, name: &str, market: symbols::Market) -> Watch {
        Watch { code: code.into(), name: name.into(), market }
    }

    #[test]
    fn fmt_usd_two_decimals() {
        assert_eq!(fmt_usd("242.8400"), "$242.84");
        assert_eq!(fmt_usd(" 13 "), "$13.00");
        assert_eq!(fmt_usd("n/a"), "$n/a"); // 파싱 실패 → 원문에 $ 만
    }

    #[test]
    fn fmt_change_usd_uses_sign_and_two_decimals() {
        // 2 상승 → ▲ +, diff/rate 2자리
        assert_eq!(fmt_change_usd("2", "2.3400", "0.9700"), "▲ +2.34  +0.97%");
        // 5 하락 → ▼ -, diff 가 음수로 와도 절댓값
        assert_eq!(fmt_change_usd("5", "-1.2000", "-0.50"), "▼ -1.20  -0.50%");
        // 3 보합
        assert_eq!(fmt_change_usd("3", "0", "0"), "─ 0.00  0.00%");
    }

    #[test]
    fn session_markets_is_union() {
        use symbols::Market as M;
        // 국내만 → KRX
        assert_eq!(session_markets(&[w("005930", "삼성전자")]), vec![Market::Krx]);
        // 해외만 → USA
        assert_eq!(
            session_markets(&[w_mkt("TSLA", "TESLA", M::Nasdaq)]),
            vec![Market::Usa]
        );
        // 혼합 → 둘 다
        assert_eq!(
            session_markets(&[w("005930", "삼성전자"), w_mkt("TSLA", "TESLA", M::Nasdaq)]),
            vec![Market::Krx, Market::Usa]
        );
        // 빈 목록 → KRX 기본
        assert_eq!(session_markets(&[]), vec![Market::Krx]);
    }

    #[test]
    fn parse_command_variants() {
        assert_eq!(
            parse_command("/add 삼성전자 000660"),
            Some(Command::Add(vec!["삼성전자".into(), "000660".into()]))
        );
        assert_eq!(parse_command("/rm 삼성전자"), Some(Command::Rm(vec!["삼성전자".into()])));
        assert_eq!(parse_command("/del 005930"), Some(Command::Rm(vec!["005930".into()])));
        assert_eq!(parse_command("/list"), Some(Command::List));
        assert_eq!(parse_command("/LS"), Some(Command::List)); // 대소문자 무시
        assert_eq!(parse_command("/clear"), Some(Command::Clear));
        assert_eq!(parse_command("/help"), Some(Command::Help));
        assert_eq!(parse_command("/start"), Some(Command::Help));
        // 봇 멘션 접미사 처리
        assert_eq!(parse_command("/add@my_bot 005930"), Some(Command::Add(vec!["005930".into()])));
        // 비명령
        assert_eq!(parse_command("그냥 메시지"), None);
        assert_eq!(parse_command("/unknown"), None);
    }

    #[test]
    fn add_dedupes_by_code() {
        let mut list = vec![w("005930", "삼성전자")];
        let added = apply_add(&mut list, vec![w("000660", "SK하이닉스"), w("005930", "삼성전자")]);
        assert_eq!(list.len(), 2); // 중복 005930 무시
        assert_eq!(added, vec!["SK하이닉스 (000660)".to_string()]);
    }

    #[test]
    fn rm_by_code_or_name() {
        let mut list = vec![w("005930", "삼성전자"), w("000660", "SK하이닉스")];
        let removed = apply_rm(&mut list, &["005930".into(), "SK하이닉스".into()]);
        assert!(list.is_empty());
        assert_eq!(removed.len(), 2);
        // 없는 종목은 조용히 무시
        let mut list2 = vec![w("005930", "삼성전자")];
        let removed2 = apply_rm(&mut list2, &["없음".into()]);
        assert!(removed2.is_empty());
        assert_eq!(list2.len(), 1);
    }
}
