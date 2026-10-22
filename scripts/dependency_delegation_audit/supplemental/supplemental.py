"""Supplemental implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
import os

def scan_supplemental_file(abs_path: str, repo_root: str, scanner) -> list:
    """Run a supplemental scanner and preserve read failures in its ledger.
    (Chạy scanner bổ sung; lỗi đọc phải xuất hiện trong ledger.)"""
    rel_path = os.path.relpath(abs_path, repo_root).replace(os.sep, "/")
    try:
        with open(abs_path, "r", encoding="utf-8") as handle:
            return scanner(rel_path, handle.read())
    except (OSError, UnicodeError) as error:
        return [{
            "file": rel_path,
            "line": 1,
            "api": "<read-error>",
            "executable": "<unreadable-source>",
            "review_status": "unreviewed",
            "parse_error": str(error),
        }]

__all__ = ['scan_supplemental_file']
