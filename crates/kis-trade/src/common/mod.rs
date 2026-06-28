//! 데몬 엔진이 공유하는 공통 인프라.
//!
//! - [`client`]  : config.toml → KisClient 빌더
//! - [`session`] : 시장별 장 시간/공휴일/마감 임박 판정 (telegram·daytrade 공용)
//! - [`period`]  : 데이트레이드 봉 주기 enum
//! - [`resolve`] : 종목 해석·표시 헬퍼
//! - [`notify`]  : 텔레그램 Bot API HTTP 헬퍼

pub mod client;
pub mod notify;
pub mod period;
pub mod resolve;
pub mod session;
