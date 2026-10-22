"""Owns lifecycle sidecar parsing data types.
Sở hữu kiểu dữ liệu phân tích lifecycle sidecar.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

@dataclass
class LifecycleEvidence:
    """Validated records plus bounded parsing evidence.
    Record đã kiểm tra cùng bằng chứng phân tích có giới hạn.
    """
    records: dict[tuple[str, str], list[dict[str, Any]]] = field(default_factory=dict)
    errors: list[str] = field(default_factory=list)
    record_count: int = 0
    digest: str | None = None

__all__ = ["LifecycleEvidence"]
