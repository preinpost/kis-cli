# Ralph Loop 진행 상황

`python3 scripts/progress.py`로 자동 생성/업데이트. (아직 없음 — 필요 시 작성.)

수동 확인:
```bash
jq 'group_by(.category) | map({cat: .[0].category, total: length, done: map(select(.status=="done")) | length, blocked: map(select(.status=="blocked")) | length})' tasks/api-registry.json
```
