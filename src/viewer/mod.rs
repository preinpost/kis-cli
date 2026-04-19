//! wry 기반 차트 뷰어.
//!
//! - main 스레드에서 tao 이벤트 루프 실행 (macOS AppKit 요구)
//! - JS → `window.ipc.postMessage(JSON)` → Rust handler
//! - Rust → async KIS 호출 → `EventLoopProxy`로 `EvalScript` 전달 → `webview.evaluate_script(...)`

use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use serde::Deserialize;
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use tokio::runtime::Handle;
use wry::WebViewBuilder;

use crate::client::KisClient;
use crate::commands::analyze::{self, Series};
use crate::commands::backtest;
use crate::symbols::{Market, ResolveMode, Store};

#[derive(Debug, Clone)]
pub enum UserEvent {
    EvalScript(String),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum IpcMsg {
    #[serde(rename = "period")]
    Period { value: String },
    #[serde(rename = "loadMore")]
    LoadMore,
    #[serde(rename = "search")]
    Search { query: String },
    #[serde(rename = "select")]
    Select { code: String, market: String },
}

pub struct ViewerCtx {
    pub rt: Handle,
    pub client: Arc<KisClient>,
    pub store: Arc<Mutex<Store>>,
    pub state: Arc<Mutex<ViewerState>>,
}

pub struct ViewerState {
    pub series: Series,
    pub period: char,
    pub symbol_code: String,
    pub symbol_name: String,
    pub mode: ResolveMode,
}

// ─── 백테스트 인터랙티브 뷰어 ──────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum BacktestIpc {
    #[serde(rename = "run")]
    Run { params: backtest::IpcParams },
}

pub struct BacktestCtx {
    pub rt: Handle,
    pub client: Arc<KisClient>,
    pub code: String,
    pub name: String,
    pub mode: ResolveMode,
    pub series: Arc<Mutex<Series>>,
    pub period: Arc<Mutex<char>>,
    pub from: Arc<Mutex<Option<String>>>,
    pub to: Arc<Mutex<Option<String>>>,
}

pub fn launch_backtest(title: &str, html: &str, ctx: BacktestCtx) -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1440.0, 860.0))
        .build(&event_loop)
        .context("창 생성 실패")?;

    let ctx = Arc::new(ctx);
    let ipc_ctx = ctx.clone();
    let ipc_proxy = proxy.clone();

    let webview = WebViewBuilder::new()
        .with_html(html)
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            let body = req.into_body();
            let msg: BacktestIpc = match serde_json::from_str(&body) {
                Ok(m) => m,
                Err(e) => {
                    let err = format!("IPC parse: {e}").replace('\'', "\\'");
                    let _ = ipc_proxy.send_event(UserEvent::EvalScript(
                        format!("window.onBacktestError('{err}');"),
                    ));
                    return;
                }
            };
            handle_backtest_ipc(msg, ipc_ctx.clone(), ipc_proxy.clone());
        })
        .build(&window)
        .context("webview 생성 실패")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::EvalScript(script)) => {
                let _ = webview.evaluate_script(&script);
            }
            _ => {}
        }
    });
}

fn handle_backtest_ipc(
    msg: BacktestIpc,
    ctx: Arc<BacktestCtx>,
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
) {
    match msg {
        BacktestIpc::Run { params } => {
            let ctx_clone = ctx.clone();
            ctx.rt.spawn(async move {
                let new_period = params.period_char();
                let new_from = params.from_norm();
                let new_to = params.to_norm();

                let (current_period, current_from, current_to) = {
                    let p = *ctx_clone.period.lock().unwrap();
                    let f = ctx_clone.from.lock().unwrap().clone();
                    let t = ctx_clone.to.lock().unwrap().clone();
                    (p, f, t)
                };

                // period / from / to 중 하나라도 바뀌면 재fetch
                if new_period != current_period
                    || new_from != current_from
                    || new_to != current_to
                {
                    match backtest::fetch_series_range(
                        &ctx_clone.client,
                        &ctx_clone.code,
                        ctx_clone.mode,
                        new_period,
                        new_from.as_deref(),
                        new_to.as_deref(),
                    )
                    .await
                    {
                        Ok(new_series) => {
                            *ctx_clone.series.lock().unwrap() = new_series;
                            *ctx_clone.period.lock().unwrap() = new_period;
                            *ctx_clone.from.lock().unwrap() = new_from.clone();
                            *ctx_clone.to.lock().unwrap() = new_to.clone();
                        }
                        Err(e) => {
                            let msg = e.to_string().replace('\'', "\\'").replace('\n', " ");
                            let _ = proxy.send_event(UserEvent::EvalScript(format!(
                                "window.onBacktestError('{msg}');"
                            )));
                            return;
                        }
                    }
                }

                let from_snap = ctx_clone.from.lock().unwrap().clone();
                let to_snap = ctx_clone.to.lock().unwrap().clone();
                let p = params.into_params(from_snap, to_snap);
                let series = ctx_clone.series.lock().unwrap();
                if series.closes.len() < 30 {
                    let _ = proxy.send_event(UserEvent::EvalScript(format!(
                        "window.onBacktestError('데이터 부족 ({}봉) — 최소 30봉 필요');",
                        series.closes.len()
                    )));
                    return;
                }
                let json = backtest::compute_payload_json(&ctx_clone.code, &ctx_clone.name, &series, &p);
                let script = format!("window.onBacktestData({});", json);
                let _ = proxy.send_event(UserEvent::EvalScript(script));
            });
        }
    }
}

/// IPC가 필요 없는 정적 차트 (백테스트 결과 등)용 간이 런처.
pub fn launch_static(title: &str, html: &str) -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1440.0, 860.0))
        .build(&event_loop)
        .context("창 생성 실패")?;
    let _webview = WebViewBuilder::new()
        .with_html(html)
        .build(&window)
        .context("webview 생성 실패")?;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
            *control_flow = ControlFlow::Exit;
        }
    });
}

pub fn launch(title: &str, html: &str, ctx: ViewerCtx) -> Result<()> {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(LogicalSize::new(1280.0, 820.0))
        .build(&event_loop)
        .context("창 생성 실패")?;

    let ctx = Arc::new(ctx);
    let ipc_ctx = ctx.clone();
    let ipc_proxy = proxy.clone();

    let webview = WebViewBuilder::new()
        .with_html(html)
        .with_ipc_handler(move |req: wry::http::Request<String>| {
            let body = req.into_body();
            let msg: IpcMsg = match serde_json::from_str(&body) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("IPC parse error: {e} — body={body}");
                    return;
                }
            };
            handle_ipc(msg, ipc_ctx.clone(), ipc_proxy.clone());
        })
        .build(&window)
        .context("webview 생성 실패")?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::EvalScript(script)) => {
                let _ = webview.evaluate_script(&script);
            }
            _ => {}
        }
    });
}

fn handle_ipc(
    msg: IpcMsg,
    ctx: Arc<ViewerCtx>,
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
) {
    match msg {
        IpcMsg::Period { value } => {
            let period = value.chars().next().unwrap_or('D');
            let ctx_clone = ctx.clone();
            ctx.rt.spawn(async move {
                let result = fetch_initial(&ctx_clone, period).await;
                send_chart_result(proxy, result);
            });
        }
        IpcMsg::LoadMore => {
            let ctx_clone = ctx.clone();
            ctx.rt.spawn(async move {
                let result = fetch_older(&ctx_clone).await;
                send_chart_result(proxy, result);
            });
        }
        IpcMsg::Search { query } => {
            let ctx_clone = ctx.clone();
            ctx.rt.spawn(async move {
                let result = search_symbols(&ctx_clone, &query);
                let script = match result {
                    Ok(json) => format!("window.onSearchResults({});", json),
                    Err(_) => "window.onSearchResults([]);".to_string(),
                };
                let _ = proxy.send_event(UserEvent::EvalScript(script));
            });
        }
        IpcMsg::Select { code, market } => {
            let ctx_clone = ctx.clone();
            ctx.rt.spawn(async move {
                let result = select_symbol(&ctx_clone, &code, &market).await;
                send_chart_result(proxy, result);
            });
        }
    }
}

fn send_chart_result(
    proxy: tao::event_loop::EventLoopProxy<UserEvent>,
    result: Result<String>,
) {
    let script = match result {
        Ok(payload_json) => format!("window.onChartData({});", payload_json),
        Err(e) => {
            let msg = e.to_string().replace('\'', "\\'").replace('\n', " ");
            format!("window.onChartError('{}');", msg)
        }
    };
    let _ = proxy.send_event(UserEvent::EvalScript(script));
}

async fn fetch_initial(ctx: &ViewerCtx, period: char) -> Result<String> {
    let (code, mode) = {
        let state = ctx.state.lock().unwrap();
        (state.symbol_code.clone(), state.mode)
    };
    let series = match mode {
        ResolveMode::Domestic => analyze::fetch_domestic_with_period(&ctx.client, &code, period).await?,
        ResolveMode::Overseas => analyze::fetch_overseas_with_period(&ctx.client, &code, period).await?,
        _ => return Err(anyhow::anyhow!("뷰어는 주식만 지원")),
    };
    let payload = payload_with_meta(&series, &ctx.state.lock().unwrap());
    {
        let mut state = ctx.state.lock().unwrap();
        state.series = series;
        state.period = period;
    }
    Ok(payload)
}

async fn fetch_older(ctx: &ViewerCtx) -> Result<String> {
    let (period, code, mode, oldest_date) = {
        let state = ctx.state.lock().unwrap();
        let oldest = state.series.dates.first().cloned();
        (state.period, state.symbol_code.clone(), state.mode, oldest)
    };
    let Some(oldest) = oldest_date else {
        return Err(anyhow::anyhow!("기존 데이터 없음"));
    };
    let end = chrono::NaiveDate::parse_from_str(&oldest, "%Y%m%d")
        .map_err(|_| anyhow::anyhow!("날짜 파싱 실패: {oldest}"))?
        - chrono::Duration::days(1);
    let to = end.format("%Y%m%d").to_string();

    let chunk = match mode {
        ResolveMode::Domestic => analyze::fetch_domestic_chunk(&ctx.client, &code, period, &to).await?,
        ResolveMode::Overseas => analyze::fetch_overseas_chunk(&ctx.client, &code, period, &to).await?,
        _ => return Err(anyhow::anyhow!("뷰어는 주식만 지원")),
    };
    if chunk.dates.is_empty() {
        return Err(anyhow::anyhow!("더 이상 과거 데이터 없음"));
    }

    let mut state = ctx.state.lock().unwrap();
    let merged = prepend_series(chunk, &state.series);
    state.series = merged;
    Ok(payload_with_meta(&state.series, &state))
}

async fn select_symbol(ctx: &ViewerCtx, code: &str, market: &str) -> Result<String> {
    let market_enum = Market::from_str(market)
        .ok_or_else(|| anyhow::anyhow!("알 수 없는 시장: {market}"))?;
    let mode = if market_enum.is_domestic() {
        ResolveMode::Domestic
    } else if market_enum.is_overseas() {
        ResolveMode::Overseas
    } else {
        return Err(anyhow::anyhow!("주식 외 시장은 미지원: {market}"));
    };

    // 종목명 찾기
    let name = {
        let store = ctx.store.lock().unwrap();
        store.find_by_code(code)?
            .into_iter()
            .find(|s| s.market == market_enum)
            .map(|s| if !s.name_kr.is_empty() { s.name_kr } else { s.name_en })
            .unwrap_or_else(|| code.to_string())
    };

    let period = { ctx.state.lock().unwrap().period };
    let series = match mode {
        ResolveMode::Domestic => analyze::fetch_domestic_with_period(&ctx.client, code, period).await?,
        ResolveMode::Overseas => analyze::fetch_overseas_with_period(&ctx.client, code, period).await?,
        _ => unreachable!(),
    };

    {
        let mut state = ctx.state.lock().unwrap();
        state.symbol_code = code.to_string();
        state.symbol_name = name.clone();
        state.mode = mode;
        state.series = series;
    }
    let state = ctx.state.lock().unwrap();
    Ok(payload_with_meta(&state.series, &state))
}

fn search_symbols(ctx: &ViewerCtx, query: &str) -> Result<String> {
    let store = ctx.store.lock().unwrap();
    let results = store.search(query, 12)?;
    let mut items = String::from("[");
    for (i, s) in results.iter().enumerate() {
        if i > 0 { items.push(','); }
        let name = if !s.name_kr.is_empty() { &s.name_kr } else { &s.name_en };
        items.push_str(&format!(
            r#"{{"code":"{}","market":"{}","name":"{}"}}"#,
            s.code.replace('"', "\\\""),
            s.market.as_str(),
            name.replace('"', "\\\""),
        ));
    }
    items.push(']');
    Ok(items)
}

fn prepend_series(older: Series, current: &Series) -> Series {
    let mut s = older;
    s.dates.extend(current.dates.iter().cloned());
    s.open.extend(current.open.iter().cloned());
    s.high.extend(current.high.iter().cloned());
    s.low.extend(current.low.iter().cloned());
    s.closes.extend(current.closes.iter().cloned());
    s.volume.extend(current.volume.iter().cloned());
    s
}

/// payload에 meta(심볼/이름/시장) 추가 — JS에서 헤더 갱신용.
fn payload_with_meta(series: &Series, state: &ViewerState) -> String {
    let base = analyze::series_to_js_payload(series);
    let market_str = match state.mode {
        ResolveMode::Domestic => "국내",
        ResolveMode::Overseas => "해외",
        _ => "",
    };
    // `{...}`에 meta 추가. 끝의 `}`를 `,"meta":{...}}`로 치환.
    let meta = format!(
        r#","meta":{{"symbol":"{}","name":"{}","market":"{}"}}}}"#,
        state.symbol_code.replace('"', "\\\""),
        state.symbol_name.replace('"', "\\\""),
        market_str,
    );
    let mut out = base;
    if out.ends_with('}') {
        out.pop();
        out.push_str(&meta);
    }
    out
}
