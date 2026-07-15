//! `kis brief stream` — 관심 종목 시세를 텔레그램 메시지 1건에 **in-place** 로 실시간 갱신.
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

use kis_core::api::domestic_stock::quotations::inquire_price;
use kis_core::api::domestic_stock::market_analysis::inquire_investor_time_by_market as inv_by_market;
use kis_core::api::domestic_stock::market_analysis::investor_trend_estimate as inv_estimate;
use kis_core::api::domestic_stock::market_analysis::investor_trade_by_stock_daily as inv_by_stock;
use kis_core::api::overseas_stock::quotations::price as overseas_price;
use kis_core::client::KisClient;
use crate::common::session::{self, HolidayCache, Market};
use crate::common::resolve::{format_number, resolve_symbol};
use kis_core::config::{load_config, TelegramConfig};
use kis_data::symbols::{self, ResolveMode};

// 텔레그램 Bot API HTTP 헬퍼는 kis-trade::common::notify 공용.
use crate::common::notify::{edit_message_text, register_commands, send_message, EditOutcome, SendOutcome};

pub struct StreamConfig {
    /// 관심 종목 (이름 또는 코드). 비어 있으면 영속 파일(brief-stream.toml)에서 로드.
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

/// 세션 밖(장 마감·주말·공휴일) 갱신 주기. 시세가 거의 안 변하므로 느리게.
const OFF_HOURS_REFRESH_SECS: u64 = 60;

/// 텔레그램 실시간 관심종목 스트림 엔진. 로깅 init·시그널 리스너는 호출자가 소유하고
/// `cancel` 로 그레이스풀 종료를 주입한다. `--background`(systemd) 분기는 그대로 처리한다.
pub async fn run(
    client: Arc<KisClient>,
    cfg: StreamConfig,
    cancel: CancellationToken,
) -> Result<()> {
    // 1) 관심종목 확정: CLI 인자가 있으면 그 목록으로 영속 파일을 덮어쓰고(seed),
    //    없으면 영속 파일(brief-stream.toml)에서 로드.
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

    // 3) 텔레그램 자격증명 필수.
    let tg = load_config()
        .ok()
        .and_then(|c| c.telegram)
        .filter(|t| !t.bot_token.is_empty() && !t.chat_id.is_empty())
        .ok_or_else(|| anyhow!("텔레그램 봇이 설정되지 않았습니다. 먼저 `kis config telegram` 을 실행하세요."))?;

    // 4) --once: 세션 무시, 즉시 1회 전송 후 종료.
    if cfg.once {
        if watches.is_empty() {
            return Err(anyhow!("관심종목 없음 — `kis brief stream <종목> --once` 로 시드하세요"));
        }
        let cache = HolidayCache::new();
        let now = session::now_kst();
        let in_session = any_in_session(&session_markets(&watches), now, &client, &cache).await;
        let text = render(&client, &watches, cfg.interval_secs, in_session).await;
        let id = match send_message(&tg, &text).await? {
            SendOutcome::Sent(id) => id,
            SendOutcome::RateLimited(secs) => {
                return Err(anyhow!("sendMessage rate-limited (retry_after {secs}초)"));
            }
        };
        info!("--once 전송 완료 (message_id={id}, {}종목)", watches.len());
        return Ok(());
    }

    info!(
        "brief stream 시작: {}종목 · 주기 {}초 · 명령수신 {}",
        watches.len(),
        cfg.interval_secs,
        if cfg.listen { "ON (/add /rm /list)" } else { "OFF" },
    );

    if watches.is_empty() {
        warn!("관심종목 없음 — 텔레그램 /add <종목> 또는 `kis brief stream <종목> --once` 로 시드하세요");
    }

    // 5) 공유 상태 (종료 시그널은 주입된 cancel 사용).
    let shared: Shared = Arc::new(Mutex::new(watches));

    // 6) 명령 폴러 (getUpdates) 기동.
    if cfg.listen {
        // 텔레그램 명령 메뉴(/ 자동완성·Menu 버튼)에 명령 목록 등록 — 실패해도 데몬은 계속.
        if let Err(e) = register_commands(&tg).await {
            warn!("명령 메뉴 등록 실패(setMyCommands): {e}");
        }
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
    // (메시지 생성 날짜, message_id, 마지막 렌더 텍스트)
    let mut current: Option<(NaiveDate, i64, String)> = None;
    let mut prev_in_session: Option<bool> = None;
    // 연속 edit/발행 실패(429·네트워크) 카운트 — 점진 백오프에 사용. 성공 시 0 리셋.
    let mut failures: u32 = 0;

    loop {
        if cancel.is_cancelled() {
            break;
        }
        let now = session::now_kst();
        let today = now.date_naive();
        let snapshot = shared.lock().await.clone();
        if snapshot.is_empty() {
            // 관심종목 없음 — 시세 조회·메시지 발행/갱신 스킵. /add 런타임 추가 대기.
            sleep_or_cancel(&cancel, OFF_HOURS_REFRESH_SECS).await;
            continue;
        }
        let markets = session_markets(&snapshot);
        let in_session = any_in_session(&markets, now, &client, &cache).await;

        // 세션 전환 시에만 로그 (세션 밖엔 60초 폴링이라 매번 찍으면 시끄러움).
        if prev_in_session != Some(in_session) {
            info!("{}", if in_session {
                "장중 — 갱신 시작"
            } else {
                "세션 밖 — 마지막 체결가 표시 (느린 갱신)"
            });
            prev_in_session = Some(in_session);
        }

        let text = render(&client, &snapshot, cfg.interval_secs, in_session).await;

        // 새 메시지는 "첫 실행" 또는 "장중에 트레이딩일이 바뀐 경우"에만 발행.
        // 세션 밖에는 기존 메시지를 계속 갱신(자정 넘어도 새 메시지 안 띄움 → 마감/주말 도배 방지).
        let need_fresh = match &current {
            None => true,
            Some((d, _, _)) => in_session && *d != today,
        };

        // 이번 주기 끝에 추가로 대기할 백오프. None 이면 정상 주기 대기.
        // RateLimited/네트워크 실패 시에만 세워, 이중 sleep(백오프+정상 wait)을 막는다.
        let mut backoff: Option<u64> = None;

        match current.clone() {
            // 기존 메시지 유지 → 텍스트가 바뀐 경우만 in-place 갱신.
            Some((d, id, last)) if !need_fresh => {
                if last != text {
                    match edit_message_text(&tg, id, &text).await {
                        Ok(EditOutcome::Ok) | Ok(EditOutcome::NotModified) => {
                            failures = 0;
                            current = Some((d, id, text));
                        }
                        Ok(EditOutcome::RateLimited(secs)) => {
                            failures = failures.saturating_add(1);
                            warn!("텔레그램 rate-limit, {secs}초 백오프 (연속 {failures}회)");
                            backoff = Some(secs);
                        }
                        Ok(EditOutcome::NotFound) => {
                            warn!("메시지를 찾을 수 없음 (삭제됨?) — 새 메시지 발행");
                            match send_fresh(&tg, today, &text).await {
                                FreshOutcome::Sent(s) => {
                                    failures = 0;
                                    current = Some(s);
                                }
                                FreshOutcome::RateLimited(secs) => {
                                    failures = failures.saturating_add(1);
                                    warn!("sendMessage rate-limit (연속 {failures}회, {secs}초 백오프)");
                                    backoff = Some(secs);
                                }
                                FreshOutcome::Failed => {
                                    failures = failures.saturating_add(1);
                                    let bo = backoff_for(failures);
                                    error!("sendMessage 실패 (연속 {failures}회, {bo}초 후 재시도)");
                                    backoff = Some(bo);
                                }
                            }
                        }
                        Err(e) => {
                            failures = failures.saturating_add(1);
                            let bo = backoff_for(failures);
                            error!("editMessageText 실패 (연속 {failures}회, {bo}초 후 재시도): {e}");
                            backoff = Some(bo);
                        }
                    }
                }
            }
            // 첫 실행 또는 새 트레이딩일 → 새 메시지 발행.
            _ => {
                let next = send_fresh(&tg, today, &text).await;
                match next {
                    FreshOutcome::Sent(s) => {
                        failures = 0;
                        current = Some(s);
                    }
                    FreshOutcome::RateLimited(secs) => {
                        failures = failures.saturating_add(1);
                        warn!("sendMessage rate-limit (연속 {failures}회, {secs}초 백오프)");
                        backoff = Some(secs);
                    }
                    FreshOutcome::Failed => {
                        failures = failures.saturating_add(1);
                        let bo = backoff_for(failures);
                        error!("sendMessage 실패 (연속 {failures}회, {bo}초 후 재시도)");
                        backoff = Some(bo);
                    }
                }
            }
        }

        // 백오프가 있으면 그것만 대기(이중 sleep 방지). 없으면 정상 주기.
        let wait = match backoff {
            Some(bo) => bo,
            None if in_session => cfg.interval_secs,
            None => OFF_HOURS_REFRESH_SECS,
        };
        sleep_or_cancel(&cancel, wait).await;
    }

    info!("종료 신호 수신 — brief stream 정리");
    Ok(())
}

enum FreshOutcome {
    Sent((NaiveDate, i64, String)),
    RateLimited(u64),
    Failed,
}

/// 새 메시지 발행. 성공 시 상태 튜플, 429 시 `retry_after`(초), 그 외 실패.
async fn send_fresh(tg: &TelegramConfig, today: NaiveDate, text: &str) -> FreshOutcome {
    match send_message(tg, text).await {
        Ok(SendOutcome::Sent(id)) => {
            info!("새 메시지 발행 (message_id={id})");
            FreshOutcome::Sent((today, id, text.to_string()))
        }
        Ok(SendOutcome::RateLimited(secs)) => {
            warn!("텔레그램 rate-limit (retry_after {secs}초)");
            FreshOutcome::RateLimited(secs)
        }
        Err(e) => {
            error!("sendMessage 실패: {e}");
            FreshOutcome::Failed
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

/// 연속 실패에 대한 점진 백오프(초): 2, 4, 8, 16, 32, 60, 60, ...
/// 네트워크 실패 시 1초 폭격을 막고, 429 누적 페널티를 식히는 데 사용.
fn backoff_for(failures: u32) -> u64 {
    (1u64 << failures.min(6)).min(60)
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

/// 관심 시장 중 하나라도 세션이 열려 있으면 true (공휴일 반영).
async fn any_in_session(
    markets: &[Market],
    now: chrono::DateTime<chrono_tz::Tz>,
    client: &KisClient,
    cache: &HolidayCache,
) -> bool {
    for m in markets {
        if session::is_in_session_async(*m, now, client, cache).await {
            return true;
        }
    }
    false
}

// ─────────────────────────────────────────────────────────────
// 관심종목 영속화 (brief-stream.toml)
// ─────────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct WatchlistFile {
    #[serde(default)]
    symbols: Vec<String>,
}

/// 영속 파일에서 종목 코드 목록 로드. 없거나 깨졌으면 빈 목록.
fn load_watchlist() -> Vec<String> {
    let path = match kis_core::config::brief_stream_path() {
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
    let path = kis_core::config::brief_stream_path()?;
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
    Sync,
    /// 수급. 인자 없으면 코스피/코스닥 전체, 있으면 해당 개별종목.
    Supply(Vec<String>),
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
        "sync" | "symbols" => Some(Command::Sync),
        "su" | "sugup" | "sg" | "flow" => Some(Command::Supply(args)),
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
    /su — 국장 수급 (코스피/코스닥 전체, 기관 세분화)\n\
    /su 삼성전자 — 개별종목 수급 (일별 세분화 + 장중 잠정)\n\
    /sync — 종목 마스터 수동 갱신 (신규 상장·미인식 종목)\n\
    /help — 이 도움말";

async fn run_command_poller(
    client: Arc<KisClient>,
    tg: TelegramConfig,
    shared: Shared,
    pick: Option<usize>,
    cancel: CancellationToken,
) {
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
                let reply = handle_command(&client, cmd, &shared, pick).await;
                if let Err(e) = send_message(&tg, &reply).await {
                    error!("명령 회신 전송 실패: {e}");
                }
            }
        }
    }
    info!("명령 폴러 종료");
}

/// 명령 처리 → 공유 리스트 갱신 + 영속화 → 회신 텍스트. 수급 조회에 client 사용.
async fn handle_command(client: &KisClient, cmd: Command, shared: &Shared, pick: Option<usize>) -> String {
    match cmd {
        Command::Help => HELP_TEXT.to_string(),
        Command::Supply(queries) => handle_supply(client, queries, pick).await,
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
        Command::Sync => sync_symbols().await,
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
                msg.push_str("마스터가 오래되었다면 /sync 로 갱신 후 다시 시도하세요.\n");
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

/// 종목 마스터 DB 수동 갱신. `/add` 가 "일치 종목 없음"으로 실패할 때 사용.
/// 데이터를 강제로 다시 받는다(if_stale=false). 진행 로그는 stdout(데몬 로그)으로.
async fn sync_symbols() -> String {
    let path = match kis_core::config::symbols_db_path() {
        Ok(p) => p,
        Err(e) => return format!("⚠️ 심볼 DB 경로 확인 실패: {e}"),
    };
    match kis_data::symbols::sync::sync_all(&path, false).await {
        Ok(report) => {
            if report.results.is_empty() {
                return "동기화할 항목이 없습니다.".into();
            }
            let mut ok = 0usize;
            let mut lines = Vec::with_capacity(report.results.len());
            for r in &report.results {
                match &r.error {
                    None => {
                        ok += 1;
                        lines.push(format!("• {} {}건", r.market.as_str(), r.count));
                    }
                    Some(e) => lines.push(format!("• {} 실패 — {}", r.market.as_str(), esc(e))),
                }
            }
            format!(
                "🔄 심볼 마스터 동기화 완료 ({}/{})\n{}",
                ok,
                report.results.len(),
                lines.join("\n"),
            )
        }
        Err(e) => format!("⚠️ 동기화 실패: {e}"),
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

async fn render(client: &KisClient, watches: &[Watch], interval_secs: u64, in_session: bool) -> String {
    let now = session::now_kst();
    let header = format!(
        "관심종목  {}({}) {}",
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

    // 종목(좌) · 현재가(우) · 등락(화살표 정렬 위해 좌측) 정렬 표.
    let headers: Vec<(&str, bool)> = vec![("종목", false), ("현재가", true), ("등락", false)];
    let grid_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|r| vec![r.name.clone(), r.price.clone(), r.change.clone()])
        .collect();
    let body = render_grid(&headers, &grid_rows);

    let footer = if in_session {
        format!("{interval_secs}초마다 갱신")
    } else {
        "세션 밖 · 마지막 체결가".to_string()
    };

    // 국장 수급 — 관심종목에 국내 종목이 하나라도 있으면 두 블록을 덧붙인다.
    //  ① 시장 순매수: 코스피/코스닥 × (외인·개인·기관계 + 기관 세분화: 금투/투신/연기금/사모/…)
    //     데이터는 시세성(장중 갱신, 약 30분 간격 확정) — 세션 밖엔 당일 최종치.
    //  ② 종목별 외인·기관 잠정(추정 가집계) — 장중에만(입력시간 09:30~14:30 누계).
    // 조회 실패한 블록은 조용히 생략.
    let mut sections: Vec<String> = Vec::new();
    if watches.iter().any(|w| w.market.is_domestic()) {
        if let Some(s) = render_supply(client).await {
            sections.push(s);
        }
        if in_session {
            if let Some(e) = render_estimate(client, watches).await {
                sections.push(e);
            }
        }
    }

    if sections.is_empty() {
        format!("{header}\n\n{body}\n{footer}")
    } else {
        format!("{header}\n\n{body}\n\n{}\n{footer}", sections.join("\n\n"))
    }
}

// ─────────────────────────────────────────────────────────────
// 국장 수급 (시장별 투자자매매동향 · 시세성)
// ─────────────────────────────────────────────────────────────

/// 시장 순매수(대금)를 **구분 × 시장(코스피/코스닥)** 표로 렌더. 기관계는
/// 금투(증권)·투신·연기금·사모·보험·은행·종금·기타금융 으로 세분화해 보여준다.
/// KIS `inquire-investor-time-by-market`(FHPTJ04030000) 을 시장별 1회씩 조회.
/// 둘 다 실패하면 None (수급 블록 생략).
async fn render_supply(client: &KisClient) -> Option<String> {
    // 시장구분 KSP(코스피)/KSQ(코스닥), 업종구분 0001/1001 = 각 종합.
    let kospi = fetch_supply(client, "KSP", "0001").await;
    let kosdaq = fetch_supply(client, "KSQ", "1001").await;

    // 사용 가능한 시장만 열로 구성 (한 쪽만 조회되어도 표시).
    let mut cols: Vec<(&'static str, inv_by_market::Response)> = Vec::new();
    if let Some(k) = kospi {
        cols.push(("코스피", k));
    }
    if let Some(k) = kosdaq {
        cols.push(("코스닥", k));
    }
    if cols.is_empty() {
        return None;
    }

    // (라벨, 순매수 대금 필드 선택자) — 기관계 하위는 `· ` 들여쓰기.
    type Sel = fn(&inv_by_market::Response) -> &str;
    let spec: &[(&str, Sel)] = &[
        ("외국인", |r| &r.frgn_ntby_tr_pbmn),
        ("개인", |r| &r.prsn_ntby_tr_pbmn),
        ("기관계", |r| &r.orgn_ntby_tr_pbmn),
        ("· 금투", |r| &r.scrt_ntby_tr_pbmn),
        ("· 투신", |r| &r.ivtr_ntby_tr_pbmn),
        ("· 연기금", |r| &r.fund_ntby_tr_pbmn),
        ("· 사모", |r| &r.pe_fund_ntby_tr_pbmn),
        ("· 보험", |r| &r.insu_ntby_tr_pbmn),
        ("· 은행", |r| &r.bank_ntby_tr_pbmn),
        ("· 종금", |r| &r.mrbn_ntby_tr_pbmn),
        ("· 기타금융", |r| &r.etc_orgt_ntby_tr_pbmn),
        ("기타법인", |r| &r.etc_corp_ntby_tr_pbmn),
    ];

    // 셀 문자열 미리 계산 ([행][열]).
    let cells: Vec<Vec<String>> = spec
        .iter()
        .map(|(_, sel)| cols.iter().map(|(_, r)| fmt_eok(sel(r))).collect())
        .collect();

    const H_GB: &str = "구분";
    // 구분(좌측) + 시장별(우측) 그리드 표.
    let mut headers: Vec<(&str, bool)> = vec![(H_GB, false)];
    for (h, _) in &cols {
        headers.push((*h, true));
    }
    let grid_rows: Vec<Vec<String>> = spec
        .iter()
        .enumerate()
        .map(|(ri, (label, _))| {
            let mut row = vec![label.to_string()];
            row.extend((0..cols.len()).map(|ci| cells[ri][ci].clone()));
            row
        })
        .collect();
    let t = render_grid(&headers, &grid_rows);

    Some(format!("📊 국장 수급 · 순매수(억원)\n{t}"))
}

/// 관심 국내종목별 외인·기관 **잠정(추정 가집계)** 순매수를 표로 렌더.
/// KIS `investor-trend-estimate`(HHPTJ04160200) — 증권사 직원이 장중 입력한 누계 추정치.
/// 입력시간: 외국인 09:30/11:20/13:20/14:30, 기관 10:00/11:20/13:20/14:30.
/// 각 종목의 최신(가장 큰 입력구분) 1건만 표시. 모두 실패/빈 값이면 None.
async fn render_estimate(client: &KisClient, watches: &[Watch]) -> Option<String> {
    struct ERow {
        name: String,
        prsn: String,
        frgn: String,
        orgn: String,
    }
    let mut rows: Vec<ERow> = Vec::new();
    let mut last_hour = String::new();
    for w in watches.iter().filter(|w| w.market.is_domestic()) {
        let req = inv_estimate::Request { mksc_shrn_iscd: w.code.clone() };
        match inv_estimate::call(client, &req).await {
            Ok(list) => {
                // 가장 늦은 입력구분(bsop_hour_gb 최대) = 최신 누계.
                if let Some(latest) = list.into_iter().max_by(|a, b| a.bsop_hour_gb.cmp(&b.bsop_hour_gb)) {
                    if latest.bsop_hour_gb > last_hour {
                        last_hour = latest.bsop_hour_gb.clone();
                    }
                    // 가집계는 외인·기관만 제공 → 개인(추정)은 -(외인+기관) 로 근사.
                    let frgn_q = latest.frgn_fake_ntby_qty.trim().parse::<f64>().unwrap_or(0.0);
                    let orgn_q = latest.orgn_fake_ntby_qty.trim().parse::<f64>().unwrap_or(0.0);
                    rows.push(ERow {
                        name: w.name.clone(),
                        prsn: fmt_man(&format!("{}", -(frgn_q + orgn_q))),
                        frgn: fmt_man(&latest.frgn_fake_ntby_qty),
                        orgn: fmt_man(&latest.orgn_fake_ntby_qty),
                    });
                }
            }
            Err(e) => warn!("[{}] {} 추정 수급 조회 실패: {e}", w.code, w.name),
        }
    }
    if rows.is_empty() {
        return None;
    }

    const H_NAME: &str = "종목";
    const H_PRSN: &str = "개인";
    const H_FRGN: &str = "외인";
    const H_ORGN: &str = "기관";
    // 개인은 가집계가 직접 주지 않아 -(외인+기관) 추정치.
    let headers: Vec<(&str, bool)> =
        vec![(H_NAME, false), (H_PRSN, true), (H_FRGN, true), (H_ORGN, true)];
    let grid_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|r| vec![r.name.clone(), r.prsn.clone(), r.frgn.clone(), r.orgn.clone()])
        .collect();
    let t = render_grid(&headers, &grid_rows);

    let hour = hour_label(&last_hour);
    let title = if hour.is_empty() {
        "🕒 종목별 개인·외인·기관 잠정(추정·만주)".to_string()
    } else {
        format!("🕒 종목별 개인·외인·기관 잠정(추정·만주) · {hour} 누계")
    };
    Some(format!("{title}\n{t}"))
}

/// 시장별 투자자매매동향(시세) 1건 조회. 실패는 로그만 남기고 None.
async fn fetch_supply(
    client: &KisClient,
    iscd: &str,
    iscd2: &str,
) -> Option<inv_by_market::Response> {
    let req = inv_by_market::Request {
        fid_input_iscd: iscd.into(),
        fid_input_iscd_2: iscd2.into(),
    };
    match inv_by_market::call(client, &req).await {
        Ok(r) => Some(r),
        Err(e) => {
            warn!("국장 수급 조회 실패 ({iscd}): {e}");
            None
        }
    }
}

/// 순매수 대금(백만원) → 억원 반올림 + 천단위 + 부호. 양수 앞에 '+', 음수는 '-' 유지.
/// KIS 투자자별 `*_ntby_tr_pbmn` 필드는 백만원 단위이므로 억원 = 백만원 / 100.
/// 파싱 실패 시 원문 그대로.
fn fmt_eok(mn_won: &str) -> String {
    match mn_won.trim().parse::<f64>() {
        Ok(v) => {
            let mut eok = (v / 100.0).round();
            if eok == 0.0 {
                eok = 0.0; // -0.0 정규화
            }
            let num = format_number(&format!("{eok:.0}"));
            if eok > 0.0 {
                format!("+{num}")
            } else {
                num
            }
        }
        Err(_) => mn_won.trim().to_string(),
    }
}

/// 추정 순매수 수량(주) → 만주(소수 1자리) + 부호. 양수 앞에 '+', 음수는 '-' 유지.
/// 파싱 실패 시 원문 그대로.
fn fmt_man(qty: &str) -> String {
    match qty.trim().parse::<f64>() {
        Ok(v) => {
            let mut man = v / 1e4;
            if man == 0.0 {
                man = 0.0; // -0.0 정규화
            }
            let num = format_number(&format!("{man:.1}"));
            if man > 0.0 {
                format!("+{num}")
            } else {
                num
            }
        }
        Err(_) => qty.trim().to_string(),
    }
}

/// 추정가집계 입력구분(bsop_hour_gb) → 입력 시각 라벨. 미지정 값은 빈 문자열.
fn hour_label(gb: &str) -> &'static str {
    match gb {
        "1" => "09:30",
        "2" => "10:00",
        "3" => "11:20",
        "4" => "13:20",
        "5" => "14:30",
        _ => "",
    }
}

// ─────────────────────────────────────────────────────────────
// /su 명령 — 온디맨드 수급 조회 (시장 전체 / 개별종목)
// ─────────────────────────────────────────────────────────────

/// `/su` 처리. 인자 없으면 코스피/코스닥 전체 수급, 있으면 각 국내 개별종목 수급.
async fn handle_supply(client: &KisClient, queries: Vec<String>, pick: Option<usize>) -> String {
    // 인자 없음 → 시장 전체.
    if queries.is_empty() {
        return render_supply(client)
            .await
            .unwrap_or_else(|| "국장 수급 조회 실패 — 잠시 후 다시 시도하세요.".into());
    }
    // 인자 있음 → 개별종목.
    let mut out: Vec<String> = Vec::new();
    for q in &queries {
        match resolve_one(q, pick) {
            Ok(w) if w.market.is_domestic() => match render_stock_supply(client, &w).await {
                Some(s) => out.push(s),
                None => out.push(format!(
                    "{} ({}) 수급 데이터 없음 (당일 일별은 장 종료 후, 장중은 잠정만 제공)",
                    esc(&w.name), w.code
                )),
            },
            Ok(w) => out.push(format!("{} 은(는) 해외 종목 — 국장 수급 미지원", esc(&w.name))),
            Err(e) => out.push(format!("{q}: {e}")),
        }
    }
    out.join("\n\n")
}

/// 개별종목 수급 — 일별 투자자매매동향(기관 세분화, 억원) + 장중 외인/기관 잠정(추정).
/// 일별: `investor-trade-by-stock-daily`(FHPTJ04160001), 잠정: `investor-trend-estimate`(HHPTJ04160200).
/// 둘 다 비면 None.
async fn render_stock_supply(client: &KisClient, w: &Watch) -> Option<String> {
    let today = session::now_kst().format("%Y%m%d").to_string();

    // 일별 — 가장 최근 영업일 1행.
    let daily = {
        let req = inv_by_stock::Request {
            fid_cond_mrkt_div_code: "J".into(),
            fid_input_iscd: w.code.clone(),
            fid_input_date_1: today,
            fid_org_adj_prc: String::new(),
            fid_etc_cls_code: "1".into(),
        };
        match inv_by_stock::call(client, &req).await {
            Ok(resp) => resp
                .rows
                .into_iter()
                .max_by(|a, b| a.stck_bsop_date.cmp(&b.stck_bsop_date)),
            Err(e) => {
                warn!("[{}] {} 종목 수급(일별) 조회 실패: {e}", w.code, w.name);
                None
            }
        }
    };

    // 잠정(추정) — 최신 입력구분 1건.
    let est = {
        let req = inv_estimate::Request { mksc_shrn_iscd: w.code.clone() };
        match inv_estimate::call(client, &req).await {
            Ok(list) => list
                .into_iter()
                .max_by(|a, b| a.bsop_hour_gb.cmp(&b.bsop_hour_gb)),
            Err(_) => None,
        }
    };

    if daily.is_none() && est.is_none() {
        return None;
    }

    let mut parts: Vec<String> = Vec::new();

    if let Some(r) = &daily {
        type Sel = fn(&inv_by_stock::Row) -> &str;
        let spec: &[(&str, Sel)] = &[
            ("외국인", |r| &r.frgn_ntby_tr_pbmn),
            ("개인", |r| &r.prsn_ntby_tr_pbmn),
            ("기관계", |r| &r.orgn_ntby_tr_pbmn),
            ("· 금투", |r| &r.scrt_ntby_tr_pbmn),
            ("· 투신", |r| &r.ivtr_ntby_tr_pbmn),
            ("· 연기금", |r| &r.fund_ntby_tr_pbmn),
            ("· 사모", |r| &r.pe_fund_ntby_tr_pbmn),
            ("· 보험", |r| &r.insu_ntby_tr_pbmn),
            ("· 은행", |r| &r.bank_ntby_tr_pbmn),
            ("· 종금", |r| &r.mrbn_ntby_tr_pbmn),
            ("· 기타금융", |r| &r.etc_orgt_ntby_tr_pbmn),
            ("기타법인", |r| &r.etc_corp_ntby_tr_pbmn),
        ];
        let labels: Vec<&str> = spec.iter().map(|(l, _)| *l).collect();
        let vals: Vec<String> = spec.iter().map(|(_, s)| fmt_eok(s(r))).collect();
        let table = kv_table("국분", "순매수", &labels, &vals);
        parts.push(format!(
            "📊 {} 수급 · 순매수(억원) · {}\n{}",
            esc(&w.name),
            fmt_date(&r.stck_bsop_date),
            table
        ));
    }

    if let Some(e) = &est {
        let hour = hour_label(&e.bsop_hour_gb);
        let when = if hour.is_empty() { String::new() } else { format!(" {hour} 누계") };
        parts.push(format!(
            "🕒 장중 잠정(추정·만주){} — 외인 {} / 기관 {}",
            when,
            fmt_man(&e.frgn_fake_ntby_qty),
            fmt_man(&e.orgn_fake_ntby_qty),
        ));
    }

    Some(parts.join("\n\n"))
}

/// (라벨, 값) 리스트를 2열 <pre> 표로. 라벨 좌측, 값 우측 정렬.
fn kv_table(h_label: &str, h_val: &str, labels: &[&str], vals: &[String]) -> String {
    let label_w = labels
        .iter()
        .map(|s| display_width(s))
        .chain(std::iter::once(display_width(h_label)))
        .max()
        .unwrap_or(0);
    let val_w = vals
        .iter()
        .map(|s| display_width(s))
        .chain(std::iter::once(display_width(h_val)))
        .max()
        .unwrap_or(0);
    let mut t = String::new();
    t.push_str("<pre>");
    t.push_str(&pad_to(h_label, label_w));
    t.push_str("  ");
    t.push_str(&pad_left(h_val, val_w));
    t.push('\n');
    t.push_str(&"─".repeat(label_w + 2 + val_w));
    t.push('\n');
    for (l, v) in labels.iter().zip(vals.iter()) {
        t.push_str(&pad_to(l, label_w));
        t.push_str("  ");
        t.push_str(&pad_left(v, val_w));
        t.push('\n');
    }
    t.push_str("</pre>");
    t
}

/// "20250812" → "08/12". 길이가 다르면 원문.
fn fmt_date(yyyymmdd: &str) -> String {
    let s = yyyymmdd.trim();
    if s.len() == 8 {
        format!("{}/{}", &s[4..6], &s[6..8])
    } else {
        s.to_string()
    }
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

/// 표시 폭 기준 우측 정렬 (앞쪽에 공백 패딩). 현재가 열 정렬용.
fn pad_left(s: &str, width: usize) -> String {
    let w = display_width(s);
    if w >= width {
        s.to_string()
    } else {
        format!("{}{}", " ".repeat(width - w), s)
    }
}

/// 정렬된 표를 `<pre>` 로 렌더 — 세로 경계선 없이 공백 정렬 + 헤더 아래 가로구분선만.
/// (박스드로잉 세로줄은 텔레그램 폰트에서 한글과 폭이 안 맞아 어꺋나므로 사용하지 않는다.)
/// `cols`: (헤더, 우측정렬여부). `rows`: 각 행의 셀 문자열(열 수는 `cols` 와 동일 가정).
/// 셀 내용은 내부에서 HTML escape 한다.
fn render_grid(cols: &[(&str, bool)], rows: &[Vec<String>]) -> String {
    let n = cols.len();
    // 열별 표시폭 = 헤더/셀 최댓값.
    let mut w = vec![0usize; n];
    for (i, (h, _)) in cols.iter().enumerate() {
        w[i] = display_width(h);
    }
    for row in rows {
        for (i, cell) in row.iter().take(n).enumerate() {
            w[i] = w[i].max(display_width(cell));
        }
    }
    // 한 행 — 첫 열은 바로, 이후 열은 앞에 공백 2칸 띄우고 정렬.
    let line = |cells: &[String]| -> String {
        let mut s = String::new();
        for (i, (_, right)) in cols.iter().enumerate() {
            if i > 0 {
                s.push_str("  ");
            }
            let raw = cells.get(i).map(String::as_str).unwrap_or("");
            let padded = if *right { pad_left(raw, w[i]) } else { pad_to(raw, w[i]) };
            s.push_str(&esc(&padded));
        }
        s.push('\n');
        s
    };

    let header: Vec<String> = cols.iter().map(|(h, _)| (*h).to_string()).collect();
    let total_w = w.iter().sum::<usize>() + 2 * n.saturating_sub(1);
    let mut t = String::from("<pre>");
    t.push_str(&line(&header));
    t.push_str(&"─".repeat(total_w));
    t.push('\n');
    for row in rows {
        t.push_str(&line(row));
    }
    t.push_str("</pre>");
    t
}


// ─────────────────────────────────────────────────────────────
// `--background` — systemd user unit (Linux 전용, 단일 서비스)
// ─────────────────────────────────────────────────────────────

const SERVICE_NAME: &str = "kis-brief-stream";

fn unit_path() -> String {
    format!("/etc/systemd/system/{SERVICE_NAME}.service")
}

fn install_systemd_unit(interval_secs: u64) -> Result<()> {
    let run_user = std::env::var("SUDO_USER")
        .ok()
        .or_else(|| std::env::var("USER").ok())
        .ok_or_else(|| anyhow!("$USER 를 읽을 수 없습니다"))?;

    let exe = std::env::current_exe()?;
    // 종목은 brief-stream.toml 에서 읽으므로 ExecStart 에 박지 않는다.
    let exec_start = format!(
        "{} brief stream --interval {interval_secs}",
        shell_escape(&exe.to_string_lossy())
    );

    let unit = format!(
        "[Unit]\n\
         Description=kis-cli brief stream (관심종목 라이브)\n\
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
        println!("  sudo $(which kis) brief stream --interval {interval_secs} --background");
        println!("종목은 텔레그램에서 /add 로 등록하거나 brief-stream.toml 을 편집하세요.");
        return Ok(());
    }

    match std::fs::write(&path, &unit) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            return Err(anyhow!(
                "{path} 에 쓰기 권한이 없습니다. 재실행: sudo $(which kis) brief stream --interval {interval_secs} --background"
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
    println!("제거:      sudo $(which kis) brief remove");
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
    println!("제거: sudo $(which kis) brief remove");
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
            return Err(anyhow!("{path} 삭제 권한이 없습니다. 재실행: sudo $(which kis) brief remove"));
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
        assert_eq!(parse_command("/sync"), Some(Command::Sync));
        assert_eq!(parse_command("/symbols"), Some(Command::Sync));
        // 수급 — 인자 없으면 시장 전체, 있으면 개별종목
        assert_eq!(parse_command("/su"), Some(Command::Supply(vec![])));
        assert_eq!(parse_command("/su 삼성전자"), Some(Command::Supply(vec!["삼성전자".into()])));
        assert_eq!(parse_command("/flow 005930"), Some(Command::Supply(vec!["005930".into()])));
        assert_eq!(parse_command("/sg"), Some(Command::Supply(vec![])));
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

    #[test]
    fn fmt_eok_signs_and_rounding() {
        // 순매수(원) → 억원 반올림 + 천단위 + 부호
        assert_eq!(fmt_eok("123400"), "+1,234"); // 123,400백만원 = +1,234억
        assert_eq!(fmt_eok("-56700"), "-567");    // -56,700백만원 = -567억
        assert_eq!(fmt_eok("0"), "0");             // 보합
        assert_eq!(fmt_eok("-4"), "0");            // -0.04억 → 반올림 0 (음수부호 없음)
        assert_eq!(fmt_eok("n/a"), "n/a");         // 파싱 실패 → 원문
    }

    #[test]
    fn fmt_man_signs_and_rounding() {
        // 추정 순매수(주) → 만주(소수 1자리) + 부호
        assert_eq!(fmt_man("123000"), "+12.3");   // +12.3만주
        assert_eq!(fmt_man("-45000"), "-4.5");     // -4.5만주
        assert_eq!(fmt_man("0"), "0.0");           // 보합
        assert_eq!(fmt_man("-500"), "-0.1");       // -0.05만 → 반올림 -0.1
        assert_eq!(fmt_man("1234500"), "+123.5");  // 천단위는 없고 소수 1자리
        assert_eq!(fmt_man("n/a"), "n/a");         // 파싱 실패 → 원문
    }

    #[test]
    fn fmt_date_yyyymmdd() {
        assert_eq!(fmt_date("20250812"), "08/12");
        assert_eq!(fmt_date("2025"), "2025"); // 비규격 → 원문
    }

    #[test]
    fn hour_label_maps_input_slots() {
        assert_eq!(hour_label("1"), "09:30");
        assert_eq!(hour_label("5"), "14:30");
        assert_eq!(hour_label("9"), ""); // 미지정
    }

    #[test]
    fn backoff_grows_then_caps_at_60() {
        assert_eq!(backoff_for(1), 2);
        assert_eq!(backoff_for(2), 4);
        assert_eq!(backoff_for(3), 8);
        assert_eq!(backoff_for(4), 16);
        assert_eq!(backoff_for(5), 32);
        assert_eq!(backoff_for(6), 60); // 64 → cap 60
        assert_eq!(backoff_for(7), 60); // 상한 유지
        assert_eq!(backoff_for(100), 60);
    }
}
