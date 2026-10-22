"""Existing fail-fast CLI error behavior."""

from __future__ import annotations

import sys

def _fail(message: str, detail: str = "") -> None:
    """Abort generation — a crashed binary run must never look like a
    capability matrix (fail-closed collection, same contract as audit).
    Huỷ sinh matrix — binary crash không bao giờ được thành matrix."""
    print(f"LIFECYCLE MATRIX COLLECTION FAILED: {message}", file=sys.stderr)
    if detail:
        print(detail[-4000:], file=sys.stderr)
    sys.exit(1)

__all__ = ['_fail']
