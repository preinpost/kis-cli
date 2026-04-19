use std::sync::Arc;

use aes::cipher::{BlockDecryptMut, KeyIvInit};
use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

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

pub async fn run_domestic(token_manager: Arc<TokenManager>, symbol: &str) -> Result<()> {
    let approval_key = token_manager
        .get_ws_approval_key_string()
        .await
        .context("WebSocket approval key 발급 실패")?;

    let url = format!("{WS_URL_REAL}/tryitout");
    println!("[{symbol}] 실시간 체결가 스트리밍 시작...");
    println!("종료: Ctrl+C\n");

    let max_retries = 3;
    let mut retry_count = 0;

    while retry_count < max_retries {
        match connect_and_stream(&url, &approval_key, WS_TR_ID_DOMESTIC_CCNL, symbol, Feed::Domestic).await {
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

    let max_retries = 3;
    let mut retry_count = 0;
    while retry_count < max_retries {
        match connect_and_stream(&url, &approval_key, WS_TR_ID_NIGHT_FUTURES_CCNL, symbol, Feed::NightFutures).await {
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

    let max_retries = 3;
    let mut retry_count = 0;
    while retry_count < max_retries {
        match connect_and_stream(&url, &approval_key, WS_TR_ID_OVERSEAS_CCNL, &tr_key, Feed::Overseas).await {
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

async fn connect_and_stream(
    url: &str,
    approval_key: &str,
    tr_id: &str,
    tr_key: &str,
    feed: Feed,
) -> Result<()> {
    let (ws_stream, _) = connect_async(url).await.context("WebSocket 연결 실패")?;
    let (mut write, mut read) = ws_stream.split();

    // 구독 메시지 전송
    let subscribe_msg = serde_json::json!({
        "header": {
            "approval_key": approval_key,
            "custtype": "P",
            "tr_type": "1",
            "content-type": "utf-8",
        },
        "body": {
            "input": {
                "tr_id": tr_id,
                "tr_key": tr_key,
            },
        },
    });

    write
        .send(Message::Text(subscribe_msg.to_string().into()))
        .await
        .context("구독 메시지 전송 실패")?;

    let mut decrypt_info: Option<DecryptInfo> = None;
    let mut header_printed = false;

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
            let _tr_id = parts[1];
            let _count = parts[2];
            let mut data_str = parts[3].to_string();

            // 암호화된 데이터 복호화
            if encrypt_flag == "1" {
                if let Some(ref info) = decrypt_info {
                    if let Ok(decrypted) = aes_cbc_decrypt(&info.key, &info.iv, &data_str) {
                        data_str = decrypted;
                    }
                }
            }

            let fields: Vec<&str> = data_str.split('^').collect();

            if !header_printed {
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
                header_printed = true;
            }

            match feed {
                Feed::Domestic => print_domestic(&fields),
                Feed::Overseas => print_overseas(&fields),
                Feed::NightFutures => print_night_futures(&fields),
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

                // 복호화 키 저장
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
