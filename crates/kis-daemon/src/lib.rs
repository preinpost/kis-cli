//! kis-daemon — 데몬 공통 인프라.
//!
//! 4개 데몬(stop-loss / signal-watch / daytrade daemon / telegram stream)이 공유하는
//! 스캐폴딩을 모은 leaf crate. **데몬을 한 프로세스로 합치지 않는다** — 각 데몬은 여전히
//! 자기 subcommand·컨테이너로 돌고, 여기서는 라이브러리 코드만 가져다 쓴다.
//!
//! - `logging`      : tracing subscriber 초기화 (파일+stderr / foreground stderr-only)
//! - `shutdown`     : SIGTERM/Ctrl-C → CancellationToken 취소 (그레이스풀 종료)
//! - `config_watch` : notify 기반 설정 파일 핫리로드 감시
//!
//! KisClient/SDK 에 의존하지 않는 generic 인프라다(다른 leaf 크레이트와 동일).
pub mod logging;
