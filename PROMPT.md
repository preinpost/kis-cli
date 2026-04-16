# kis-cli Ralph Loop — 외부 실행용 프롬프트

> 권장 실행 경로: Claude Code에서 `/loop /kis-ralph` (interval 없이).
> 아래는 Huntley 원본 방식(터미널 외부 while 루프)으로 돌리고 싶을 때의 백업 프롬프트다.

---

AGENT.md와 tasks/lessons.md, .agent/conventions.md, .agent/rust-patterns.md를 먼저 읽어라.

tasks/api-registry.json에서 첫 번째 `status=="pending"` 항목을 고르고, 그 항목 1개만 구현하라. 완료 조건은 `cargo check` 통과 + `git commit` + registry status=done. 3회 실패 시 status=blocked + lessons.md 업데이트.

규칙: iteration당 정확히 1개. `src/api/` 안에서만 작업. CLI / main.rs / commands 건드리지 말 것. 실제 API 호출 금지 — `cargo check`만. 스펙에 있는 모든 응답 필드를 Response struct에 포함.

---

외부 while 루프 사용 예:
```bash
while :; do
  cat PROMPT.md | claude -p --dangerously-skip-permissions
  # pending 수가 0이면 종료
  pending=$(jq '[.[] | select(.status=="pending")] | length' tasks/api-registry.json)
  [ "$pending" -eq 0 ] && break
done
```
