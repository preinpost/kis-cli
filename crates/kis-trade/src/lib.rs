//! kis-trade — 컨테이너 데몬의 엔진 라이브러리.
//!
//! telegram 실시간 스트림 / 자동 손절(stop-loss) / 데이트레이드(daytrade) 데몬의
//! **상태를 가진 엔진**을 모은다. kis-cli(인터랙티브 CLI)와 얇은 데몬 바이너리
//! (kisd-telegram / kisd-stop-loss / kisd-daytrade)가 공유한다.
//!
//! 설계 계약: 모든 엔진 `run*()` 은 `cancel: CancellationToken` 을 주입받고
//! 내부에서 로깅/시그널 리스너를 초기화하지 않는다 — 그 배선은 호출자(데몬 main 또는
//! kis-cli wrapper)의 책임이다. KisClient/SDK 는 kis-core, 종목 마스터는 kis-data,
//! 지표·시그널 계산은 kis-analysis, 데몬 공통 인프라는 kis-daemon 에 의존한다.

pub mod common;
pub mod daytrade;
pub mod stop_loss;
pub mod telegram;
