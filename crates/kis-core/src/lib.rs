//! kis-core — 한국투자증권 Open API 헤드리스 SDK.
//!
//! REST/WebSocket 클라이언트, 토큰·레이트리밋 관리, 타입-안전 endpoint 바인딩을 제공한다.
//! CLI/데몬/뷰어 등 표현 계층에 의존하지 않는 순수 라이브러리(leaf crate).
pub mod api;
pub mod client;
pub mod config;
pub mod models;
pub mod rate_limit;
pub mod token;
pub mod ws;
