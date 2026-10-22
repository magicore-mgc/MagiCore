"""Owns lifecycle sidecar import orchestration.
Sở hữu điều phối import lifecycle sidecar.
"""

from __future__ import annotations

from pathlib import Path

from .build_results.build_results import build_lifecycle_results
from .read_records.read_records import read_sidecar_records
from ...result.result import Result

def read_web_lifecycle_evidence(path: str | Path | None, binary: Path) -> list[Result]:
    """Import complete, binary-bound evidence from native web lifecycle E2Es.
    Nạp bằng chứng E2E web đầy đủ và gắn với đúng binary đang được stress-test.
    """
    if path is None or not str(path).strip():
        return []
    return build_lifecycle_results(read_sidecar_records(Path(path).expanduser(), binary))

__all__ = ["read_web_lifecycle_evidence"]
