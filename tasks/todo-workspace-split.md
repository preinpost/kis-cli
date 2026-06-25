# 워크스페이스 분리 (멀티 크레이트) — Phase A

단일 바이너리(`kis-cli`, 50K LOC)를 카고 워크스페이스 4크레이트로 분리.
**멀티레포 아님 — 같은 repo의 워크스페이스.** 동작 변화 0 (순수 구조 리팩터).

## 목표 구조
```
kis/ (워크스페이스 루트)
├── Cargo.toml            # [workspace] + [workspace.dependencies]
├── Cargo.lock
├── skill/                # 루트 유지 (skill.rs 가 include_str! 로 임베드)
└── crates/
    ├── kis-core/    (lib) api/ + client/token/models/config/rate_limit/ws  ← 헤드리스 SDK, leaf
    ├── kis-data/    (lib) symbols/                                         ← leaf
    ├── kis-analysis/(lib) analysis/                                        ← leaf (crate 의존 0)
    └── kis-cli/     (bin `kis`) main·commands·viewer·logging·error          ← core·data·analysis 의존
```

## 핵심 전략 — 재수출(re-export)
`pub(crate)`/`pub(super)` 가 core/data/analysis 에 **0개** → 전부 plain `pub`.
kis-cli `main.rs` 에서 `pub use kis_core::{api,client,...}` 등으로 **기존 `crate::` 경로 재수출**
→ commands/viewer 의 cross-ref(`crate::api`, `crate::symbols` …) **무수정 이전**.

검증된 `crate::` 1단계 경로 매핑:
- crate::commands / crate::logging / crate::error / crate::viewer → kis-cli 잔류
- crate::api / crate::client / crate::config / crate::models / crate::token / crate::rate_limit / crate::ws → `kis_core` 재수출
- crate::symbols → `kis_data` 재수출
- crate::analysis → `kis_analysis` 재수출

## 체크리스트
- [x] 브랜치 `refactor/workspace-split`
- [ ] `crates/{kis-core,kis-data,kis-analysis,kis-cli}/src` 생성
- [ ] `git mv` 로 파일 이전 (히스토리 보존)
- [ ] 각 lib `lib.rs` 작성 (`pub mod ...`)
- [ ] 루트 `Cargo.toml` → `[workspace]` + `[workspace.dependencies]`
- [ ] 멤버 4개 `Cargo.toml` 작성
- [ ] `kis-cli/src/main.rs`: 이전된 `mod` 제거 + 재수출 추가
- [ ] `skill.rs` include_str! 경로 `../../` → `../../../../skill/SKILL.md`
- [ ] Dockerfile: `COPY src`→`COPY crates`, `cargo build`→`cargo build -p kis-cli ...`
- [ ] `scripts/build-registry.py`: file_path 프리픽스 `src/`→`crates/kis-core/src/`
- [ ] `cargo build` (default + `--no-default-features`) 통과
- [ ] `cargo test` 통과
- [ ] 커밋

## Phase B (완료) — kis-daemon = lib (단일 슈퍼바이저 아님)

**결정:** 데몬 배포가 Docker per-container(compose 가 데몬별 컨테이너로 이미 슈퍼바이징)라
단일 `kisd` 프로세스는 만들지 않음. `kis-daemon` 은 **bin 이 아니라 lib** — 4개 데몬이
복붙해둔 스캐폴딩(logging/shutdown/config_watch)만 공유. 각 데몬 subcommand·컨테이너 유지.

커밋:
- `fbc1eab` kis-daemon 크레이트 신설 + logging 이전 (재수출로 호출부 무수정)
- `9111ba7` shutdown(spawn_signal_listener·wait_for_shutdown) + config_watch(spawn_watcher)
  추출, daytrade/telegram dedup (바이트 동일 복붙 제거)
- `c6244f4` signal-watch 종료 통일
- `3527046` **stop-loss 그레이스풀 종료 추가** (유일한 동작 변경) — 라이브 스모크로
  검증: DRY-RUN 띄우고 SIGTERM → exit 0 + "종료 신호 수신" 로그 (이전엔 SIGKILL 당함)

검증: cargo build(default+headless) · cargo test 16/16 · stop-loss SIGTERM 라이브 스모크.

### Phase B 중 발견한 기존 버그 (Phase B 무관, 미수정)
`docker-compose.yml:44` 의 `command: [stop-loss, run, --threshold, -5]` 는 clap 이 `-5` 를
플래그로 오인 → "unexpected argument '-5'" 로 **컨테이너 크래시루프**. profile-gated 라
기본 비활성이지만 stop-loss 프로필 켜면 자동손절이 안 뜸. 수정 옵션: compose `--threshold=-5`
(threshold 기본값이 이미 -5라 플래그 생략도 가능) 또는 clap arg 에 `allow_hyphen_values`.

### Phase B2 (선택, 보류)
daytrade `RunningTask`/`apply_diff` drain-join 을 generic `TaskSupervisor` 로 일반화
(+telegram spawn task explicit join). 더 침습적이라 별도 진행.

## Review (완료 — Phase A)
- 4크레이트 분리 완료. `git mv` 415 rename (히스토리 보존).
- 재수출 전략 적중: commands/viewer cross-ref **0줄 수정**.
- 검증: leaf 독립 컴파일 ✓ · kis-cli(chart) ✓ · headless(`--no-default-features`) ✓ ·
  `cargo test --workspace` 16/16 ✓ · `kis --help` ✓.
- 경로 의존 보정: Dockerfile / skill.rs include_str! / build-registry.py.
- CI(release/bump)·docker-compose 무영향 확인 (bump 의 `^version` sed 는 workspace.package 명중).
- 커밋: `7b48bc5` refactor(분리) + `90bc784` fix(is_ticker_like 선행버그).

### 발견한 선행 버그 (분리와 무관, 별도 커밋)
`is_ticker_like("Tesla")==true` → 소문자 회사명을 티커로 오인. main 에도 동일 존재.
CI 가 `cargo test` 미실행이라 미발견. all() 조건 대문자로 좁혀 수정, 테스트 통과.

### 남은 것
- **Phase B (kis-daemon 슈퍼바이저)** — 사용자 복귀 후 설계 확정 필요 (아래 참조).
- (선택) `tasks/api-registry.json` 의 file_path 가 옛 `src/api/...` (pending=0 이라 무해, 재생성 시 자동 보정).
- (선택) 선행 dead_code 경고들(`KisError`, `launch_static`, `split_kr_en`, `time_until_open` 등) 정리.
