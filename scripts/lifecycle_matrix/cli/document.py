"""Build the machine-readable lifecycle matrix document."""

from __future__ import annotations

from .context import MatrixRunContext
from ..contract.dimensions import ALL_DIMENSIONS
from ..contract.status import PASS_STATUSES, SCHEMA_VERSION, STATUS_FAILED, STATUS_UNSUPPORTED, STATUS_UNVERIFIED
from ..evidence.platform import _platform_evidence_load
from ..evidence.records import FRAMEWORK_QUALIFICATION_EVIDENCE, PLATFORM_EVIDENCE_RELEASE_SCOPE
from ..evidence.source import lifecycle_platform_name, lifecycle_source_matches

def build_matrix_document(context: MatrixRunContext, results: list[dict]) -> dict:
    """Create the v7 document from source declarations and collected evidence."""
    commit = context.commit
    now = context.generated_at
    source_tree_clean_before_run = context.source_tree_clean_before_run
    out = {
        "schema_version": SCHEMA_VERSION,
        "generated_at": now,
        "commit": commit,
        "working_tree_clean": lifecycle_source_matches(
            commit, source_tree_clean_before_run
        ),
        "platform": lifecycle_platform_name(),
        "matrix_kind": "lifecycle",
        "release_scope": [list(item) for item in sorted(PLATFORM_EVIDENCE_RELEASE_SCOPE)],
        "framework_catalog": FRAMEWORK_QUALIFICATION_EVIDENCE,
        # The six-value status vocabulary (Gate 11-C schema v7) so every
        # consumer interprets statuses identically — only *-pass statuses
        # satisfy, unverified/unsupported/failed satisfy nothing.
        # (Bộ 6 trạng thái (Gate 11-C schema v7) để mọi consumer đọc
        # trạng thái thống nhất — chỉ trạng thái *-pass thỏa,
        # unverified/unsupported/failed không thỏa gì.)
        "status_vocabulary": list(PASS_STATUSES) + [
            STATUS_UNVERIFIED, STATUS_UNSUPPORTED, STATUS_FAILED,
        ],
        "dimensions": ALL_DIMENSIONS,
        # P0-3: per-OS CONSECUTIVE green-run platform evidence (see the
        # counter block above). Embedded so one artifact carries both the
        # matrix verdicts and how much platform evidence backs them.
        # (P0-3: evidence platform số lần xanh LIÊN TIẾP theo OS (xem block
        # counter bên trên). Nhúng vào đây để một artifact mang cả verdict
        # matrix lẫn lượng evidence platform đứng sau verdict đó.)
        "platform_evidence": _platform_evidence_load(),
        "lanes": results,
    }
    return out

__all__ = ["build_matrix_document"]
