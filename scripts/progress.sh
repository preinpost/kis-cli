#!/bin/bash
# Ralph Loop 진행 상황 요약
set -euo pipefail
cd "$(dirname "$0")/.."

echo "# kis-cli Ralph Loop 진행 상황"
echo
echo "생성 시각: $(date '+%Y-%m-%d %H:%M:%S')"
echo

echo "| 카테고리 | 전체 | 완료 | 블록 | 남음 |"
echo "|---|---:|---:|---:|---:|"
jq -r '
  group_by(.category)[]
  | {cat: .[0].category,
     total: length,
     done: (map(select(.status=="done")) | length),
     blocked: (map(select(.status=="blocked")) | length),
     pending: (map(select(.status=="pending")) | length)}
  | "| \(.cat) | \(.total) | \(.done) | \(.blocked) | \(.pending) |"
' tasks/api-registry.json

echo
echo "## 전체"
jq -r '
  {total: length,
   done: (map(select(.status=="done")) | length),
   blocked: (map(select(.status=="blocked")) | length),
   pending: (map(select(.status=="pending")) | length)}
  | "- 전체: \(.total)\n- 완료: \(.done)\n- 블록: \(.blocked)\n- 남음: \(.pending)"
' tasks/api-registry.json
