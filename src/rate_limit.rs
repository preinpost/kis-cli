//! 파일락 기반 프로세스간 KIS API TPS 제한기.
//!
//! 여러 `kis` 프로세스가 동시에 떠도 KIS 오픈API의 초당 호출수 한도(실전 20 / 모의 2)를
//! 넘지 않도록, `~/.kis-cli/rate_limit_{prod,mock}.json`에 최근 1초 내 전송 타임스탬프(ms)를
//! 기록하고 `fs2` advisory flock으로 상호배제한다. 슬라이딩 윈도우 방식.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

const WINDOW_MS: u64 = 1000;
const BUDGET_PROD: usize = 18;
const BUDGET_MOCK: usize = 2;

#[derive(Default, Serialize, Deserialize)]
struct Bucket {
    recent: Vec<u64>,
}

fn state_path(is_mock: bool) -> Result<PathBuf> {
    let mut dir = dirs::home_dir().context("홈 디렉토리 조회 실패")?;
    dir.push(".kis-cli");
    std::fs::create_dir_all(&dir).context("~/.kis-cli 생성 실패")?;
    let name = if is_mock {
        "rate_limit_mock.json"
    } else {
        "rate_limit_prod.json"
    };
    dir.push(name);
    Ok(dir)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn acquire_blocking(path: PathBuf, budget: usize) -> Result<()> {
    loop {
        let mut file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .context("rate_limit 상태 파일 open 실패")?;
        file.lock_exclusive().context("flock 획득 실패")?;

        let mut buf = String::new();
        file.read_to_string(&mut buf).ok();
        let mut bucket: Bucket = if buf.trim().is_empty() {
            Bucket::default()
        } else {
            serde_json::from_str(&buf).unwrap_or_default()
        };

        let now = now_ms();
        bucket.recent.retain(|t| t + WINDOW_MS > now);

        if bucket.recent.len() < budget {
            bucket.recent.push(now);
            bucket.recent.sort_unstable();
            let out = serde_json::to_string(&bucket)?;
            file.set_len(0)?;
            file.seek(SeekFrom::Start(0))?;
            file.write_all(out.as_bytes())?;
            FileExt::unlock(&file).ok();
            return Ok(());
        }

        let oldest = *bucket.recent.first().expect("non-empty when at budget");
        let wait_ms = (oldest + WINDOW_MS).saturating_sub(now).max(10);
        FileExt::unlock(&file).ok();
        drop(file);
        std::thread::sleep(Duration::from_millis(wait_ms));
    }
}

/// KIS API 호출 1건 전송 직전에 호출해 TPS 한도 내에서 슬롯을 확보한다.
pub async fn acquire(is_mock: bool) -> Result<()> {
    let path = state_path(is_mock)?;
    let budget = if is_mock { BUDGET_MOCK } else { BUDGET_PROD };
    tokio::task::spawn_blocking(move || acquire_blocking(path, budget))
        .await
        .map_err(|e| anyhow::anyhow!("rate_limit join 실패: {e}"))?
}
