"""Owns the resolve binary responsibility in the stress-matrix runner.
Sở hữu trách nhiệm resolve binary trong stress-matrix runner.
"""

from __future__ import annotations

import os

from pathlib import Path

def resolve_binary(value: str) -> Path:
    """Resolve mgc or its Windows executable name.
    Tìm binary mgc hoặc tên file thực thi trên Windows.
    """
    candidate = Path(value).expanduser().resolve()
    if candidate.is_file():
        return candidate
    if os.name == "nt" and candidate.suffix.lower() != ".exe":
        windows_candidate = candidate.with_suffix(".exe")
        if windows_candidate.is_file():
            return windows_candidate
    raise FileNotFoundError(f"MagiCore binary does not exist: {candidate}")

__all__ = ['resolve_binary']
