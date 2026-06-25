//! KIS 종목 마스터 zip 다운로드 → 해제 → 파싱 → 저장.

use std::io::Read;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};

use crate::symbols::master;
use crate::symbols::store::{Market, Store};

const BASE_URL: &str = "https://new.real.download.dws.co.kr/common/master";
const META_LAST_SYNC: &str = "last_sync_epoch";
const STALE_AFTER_SECS: u64 = 24 * 60 * 60;

struct Source {
    market: Market,
    file: &'static str,
    kind: FileKind,
}

enum FileKind {
    DomesticMst, // .mst (고정폭 바이너리)
    OverseasCod, // .cod (라인 단위 tsv-ish)
    FoMst,       // 선물옵션 .mst (121-char 고정폭)
}

const SOURCES: &[Source] = &[
    Source {
        market: Market::Kospi,
        file: "kospi_code.mst.zip",
        kind: FileKind::DomesticMst,
    },
    Source {
        market: Market::Kosdaq,
        file: "kosdaq_code.mst.zip",
        kind: FileKind::DomesticMst,
    },
    Source {
        market: Market::Nasdaq,
        file: "nasmst.cod.zip",
        kind: FileKind::OverseasCod,
    },
    Source {
        market: Market::Nyse,
        file: "nysmst.cod.zip",
        kind: FileKind::OverseasCod,
    },
    Source {
        market: Market::Amex,
        file: "amsmst.cod.zip",
        kind: FileKind::OverseasCod,
    },
    Source {
        market: Market::FoIdx,
        file: "fo_idx_code.mst.zip",
        kind: FileKind::FoMst,
    },
    Source {
        market: Market::FoStk,
        file: "fo_stk_code.mst.zip",
        kind: FileKind::FoMst,
    },
];

pub struct SyncReport {
    pub results: Vec<MarketResult>,
}

pub struct MarketResult {
    pub market: Market,
    pub count: usize,
    pub error: Option<String>,
}

pub async fn sync_all(db_path: &Path, if_stale: bool) -> Result<SyncReport> {
    let mut store = Store::open(db_path)?;

    if if_stale && !is_stale(&store)? {
        println!("최근 동기화 후 24시간 이내 — skip.");
        return Ok(SyncReport { results: vec![] });
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    let mut results = Vec::new();
    for src in SOURCES {
        print!("  {} 다운로드 중... ", src.market.as_str());
        use std::io::Write;
        std::io::stdout().flush().ok();
        match sync_one(&client, &mut store, src).await {
            Ok(n) => {
                println!("✓ {} 건", n);
                results.push(MarketResult {
                    market: src.market,
                    count: n,
                    error: None,
                });
            }
            Err(e) => {
                println!("✗ {}", e);
                results.push(MarketResult {
                    market: src.market,
                    count: 0,
                    error: Some(e.to_string()),
                });
            }
        }
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    store.set_meta(META_LAST_SYNC, &now.to_string())?;

    Ok(SyncReport { results })
}

async fn sync_one(
    client: &reqwest::Client,
    store: &mut Store,
    src: &Source,
) -> Result<usize> {
    let url = format!("{}/{}", BASE_URL, src.file);
    let resp = client.get(&url).send().await?;
    if !resp.status().is_success() {
        return Err(anyhow!("HTTP {}: {}", resp.status(), url));
    }
    let zip_bytes = resp.bytes().await?;

    let body = extract_single_file(&zip_bytes)
        .with_context(|| format!("zip 해제 실패: {}", src.file))?;

    let symbols = match src.kind {
        FileKind::DomesticMst => master::parse_domestic_mst(&body, src.market)?,
        FileKind::OverseasCod => master::parse_overseas_cod(&body, src.market)?,
        FileKind::FoMst => master::parse_fo_mst(&body, src.market)?,
    };

    store.replace_market(src.market, &symbols)?;
    Ok(symbols.len())
}

/// zip 안의 첫 번째 파일을 읽어 바이트 반환.
fn extract_single_file(zip_bytes: &[u8]) -> Result<Vec<u8>> {
    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor)?;
    if archive.is_empty() {
        return Err(anyhow!("빈 zip"));
    }
    let mut file = archive.by_index(0)?;
    let mut buf = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn is_stale(store: &Store) -> Result<bool> {
    let Some(s) = store.get_meta(META_LAST_SYNC)? else {
        return Ok(true);
    };
    let last: u64 = s.parse().unwrap_or(0);
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    Ok(now.saturating_sub(last) > STALE_AFTER_SECS)
}
