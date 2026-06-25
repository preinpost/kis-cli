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

## Phase B (별도 — 사용자 복귀 후 같이)
**kis-daemon 슈퍼바이저**: stop-loss/signal-watch/daytrade/telegram 4개 데몬을
단일 프로세스(config 1·로그 파이프 1·라이프사이클 1)로 통합.
`daytrade/daemon.rs` 의 `notify` config 핫리로드 패턴을 표준으로 승격.
→ **실주문/돈 만지는 데몬의 런타임·CLI 표면을 바꾸는 설계 변경 + 라이브 검증 필요**,
   사용자 확인 후 진행. (CLI subcommand 유지 vs `kisd` 분리 = 제품 결정)

## Review
(작업 후 기록)
