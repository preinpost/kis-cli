---
description: kis-cli Ralph Loop — pending API 1개를 Rust로 구현 (한 iteration)
---

너는 지금 한국투자증권 Open API 338개를 Rust로 구현하는 장기 작업의 **한 iteration**을 수행한다.

## 시작 시 필독 (순서대로)

1. `AGENT.md` — 아키텍처와 규칙
2. `tasks/lessons.md` — 과거 실수 패턴 (특히 하단의 누적 교훈)
3. `.agent/conventions.md` — Rust 파일 템플릿
4. `.agent/rust-patterns.md` — KisClient 사용법, serde 관례, 실시간 특이사항

## 이번 iteration에서 할 일 (정확히 1개 API)

1. `tasks/api-registry.json`을 읽는다.
2. **첫 번째** `status == "pending"` 항목을 고른다 (배열 순서 그대로).
3. 그 항목의 `spec_path`(`.agent/specs/<slug>.md`)를 읽는다.
   - 스펙 파일이 없거나 비어있으면 `status`를 `"blocked"`로 바꾸고 `tasks/lessons.md`에 기록한 뒤 iteration을 종료한다.
4. 그 항목의 `file_path`에 Rust 리프 모듈을 작성한다.
   - `.agent/conventions.md`의 템플릿을 기반으로.
   - 스펙의 **모든 응답 필드**를 Response struct에 포함 (누락 금지).
   - 요청 파라미터 중 `Required=Y`만 Request struct 필드로 — `Required=N`은 주석으로만 언급하거나 `Option<String>`으로.
5. 부모 `mod.rs`에 `pub mod <leaf>;` 한 줄을 알파벳 순서대로 삽입한다.
6. `cargo check` 실행.
   - 통과 → 7단계로.
   - 실패 → 에러 메시지 읽고 파일 수정. **최대 3회 재시도**.
   - 3회 실패 시: registry `status="blocked"`, `tasks/lessons.md`에 원인과 다음 iteration이 피할 규칙 추가. 커밋 없이 종료.
7. `tasks/api-registry.json`에서 해당 항목의 `status`를 `"done"`으로 변경.
8. `git add -A && git commit -m "add: [<카테고리>] <API명> 구현"` — 한글, 접두어 `add:` 필수 (user CLAUDE.md 규칙).
9. 결과를 한 줄로 보고하고 iteration 종료.

## 절대 금지

- **iteration당 2개 이상 API 구현 금지** (다음 loop이 처리).
- **기존 `status=="done"` 항목 건드리지 않기**.
- **`src/main.rs`, `src/commands/*` 건드리지 않기** — 사용자가 따로 지시할 때만.
- **실제 API 호출 / `cargo run` / 토큰 발급 금지**. `cargo check`만.
- **codegen, 매크로로 여러 파일 생성 금지**. 수동 작성만.
- **스펙에 없는 필드 임의 추가 금지** — 스펙을 충실히 반영.

## 종료 조건

`status=="pending"`인 항목이 하나도 없으면:
- "✅ 모든 API 구현 완료 (pending=0). Ralph Loop 종료." 를 보고하고 iteration을 종료한다. `/loop`은 자동으로 중단된다.
