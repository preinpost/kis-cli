#!/usr/bin/env python3
"""
Turn the raw {category, name, endpoint} list scraped from apiportal into the
structured api-registry.json with slug + module_path + file_path + status.
"""
import json
import re
import sys
from pathlib import Path

CATEGORY_TO_MODULE = {
    "OAuth인증": "oauth",
    "[국내주식] 주문/계좌": "domestic_stock::order_account",
    "[국내주식] 기본시세": "domestic_stock::quotations",
    "[국내주식] ELW 시세": "domestic_stock::elw",
    "[국내주식] 업종/기타": "domestic_stock::sector",
    "[국내주식] 종목정보": "domestic_stock::symbol_info",
    "[국내주식] 시세분석": "domestic_stock::market_analysis",
    "[국내주식] 순위분석": "domestic_stock::ranking",
    "[국내주식] 실시간시세": "domestic_stock::realtime",
    "[국내선물옵션] 주문/계좌": "futureoption_domestic::order_account",
    "[국내선물옵션] 기본시세": "futureoption_domestic::quotations",
    "[국내선물옵션] 실시간시세": "futureoption_domestic::realtime",
    "[해외주식] 주문/계좌": "overseas_stock::order_account",
    "[해외주식] 기본시세": "overseas_stock::quotations",
    "[해외주식] 시세분석": "overseas_stock::market_analysis",
    "[해외주식] 실시간시세": "overseas_stock::realtime",
    "[해외선물옵션] 주문/계좌": "futureoption_overseas::order_account",
    "[해외선물옵션] 기본시세": "futureoption_overseas::quotations",
    "[해외선물옵션]실시간시세": "futureoption_overseas::realtime",
    "[장내채권] 주문/계좌": "bond::order_account",
    "[장내채권] 기본시세": "bond::quotations",
    "[장내채권] 실시간시세": "bond::realtime",
}


def endpoint_to_leaf(endpoint: str, category: str) -> str:
    """Derive the leaf module name from the endpoint path."""
    # WebSocket realtime: /tryitout/H0STCNT0 -> h0stcnt0
    if endpoint.startswith("/tryitout/"):
        return endpoint.split("/")[-1].lower()
    parts = [p for p in endpoint.strip("/").split("/") if p]
    # Strip 'uapi', numeric version 'v1', 'oauth2'
    filtered = []
    for p in parts:
        if p in ("uapi", "oauth2"):
            continue
        if re.fullmatch(r"v\d+", p):
            continue
        filtered.append(p)
    # The last path segment is the operation; convert dashes to underscores
    leaf = filtered[-1] if filtered else endpoint.strip("/")
    leaf = leaf.replace("-", "_").lower()
    # Reserved Rust keywords protection
    if leaf in {"type", "match", "move", "mod", "ref"}:
        leaf = f"{leaf}_"
    return leaf


def main() -> int:
    raw_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("/tmp/kis-registry-raw.json")
    out_path = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("tasks/api-registry.json")

    raw = raw_path.read_text(encoding="utf-8").strip()
    # Sometimes wrapped in quotes by agent-browser eval
    if raw.startswith('"') and raw.endswith('"'):
        raw = json.loads(raw)
    entries = json.loads(raw) if isinstance(raw, str) else raw

    seen_slugs: dict[str, int] = {}
    out = []
    for e in entries:
        category = e["category"]
        name = e["name"]
        endpoint = e["endpoint"]

        mod_prefix = CATEGORY_TO_MODULE.get(category)
        if mod_prefix is None:
            print(f"WARN unmapped category: {category}", file=sys.stderr)
            mod_prefix = "misc"

        leaf = endpoint_to_leaf(endpoint, category)
        base_slug = (mod_prefix.replace("::", "__") + "__" + leaf)
        # Dedup if the same slug appears twice
        if base_slug in seen_slugs:
            seen_slugs[base_slug] += 1
            slug = f"{base_slug}_{seen_slugs[base_slug]}"
        else:
            seen_slugs[base_slug] = 1
            slug = base_slug

        module_path = f"api::{mod_prefix}::{leaf}"
        file_path = "src/" + module_path.replace("::", "/") + ".rs"
        spec_path = f".agent/specs/{slug}.md"

        out.append({
            "category": category,
            "name": name,
            "endpoint": endpoint,
            "slug": slug,
            "module_path": module_path,
            "file_path": file_path,
            "spec_path": spec_path,
            "status": "pending",
        })

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(out, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"wrote {len(out)} entries to {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
