//! 종목 마스터 데이터: 마스터 파일 다운로드 → 파싱 → SQLite(FTS5) 저장 → 이름/코드 검색.

pub mod lookup;
pub mod master;
pub mod store;
pub mod sync;

pub use lookup::{resolve, ResolveMode, ResolvedSymbol};
pub use store::{Market, Store};
