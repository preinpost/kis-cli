//! 시황 브리프 스트림 엔진은 kis-trade::telegram 로 이동. 기존 경로 유지를 위한 재노출 shim.
//! (`run` 은 이제 cancel 토큰을 주입받는다 — 호출처 main.rs 가 로깅·시그널을 배선한다.)
pub use kis_trade::telegram::*;
