#!/usr/bin/env python3
"""
한국투자증권 Open API 포털에서 338개 API 스펙을 추출하여
.agent/specs/<slug>.md로 저장한다.

사전조건:
  - agent-browser (네이티브 모드) 이미 https://apiportal.koreainvestment.com/apiservice-apiservice
    을 열어둔 상태.
  - 이미 존재하는 spec 파일은 건너뜀 (재시작 가능).

사용법:
  python3 scripts/scrape-specs.py              # 전체
  python3 scripts/scrape-specs.py --limit 5    # 앞 5개만 (테스트)
  python3 scripts/scrape-specs.py --slug foo   # 특정 slug만 재추출
"""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
REGISTRY = ROOT / "tasks" / "api-registry.json"
SPEC_DIR = ROOT / ".agent" / "specs"
EXTRACT_JS = ROOT / "scripts" / "extract-spec.js"


def run_browser(args: list[str], *, stdin: str | None = None, timeout: int = 30) -> str:
    env = os.environ.copy()
    env["AGENT_BROWSER_NATIVE"] = "1"
    r = subprocess.run(
        ["agent-browser", *args],
        input=stdin,
        capture_output=True,
        text=True,
        env=env,
        timeout=timeout,
    )
    if r.returncode != 0:
        raise RuntimeError(f"agent-browser {args} failed: {r.stderr.strip()}")
    return r.stdout


def navigate(endpoint: str) -> None:
    # Use goLeftMenuUrl to trigger the SPA navigation
    js = f"goLeftMenuUrl('{endpoint}')"
    run_browser(["eval", js])
    run_browser(["wait", "--load", "networkidle"], timeout=45)


def extract_spec() -> str:
    extract_src = EXTRACT_JS.read_text(encoding="utf-8")
    raw = run_browser(["eval", "--stdin"], stdin=extract_src, timeout=30)
    raw = raw.strip()
    if raw.startswith('"') and raw.endswith('"'):
        return json.loads(raw)
    return raw


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--slug", type=str, default=None)
    ap.add_argument("--force", action="store_true", help="덮어쓰기")
    ap.add_argument("--start", type=int, default=0, help="registry index 시작")
    args = ap.parse_args()

    entries = json.loads(REGISTRY.read_text(encoding="utf-8"))
    if args.slug:
        entries = [e for e in entries if e["slug"] == args.slug]
        if not entries:
            print(f"no such slug: {args.slug}", file=sys.stderr)
            return 1
    if args.start:
        entries = entries[args.start:]
    if args.limit:
        entries = entries[: args.limit]

    SPEC_DIR.mkdir(parents=True, exist_ok=True)

    ok = skip = fail = 0
    total = len(entries)
    t0 = time.time()
    for i, e in enumerate(entries, 1):
        slug = e["slug"]
        out_path = SPEC_DIR / f"{slug}.md"
        if out_path.exists() and not args.force:
            skip += 1
            continue

        endpoint = e["endpoint"]
        try:
            navigate(endpoint)
            # Allow the SPA render the detail; networkidle is often quick here
            time.sleep(0.6)
            spec = extract_spec()
            if not spec or spec.startswith("<!-- NO_TABLES"):
                raise RuntimeError("empty or no tables")
            header = (
                f"<!-- endpoint: {endpoint} -->\n"
                f"<!-- category: {e['category']} -->\n"
                f"<!-- korean_name: {e['name']} -->\n\n"
            )
            out_path.write_text(header + spec.rstrip() + "\n", encoding="utf-8")
            ok += 1
        except Exception as ex:  # noqa: BLE001
            fail += 1
            (SPEC_DIR / f"{slug}.FAILED.md").write_text(
                f"endpoint: {endpoint}\nerror: {ex}\n", encoding="utf-8"
            )

        if i % 10 == 0 or i == total:
            elapsed = time.time() - t0
            rate = i / elapsed if elapsed else 0
            eta = (total - i) / rate if rate else 0
            print(
                f"[{i}/{total}] ok={ok} skip={skip} fail={fail} "
                f"rate={rate:.2f}/s eta={eta:.0f}s",
                flush=True,
            )

    print(f"\nDONE: ok={ok} skip={skip} fail={fail}")
    return 0 if fail == 0 else 2


if __name__ == "__main__":
    sys.exit(main())
