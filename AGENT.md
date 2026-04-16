# kis-cli — Ralph Loop Agent Guide

이 파일은 **매 Ralph Loop iteration 시작 시 먼저 읽어야 한다.** `tasks/lessons.md`도 함께 읽는다.

## 목표

한국투자증권 Open API 338개를 `src/api/` 하위에 타입-안전한 Rust 모듈로 구현한다. Codegen 없이 **수동** 작성. 실제 API 호출은 하지 않고 `cargo check`로만 검증.

## 구조

- **단일 source of truth**: `tasks/api-registry.json` — 338개 API 리스트. 각 항목은 `{category, name, endpoint, slug, module_path, file_path, spec_path, status}`.
- **스펙**: `.agent/specs/<slug>.md` — 포털에서 미리 추출한 요청/응답 파라미터 테이블.
- **코드**: `src/api/<category>/<subcategory>/<leaf>.rs` — endpoint 하나 = 파일 하나.

## iteration 흐름 (엄수)

1. `tasks/lessons.md` 읽기 (이전 실수 패턴 학습)
2. `tasks/api-registry.json`에서 **처음 만나는 `status=="pending"`** 항목 1개 선택
3. 해당 `spec_path`의 스펙 마크다운 읽기
4. `file_path`의 리프 `.rs` 파일 작성 — `.agent/conventions.md`의 템플릿 사용
5. 부모 `mod.rs`에 `pub mod <leaf>;` 한 줄 추가
6. `cargo check` 실행. 실패 시 최대 3회까지 수정 시도
7. 성공 시:
   - registry에서 해당 항목의 `status`를 `"done"`으로 변경
   - `git add -A` + `git commit -m "add: [<카테고리>] <API명> 구현"` (한글, user CLAUDE.md 규칙)
   - iteration 종료
8. 3회 실패 시:
   - `status`를 `"blocked"`로 변경
   - `tasks/lessons.md`에 실패 원인 + 다음 iteration이 피해야 할 규칙 추가
   - 커밋 없이 iteration 종료

## 네이밍 규칙

| 요소 | 규칙 | 예시 |
|---|---|---|
| 모듈 파일 | snake_case. endpoint leaf에서 `-` → `_`, 숫자 접두사 `v\d+` 제거 | `/uapi/domestic-stock/v1/quotations/inquire-price` → `inquire_price.rs` |
| 실시간 (WebSocket) | TR_ID 소문자 | `/tryitout/H0STCNT0` → `h0stcnt0.rs` |
| 예약어 충돌 | 언더스코어 suffix | `type` → `type_` |
| struct 이름 | `Request`, `Response` (API별 고유 이름 X, 모듈 단위로 충분) | - |
| TR_ID 상수 | `pub const TR_ID: &str = "...";` | - |

## 금지사항

- **codegen / 매크로** — 수동 작성만.
- **새 CLI 서브커맨드 추가** — 사용자가 명시적으로 요청하지 않는 한 `src/main.rs`, `src/commands/` 건드리지 말 것. `src/api/` 안에서만 작업.
- **실제 API 호출 / 토큰 발급** — `cargo check`만. `cargo run` 금지.
- **기존 `status=done` 항목 수정** — 이미 구현된 API는 읽기 전용.
- **`registry.json` 외 전역 인덱스 수정** — 단일 소스가 흔들리면 안 됨.
- **여러 API 한꺼번에 구현** — iteration당 정확히 1개.

## 공통 가정

- HTTP 클라이언트: `crate::client::KisClient`.
  - `client.get(endpoint, tr_id, &[(key, val), ...])` — GET (쿼리 대문자 자동 변환).
  - `client.post_json(endpoint, tr_id, &body, &[extra_headers])` — POST (JSON body).
  - `client.send_json(Method, ...)` — DELETE/PUT 등.
  - 실전/모의 도메인은 `client.base_url()`로 자동 분기.
- 응답 래퍼: `crate::models::ApiResponse` (`rt_cd`, `msg_cd`, `msg1`, `output`, `output1`, `output2`).
- 응답 필드는 serde로 역직렬화. 상세는 `.agent/rust-patterns.md` 참조.
- WebSocket(실시간)은 REST와 전혀 다름. 해당 카테고리 진입 시 `tasks/lessons.md` 확인 후 신중히.

## 커밋 메시지 규칙 (user CLAUDE.md)

한글, 접두어 필수:
- `add: [카테고리] API명 구현` — 신규 API 구현
- `fix: [카테고리] API명 ...` — 기존 API 버그 수정
- `update: ...` — 인프라 수정

## 종료 조건

`tasks/api-registry.json`의 모든 항목이 `status=="done"` 또는 `"blocked"`이면 Ralph Loop는 스스로 중단한다.
