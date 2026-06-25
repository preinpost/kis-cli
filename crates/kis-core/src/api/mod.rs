//! 한국투자증권 Open API 타입-안전 Rust 바인딩.
//!
//! 각 리프 모듈은 1개 endpoint에 대응하며 다음을 제공:
//! - `TR_ID` 상수
//! - `Request` struct (요청 파라미터/바디)
//! - `Response` struct (응답 바디)
//! - `call(client, req)` async 함수
//!
//! 실시간(WebSocket) API는 REST와 달라 별도 규약을 따른다 — 해당 모듈 주석 참조.

pub mod bond;
pub mod domestic_stock;
pub mod futureoption_domestic;
pub mod futureoption_overseas;
pub mod oauth;
pub mod overseas_stock;
