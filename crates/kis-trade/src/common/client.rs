//! KisClient 빌더 — config.toml 의 자격증명·모의투자 플래그로 클라이언트 구성.
//! CLI(kis)·데몬 바이너리(kisd-*)가 공유한다.

use anyhow::Result;

use kis_core::client::KisClient;
use kis_core::config;

/// `config.toml` 을 로드해 KisClient 생성. 모의투자 여부(is_mock)도 반영.
pub fn build_client() -> Result<KisClient> {
    let cfg = config::load_config()?;
    Ok(KisClient::with_mock(cfg.credentials, cfg.is_mock))
}
