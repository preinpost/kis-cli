//! kis-data — 종목 마스터 데이터(symbols) 로컬 저장/검색/동기화.
//!
//! 마스터 파일 다운로드·파싱·SQLite 저장을 담당하는 leaf crate.
//! CLI 와 데몬이 공용으로 사용한다.
pub mod symbols;
