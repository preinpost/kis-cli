use std::sync::Arc;

use aes::cipher::{BlockDecryptMut, KeyIvInit};
use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;

use crate::token::TokenManager;

const WS_URL_REAL: &str = "ws://ops.koreainvestment.com:21000";

/// 국내주식 실시간체결 TR_ID
const WS_TR_ID_DOMESTIC_CCNL: &str = "H0STCNT0";

/// 해외주식 실시간체결 TR_ID
const WS_TR_ID_OVERSEAS_CCNL: &str = "HDFSCNT0";

/// KRX 야간선물 실시간체결 TR_ID
const WS_TR_ID_NIGHT_FUTURES_CCNL: &str = "H0MFCNT0";

/// 국내주식 실시간체결 컬럼 (주요 필드만)
const DOMESTIC_CCNL_COLUMNS: &[&str] = &[
    "종목코드",
    "체결시간",
    "현재가",
    "전일대비부호",
    "전일대비",
    "전일대비율",
    "가중평균가",
    "시가",
    "고가",
    "저가",
    "매도호가",
    "매수호가",
    "체결량",
    "누적거래량",
    "누적거래대금",
    "매도체결건수",
    "매수체결건수",
    "순매수체결건수",
    "체결강도",
    "총매도수량",
    "총매수수량",
];

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

struct DecryptInfo {
    key: String,
    iv: String,
}

/// 파싱된 실시간 체결 틱 (라이브러리 소비자용 — SSE 릴레이 등).
#[derive(Clone, Debug, Serialize)]
pub struct Tick {
    pub market: &'static str, // "domestic" | "overseas"
    pub symbol: String,       // 국내 6자리코드 / 해외 티커
    pub time: String,
    pub price: String,
    pub sign: String, // KIS 전일대비부호 코드 (1상한·2상승·3보합·4하한·5하락)
    pub diff: String,
    pub rate: String,
    pub volume: String,
}

/// 한 WS 연결에서 구독할 종목.
#[derive(Clone, Debug)]
pub enum Sub {
    Domestic(String),                       // 국내 6자리 코드
    Overseas { excd: String, symbol: String }, // excd: NAS/NYS/AMS
}

/// 틱 출력 방식. CLI=Print(stdout), 웹/라이브러리=Channel(mpsc).
enum Sink {
    Print { header_printed: bool },
    Channel(mpsc::Sender<Tick>),
}

pub async fn run_domestic(token_manager: Arc<TokenManager>, symbol: &str) -> Result<()> {
    let approval_key = token_manager
        .get_ws_approval_key_string()
        .await
        .context("WebSocket approval key 발급 실패")?;

    let url = format!("{WS_URL_REAL}/tryitout");
    println!("[{symbol}] 실시간 체결가 스트리밍 시작...");
    println!("종료: Ctrl+C\n");

    let subs = [(WS_TR_ID_DOMESTIC_CCNL, symbol.to_string())];
    let mut sink = Sink::Print { header_printed: false };
    let max_retries = 3;
    let mut retry_count = 0;

    while retry_count < max_retries {
        match connect_and_stream(&url, &approval_key, &subs, &mut sink).await {
            Ok(()) => break,
            Err(e) => {
                retry_count += 1;
                eprintln!("[WS] 연결 끊김 ({retry_count}/{max_retries}): {e}");
                if retry_count < max_retries {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    if retry_count >= max_retries {
        anyhow::bail!("최대 재시도 횟수 ({max_retries}) 초과");
    }

    Ok(())
}

/// KRX 야간선물 실시간체결 (H0MFCNT0). 모의투자 미지원. `tr_key` = 야간선물 종목코드.
pub async fn run_night_futures(
    token_manager: Arc<TokenManager>,
    symbol: &str,
) -> Result<()> {
    let approval_key = token_manager
        .get_ws_approval_key_string()
        .await
        .context("WebSocket approval key 발급 실패")?;

    let url = format!("{WS_URL_REAL}/tryitout");
    println!("[{symbol}] KRX 야간선물 실시간 체결 스트리밍 시작...");
    println!("종료: Ctrl+C\n");

    let subs = [(WS_TR_ID_NIGHT_FUTURES_CCNL, symbol.to_string())];
    let mut sink = Sink::Print { header_printed: false };
    let max_retries = 3;
    let mut retry_count = 0;
    while retry_count < max_retries {
        match connect_and_stream(&url, &approval_key, &subs, &mut sink).await {
            Ok(()) => break,
            Err(e) => {
                retry_count += 1;
                eprintln!("[WS] 연결 끊김 ({retry_count}/{max_retries}): {e}");
                if retry_count < max_retries {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
    if retry_count >= max_retries {
        anyhow::bail!("최대 재시도 횟수 ({max_retries}) 초과");
    }
    Ok(())
}

/// 해외주식 실시간체결. `tr_key` = `D{EXCD}{SYMB}` — 예: `DNASTSLA`.
pub async fn run_overseas(
    token_manager: Arc<TokenManager>,
    excd: &str,
    symbol: &str,
) -> Result<()> {
    let approval_key = token_manager
        .get_ws_approval_key_string()
        .await
        .context("WebSocket approval key 발급 실패")?;

    let tr_key = format!("D{}{}", excd, symbol);
    let url = format!("{WS_URL_REAL}/tryitout");
    println!("[{excd}:{symbol}] 실시간 체결가 스트리밍 시작...");
    println!("종료: Ctrl+C\n");

    let subs = [(WS_TR_ID_OVERSEAS_CCNL, tr_key)];
    let mut sink = Sink::Print { header_printed: false };
    let max_retries = 3;
    let mut retry_count = 0;
    while retry_count < max_retries {
        match connect_and_stream(&url, &approval_key, &subs, &mut sink).await {
            Ok(()) => break,
            Err(e) => {
                retry_count += 1;
                eprintln!("[WS] 연결 끊김 ({retry_count}/{max_retries}): {e}");
                if retry_count < max_retries {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
    if retry_count >= max_retries {
        anyhow::bail!("최대 재시도 횟수 ({max_retries}) 초과");
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Feed {
    Domestic,
    Overseas,
    NightFutures,
}

fn feed_from_tr_id(tr_id: &str) -> Option<Feed> {
    match tr_id {
        WS_TR_ID_DOMESTIC_CCNL => Some(Feed::Domestic),
        WS_TR_ID_OVERSEAS_CCNL => Some(Feed::Overseas),
        WS_TR_ID_NIGHT_FUTURES_CCNL => Some(Feed::NightFutures),
        _ => None,
    }
}

fn print_header(feed: Feed) {
    match feed {
        Feed::Domestic => println!(
            "{:<8} {:>12} {:>8} {:>8}  {:>12} {:>10}",
            "시간", "현재가", "대비", "대비율", "거래량", "체결강도"
        ),
        Feed::Overseas => println!(
            "{:<10} {:>12} {:>8} {:>8}  {:>12}",
            "UTC", "현재가", "대비", "대비율", "누적거래량"
        ),
        Feed::NightFutures => println!(
            "{:<8} {:>12} {:>8} {:>8}  {:>12} {:>10}",
            "시간", "현재가", "대비", "대비율", "거래량", "미결제"
        ),
    }
    println!("─────────────────────────────────────────────────────────────");
}

/// 국내/해외 실시간 필드 → Tick. (야간선물은 채널 미지원 → None)
fn extract_tick(feed: Feed, fields: &[&str]) -> Option<Tick> {
    let get = |i: usize| fields.get(i).copied().unwrap_or("").to_string();
    match feed {
        Feed::Domestic => {
            if fields.len() <= 13 {
                return None;
            }
            Some(Tick {
                market: "domestic",
                symbol: get(0),
                time: get(1),
                price: get(2),
                sign: get(3),
                diff: get(4),
                rate: get(5),
                volume: get(13),
            })
        }
        Feed::Overseas => {
            if fields.len() <= 20 {
                return None;
            }
            // fields[0] = RSYM "D{EXCD}{TICKER}" (예: DNASTSLA) → 티커는 [4..]
            let rsym = fields[0];
            let symbol = if rsym.len() > 4 {
                rsym[4..].to_string()
            } else {
                rsym.to_string()
            };
            Some(Tick {
                market: "overseas",
                symbol,
                time: get(7),
                price: get(11),
                sign: get(12),
                diff: get(13),
                rate: get(14),
                volume: get(20),
            })
        }
        Feed::NightFutures => None,
    }
}

/// 단일 WS 연결로 여러 종목(국내/해외 혼합)을 구독하고 틱을 sink 로 흘려보낸다.
async fn connect_and_stream(
    url: &str,
    approval_key: &str,
    subs: &[(&str, String)],
    sink: &mut Sink,
) -> Result<()> {
    let (ws_stream, _) = connect_async(url).await.context("WebSocket 연결 실패")?;
    let (mut write, mut read) = ws_stream.split();

    // 구독 메시지 — sub(종목)마다 1개씩 같은 연결로 전송
    for (tr_id, tr_key) in subs {
        let subscribe_msg = serde_json::json!({
            "header": {
                "approval_key": approval_key,
                "custtype": "P",
                "tr_type": "1",
                "content-type": "utf-8",
            },
            "body": { "input": { "tr_id": tr_id, "tr_key": tr_key } },
        });
        write
            .send(Message::Text(subscribe_msg.to_string().into()))
            .await
            .context("구독 메시지 전송 실패")?;
    }

    let mut decrypt_info: Option<DecryptInfo> = None;

    while let Some(msg) = read.next().await {
        let msg = msg.context("메시지 수신 오류")?;
        let text = match msg {
            Message::Text(t) => t,
            Message::Ping(data) => {
                write.send(Message::Pong(data)).await?;
                continue;
            }
            Message::Close(_) => break,
            _ => continue,
        };

        let first_char = text.chars().next().unwrap_or(' ');

        if first_char == '0' || first_char == '1' {
            // 실시간 데이터
            let parts: Vec<&str> = text.splitn(4, '|').collect();
            if parts.len() < 4 {
                continue;
            }
            let encrypt_flag = parts[0];
            let frame_tr_id = parts[1];
            let mut data_str = parts[3].to_string();

            if encrypt_flag == "1" {
                if let Some(ref info) = decrypt_info {
                    if let Ok(decrypted) = aes_cbc_decrypt(&info.key, &info.iv, &data_str) {
                        data_str = decrypted;
                    }
                }
            }

            let fields: Vec<&str> = data_str.split('^').collect();
            let Some(feed) = feed_from_tr_id(frame_tr_id) else {
                continue;
            };

            match sink {
                Sink::Print { header_printed } => {
                    if !*header_printed {
                        print_header(feed);
                        *header_printed = true;
                    }
                    match feed {
                        Feed::Domestic => print_domestic(&fields),
                        Feed::Overseas => print_overseas(&fields),
                        Feed::NightFutures => print_night_futures(&fields),
                    }
                }
                Sink::Channel(tx) => {
                    if let Some(tick) = extract_tick(feed, &fields) {
                        // 수신측(SSE) 종료 시 send 실패 → 스트림 종료
                        if tx.send(tick).await.is_err() {
                            break;
                        }
                    }
                }
            }
        } else {
            // 시스템 메시지 (구독 응답, PINGPONG)
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                let tr_id = data["header"]["tr_id"].as_str().unwrap_or("");

                if tr_id == "PINGPONG" {
                    write
                        .send(Message::Pong(text.as_bytes().to_vec().into()))
                        .await
                        .ok();
                    continue;
                }

                if let Some(output) = data["body"]["output"].as_object() {
                    if let (Some(key), Some(iv)) = (
                        output.get("key").and_then(|v| v.as_str()),
                        output.get("iv").and_then(|v| v.as_str()),
                    ) {
                        decrypt_info = Some(DecryptInfo {
                            key: key.to_string(),
                            iv: iv.to_string(),
                        });
                    }
                }

                let rt_cd = data["body"]["rt_cd"].as_str().unwrap_or("");
                let msg = data["body"]["msg1"].as_str().unwrap_or("");
                if rt_cd == "0" {
                    eprintln!("[구독 성공] {tr_id}: {msg}");
                } else if !rt_cd.is_empty() {
                    eprintln!("[구독 실패] {tr_id}: {msg}");
                }
            }
        }
    }

    Ok(())
}

/// 라이브러리용 멀티-종목 실시간 스트림. 단일 연결로 국내+해외 혼합 구독, 틱을 채널로 전달.
/// `cancel` 취소 시 WS 연결을 닫고 종료한다(SSE 스트림 Drop 등).
pub async fn run_stream(
    token_manager: Arc<TokenManager>,
    subs: Vec<Sub>,
    tx: mpsc::Sender<Tick>,
    cancel: CancellationToken,
) -> Result<()> {
    let approval_key = token_manager
        .get_ws_approval_key_string()
        .await
        .context("WebSocket approval key 발급 실패")?;
    let url = format!("{WS_URL_REAL}/tryitout");

    let tuples: Vec<(&str, String)> = subs
        .iter()
        .map(|s| match s {
            Sub::Domestic(code) => (WS_TR_ID_DOMESTIC_CCNL, code.clone()),
            Sub::Overseas { excd, symbol } => {
                (WS_TR_ID_OVERSEAS_CCNL, format!("D{excd}{symbol}"))
            }
        })
        .collect();

    let mut sink = Sink::Channel(tx);
    let max_retries = 5;
    let mut retries = 0;

    while !cancel.is_cancelled() {
        tokio::select! {
            _ = cancel.cancelled() => break,
            res = connect_and_stream(&url, &approval_key, &tuples, &mut sink) => {
                match res {
                    Ok(()) => break, // 서버가 연결 종료
                    Err(e) => {
                        retries += 1;
                        if retries >= max_retries {
                            return Err(e);
                        }
                        tokio::select! {
                            _ = cancel.cancelled() => break,
                            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_domestic(fields: &[&str]) {
    // [1]=체결시간, [2]=현재가, [3]=전일대비부호, [4]=전일대비, [5]=전일대비율,
    // [13]=누적거래량, [18]=체결강도
    if fields.len() <= 18 { return; }
    let time = fields[1];
    let price = fields[2];
    let sign_code = fields[3];
    let diff = fields[4];
    let rate = fields[5];
    let volume = fields[13];
    let strength = fields[18];
    let sign = arrow(sign_code);
    let formatted_time = if time.len() >= 6 {
        format!("{}:{}:{}", &time[..2], &time[2..4], &time[4..6])
    } else { time.to_string() };
    println!(
        "{:<8} {:>12} {:>1}{:>7} {:>7}%  {:>12} {:>10}",
        formatted_time, price, sign, diff, rate, volume, strength
    );
}

fn print_overseas(fields: &[&str]) {
    // HDFSCNT0: [7]=한국체결시간 (KST, HHMMSS), [11]=현재가, [12]=대비부호,
    //          [13]=대비, [14]=대비율, [20]=누적거래량
    if fields.len() <= 20 { return; }
    let kst_time = fields[7];
    let price = fields[11];
    let sign_code = fields[12];
    let diff = fields[13];
    let rate = fields[14];
    let tvol = fields[20];
    let sign = arrow(sign_code);
    println!(
        "{:<10} {:>12} {:>1}{:>7} {:>7}%  {:>12}",
        fmt_utc_from_kst(kst_time),
        price, sign, diff, rate, tvol
    );
}

fn print_night_futures(fields: &[&str]) {
    // H0MFCNT0: [1]=bsop_hour, [2]=prdy_vrss, [3]=sign, [4]=prdy_ctrt, [5]=prpr,
    //          [10]=acml_vol, [18]=hts_otst_stpl_qty
    if fields.len() <= 18 { return; }
    let time = fields[1];
    let diff = fields[2];
    let sign_code = fields[3];
    let rate = fields[4];
    let price = fields[5];
    let volume = fields[10];
    let open_interest = fields[18];
    let sign = arrow(sign_code);
    let formatted_time = if time.len() >= 6 {
        format!("{}:{}:{}", &time[..2], &time[2..4], &time[4..6])
    } else { time.to_string() };
    println!(
        "{:<8} {:>12} {:>1}{:>7} {:>7}%  {:>12} {:>10}",
        formatted_time, price, sign, diff, rate, volume, open_interest
    );
}

/// KST HHMMSS → UTC HH:MM:SS (KST = UTC+9).
fn fmt_utc_from_kst(t: &str) -> String {
    if t.len() < 6 {
        return t.to_string();
    }
    let h: i32 = t[..2].parse().unwrap_or(0);
    let m = &t[2..4];
    let s = &t[4..6];
    let utc_h = (h - 9).rem_euclid(24);
    format!("{:02}:{}:{}", utc_h, m, s)
}

fn arrow(code: &str) -> &'static str {
    match code {
        "1" | "2" => "▲",
        "4" | "5" => "▼",
        _ => " ",
    }
}

pub fn aes_cbc_decrypt(key: &str, iv: &str, cipher_text: &str) -> Result<String> {
    let decoded = BASE64.decode(cipher_text).context("base64 디코딩 실패")?;
    let mut buf = decoded.clone();

    let decryptor = Aes256CbcDec::new_from_slices(key.as_bytes(), iv.as_bytes())
        .map_err(|e| anyhow::anyhow!("AES 초기화 실패: {e}"))?;

    let decrypted = decryptor
        .decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buf)
        .map_err(|e| anyhow::anyhow!("AES 복호화 실패: {e}"))?;

    String::from_utf8(decrypted.to_vec()).context("UTF-8 변환 실패")
}
