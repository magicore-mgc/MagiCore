"""Owns data types returned by stress-matrix collection.
Sở hữu các kiểu dữ liệu do bước thu thập stress-matrix trả về.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any

from ....result.result import Result

@dataclass(frozen=True)
class MatrixRun:
    """Immutable collected evidence consumed by the summary renderer.
    Bằng chứng thu thập bất biến để bộ kết xuất báo cáo sử dụng.
    """
    version: str
    rows: list[dict[str, Any]]
    results: list[Result]
    failed: list[Result]
    unverified: list[Result]
    full_evidence: list[Result]
    language_runtime_gaps: list[Result]
    unsupported: list[Result]
    not_applicable: list[Result]
    lock_gaps: list[str]
    support_gaps: list[str]
    inventory_count: int
    report_path: Path
    keep_workspace: Path | None

__all__ = ["MatrixRun"]
