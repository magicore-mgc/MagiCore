"""Owns the lockfile version responsibility in the stress-matrix runner.
Sở hữu trách nhiệm lockfile version trong stress-matrix runner.
"""

from __future__ import annotations

import re

from pathlib import Path

def lockfile_version(lock_path: Path) -> int | None:
    """Read the top-level lock schema major without accepting partial text.
    Đọc major schema đầu file, gồm cả chuỗi revision V4 như ``4.1``.
    """
    try:
        text = lock_path.read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None
    match = re.search(
        r"""(?m)^version\s*=\s*(?:"(\d+(?:\.\d+)*)"|'(\d+(?:\.\d+)*)'|(\d+))\s*(?:#.*)?$""",
        text,
    )
    if match is None:
        return None
    revision = next(value for value in match.groups() if value is not None)
    return int(revision.split(".", maxsplit=1)[0])

__all__ = ['lockfile_version']
