# KIS API 파일락 기반 TPS 제한기 도입

## 배경
- 여러 `kis` 프로세스를 병렬로 띄우면 KIS 오픈API의 초당 호출수 한도(실전 20 TPS / 모의 2 TPS)를 쉽게 넘겨 `EGW00201` 500 에러가 난다.
- 프로세스 간 조율이 필요하므로 파일락(flock) + 디스크 공유 타임스탬프 버킷으로 sliding-window rate limiter를 만든다.

## 설계 요약
- **상태 파일**: `~/.kis-cli/rate_limit_{prod|mock}.json` — 최근 1초 내 전송 타임스탬프(ms) 배열
- **잠금**: `fs2::FileExt::lock_exclusive()`로 advisory exclusive flock
- **예산(budget)**: prod 18 TPS, mock 2 TPS (헤드룸 확보)
- **알고리즘** (sliding window):
  1. 상태 파일 open + flock
  2. `recent` 중 `now - 1000ms` 이전 것 제거
  3. `recent.len() < budget` 이면 now 추가하고 write → flock 해제 → 반환
  4. 아니면 `oldest + 1000 - now` 만큼 sleep할 시간 계산, flock 해제 후 `std::thread::sleep`, 루프 재시도
- **비동기 통합**: `tokio::task::spawn_blocking`으로 감싸서 `acquire(is_mock: bool) -> Result<()>` async 함수 제공
- **훅 포인트**:
  - `KisClient::get` 시작부
  - `KisClient::send_json` 시작부 (POST/DELETE 등 모두 커버)
  - oauth 헬퍼들 (`tokenp.rs`, `approval.rs`, `revokep.rs`, `hashkey.rs`)의 HTTP 호출 직전

## 작업 항목
- [ ] `Cargo.toml`에 `fs2 = "0.4"` 추가
- [ ] `src/rate_limit.rs` 새 모듈 작성
  - `pub async fn acquire(is_mock: bool) -> Result<()>`
  - blocking 내부 구현, `spawn_blocking`으로 비동기 래핑
  - 상태 파일 경로: `dirs::home_dir() + .kis-cli/rate_limit_{prod,mock}.json`
  - budget 상수: prod 18, mock 2
  - 윈도우 상수: 1000ms
- [ ] `src/main.rs`에 `mod rate_limit;` 등록
- [ ] `src/client.rs` 수정
  - `get` 함수 첫 줄에 `crate::rate_limit::acquire(self.is_mock).await?;`
  - `send_json` 함수 첫 줄에 동일 코드 (post_json은 send_json 경유라 자동 적용)
- [ ] `src/api/oauth/tokenp.rs`, `approval.rs`, `revokep.rs`, `hashkey.rs`에 각 `call` 시작부에 `crate::rate_limit::acquire(is_mock).await?;`
- [ ] `cargo check`로 빌드 검증
- [ ] 동작 검증: 터미널 두 개로 `kis backtest` 병렬 호출 → EGW00201 발생 없는지 확인

## 비고
- 기존 `backtest.rs`의 250ms 인터-청크 딜레이는 유지 (의도된 정책상 지연). 글로벌 리미터는 중복이 아니라 안전망.
- EGW00201 자동 재시도(옵션 2)는 이번 변경에는 포함하지 않음. 필요 시 후속 작업.
