"""Owns the evidence categories responsibility in the stress-matrix runner.
Sở hữu trách nhiệm evidence categories trong stress-matrix runner.
"""

from __future__ import annotations

FRAMEWORK_RUNTIME_CATEGORIES = (
    "framework-cli-install",
    "framework-cli-run",
    "framework-cli-test",
    "framework-cli-build",
    "framework-cli-dev",
)

FULL_EVIDENCE_CATEGORIES = (
    "optimizer",
    "security-audit",
    "security-audit-nonempty",
)

MAX_SUPPORT_GAP_DETAILS = 40

MAX_WEB_LIFECYCLE_EVIDENCE_BYTES = 1_000_000

SOURCE_PROVENANCE_TIMEOUT_SECONDS = 5

__all__ = ['FRAMEWORK_RUNTIME_CATEGORIES', 'FULL_EVIDENCE_CATEGORIES', 'MAX_SUPPORT_GAP_DETAILS', 'MAX_WEB_LIFECYCLE_EVIDENCE_BYTES', 'SOURCE_PROVENANCE_TIMEOUT_SECONDS']
