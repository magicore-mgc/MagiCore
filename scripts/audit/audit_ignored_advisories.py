#!/usr/bin/env python3
"""Print the reviewed advisory ignore list from deny.toml.

The RustSec waivers live in exactly one place (deny.toml's [advisories] ignore
list) so `cargo deny check` and `cargo audit` cannot disagree about what this
project has accepted.

In Vietnamese: in danh sách advisory đã review trong deny.toml để `cargo deny`
và `cargo audit` dùng chung một nguồn waiver.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ADVISORY_ID = re.compile(r"RUSTSEC-\d{4}-\d{4}")


def advisories_section(text: str) -> str:
    """Return the body of the [advisories] table, without later tables."""
    if "[advisories]" not in text:
        return ""
    body = text.split("[advisories]", 1)[1]
    return body.split("\n[", 1)[0]


def ignored_ids(deny_toml: Path) -> list[str]:
    """Sorted, de-duplicated advisory ids this project has reviewed."""
    body = advisories_section(deny_toml.read_text(encoding="utf-8"))
    return sorted(set(ADVISORY_ID.findall(body)))


def main(argv: list[str]) -> int:
    root = Path(argv[1]) if len(argv) > 1 else Path(".")
    deny = root / "deny.toml"
    if not deny.is_file():
        print(f"error: {deny} not found", file=sys.stderr)
        return 1
    print(" ".join(ignored_ids(deny)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
