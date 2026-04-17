use anyhow::Result;

use crate::config;
use crate::symbols::{Store, sync};

pub async fn run_sync(if_stale: bool) -> Result<()> {
    let path = config::symbols_db_path()?;
    println!("symbols DB: {}", path.display());
    let report = sync::sync_all(&path, if_stale).await?;
    if report.results.is_empty() {
        return Ok(());
    }
    println!("\n요약:");
    for r in &report.results {
        match &r.error {
            None => println!("  {:<7} {} 건", r.market.as_str(), r.count),
            Some(e) => println!("  {:<7} 실패 — {}", r.market.as_str(), e),
        }
    }
    Ok(())
}

pub fn run_find(query: &str, limit: usize) -> Result<()> {
    let path = config::symbols_db_path()?;
    let store = Store::open(&path)?;
    let results = store.search(query, limit)?;
    if results.is_empty() {
        println!("일치 없음. `kis symbols sync`로 마스터를 먼저 받으세요.");
        return Ok(());
    }
    println!("{:<10} {:<7} {:<30} {}", "코드", "시장", "한글명", "영문명");
    println!("{}", "─".repeat(80));
    for s in results {
        println!(
            "{:<10} {:<7} {:<30} {}",
            s.code,
            s.market.as_str(),
            s.name_kr,
            s.name_en
        );
    }
    Ok(())
}
