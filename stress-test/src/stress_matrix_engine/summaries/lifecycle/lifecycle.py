"""Owns the lifecycle responsibility in the stress-matrix runner.
Sở hữu trách nhiệm lifecycle trong stress-matrix runner.
"""

from __future__ import annotations

import json

from typing import Any

from ...constants.evidence_categories.evidence_categories import FRAMEWORK_RUNTIME_CATEGORIES

from ...constants.framework_catalog.framework_catalog import WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK

from ...result.result import Result

def framework_lifecycle_evidence_summary(results: list[Result]) -> dict[str, Any]:
    """Describe sidecar coverage without extending it beyond its tested lanes.
    Mô tả phạm vi sidecar, không suy rộng ra ngoài lane đã chạy.
    """
    source = next(
        (result for result in results if result.category == "framework-lifecycle-evidence-source"),
        None,
    )
    if source is None:
        return {
            "status": "not-provided",
            "scope": "18 native web lanes, five CLI lifecycle operations per lane",
            "record_count": 0,
            "expected_record_count": len(WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK)
            * len(FRAMEWORK_RUNTIME_CATEGORIES),
            "sha256": None,
            "verified_frameworks": [],
        }
    try:
        detail = json.loads(source.detail)
    except json.JSONDecodeError:
        detail = {}
    return {
        "status": "complete" if source.passed and source.evidence_status == "verified" else "invalid-or-incomplete",
        "scope": "18 native web lanes, five CLI lifecycle operations per lane",
        "record_count": detail.get("record_count", 0),
        "expected_record_count": detail.get(
            "expected_record_count",
            len(WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK) * len(FRAMEWORK_RUNTIME_CATEGORIES),
        ),
        "sha256": detail.get("sha256"),
        "verified_frameworks": detail.get("verified_frameworks", []),
        "errors": detail.get("errors", []),
    }

__all__ = ['framework_lifecycle_evidence_summary']
