"""Persist the lifecycle matrix document at its configured path."""

from __future__ import annotations

import json
import os

def write_matrix_document(out: dict, results: list[dict]) -> str:
    """Write formatted JSON and report its destination."""
    target = os.environ.get("MGC_LIFECYCLE_MATRIX_OUT", "docs/specs/lifecycleCapabilityMatrix.json")
    # CI checkouts do not carry the gitignored docs/specs tree — create
    # the parent directory before writing.
    # Checkout CI không có cây docs/specs (bị gitignored) — tạo thư mục
    # cha trước khi ghi.
    os.makedirs(os.path.dirname(target) or ".", exist_ok=True)
    with open(target, "w") as f:
        json.dump(out, f, indent=2)
        f.write("\n")
    print(f"lifecycle capability matrix written to {target} ({len(results)} lanes)")
    return target

__all__ = ["write_matrix_document"]
