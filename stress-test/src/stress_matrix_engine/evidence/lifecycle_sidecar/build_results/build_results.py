"""Owns lifecycle evidence completeness checks and summary result.
Sở hữu kiểm tra độ đầy đủ lifecycle evidence và kết quả tổng hợp.
"""

from __future__ import annotations

import json

from ....constants.evidence_categories.evidence_categories import FRAMEWORK_RUNTIME_CATEGORIES
from ....constants.framework_catalog.framework_catalog import WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
from ....result.result import Result
from ..types.types import LifecycleEvidence

def build_lifecycle_results(evidence: LifecycleEvidence) -> list[Result]:
    """Require one valid record for every supported framework/category pair.
    Yêu cầu đúng một record hợp lệ cho từng cặp framework/category hỗ trợ.
    """
    imported: list[Result] = []
    expected_keys = {
        (framework, category)
        for framework in WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
        for category in FRAMEWORK_RUNTIME_CATEGORIES
    }
    for framework, category in sorted(expected_keys):
        matching = evidence.records.get((framework, category), [])
        subject = f"web/{framework}"
        if not matching:
            evidence.errors.append(f"missing {category} evidence for {subject}")
            imported.append(
                Result(category, subject, False, "missing lifecycle evidence record", None, "failed")
            )
        elif len(matching) != 1:
            evidence.errors.append(f"duplicate {category} evidence for {subject}")
            imported.append(
                Result(category, subject, False, "duplicate lifecycle evidence records", None, "failed")
            )
        else:
            record = matching[0]
            passed = record["passed"]
            if not passed:
                evidence.errors.append(f"lifecycle operation failed: {category} for {subject}")
            imported.append(
                Result(
                    category, subject, passed, record["detail"], record["returncode"],
                    "verified" if passed else "failed",
                )
            )

    extra_keys = set(evidence.records) - expected_keys
    if extra_keys:
        evidence.errors.append("sidecar contains lifecycle keys outside the supported lane matrix")
    passed_count = sum(
        result.passed and result.evidence_status == "verified" for result in imported
    )
    complete = (
        passed_count == len(expected_keys)
        and not evidence.errors
        and len(evidence.records) == len(expected_keys)
    )
    summary = {
        "record_count": evidence.record_count,
        "expected_record_count": len(expected_keys),
        "sha256": evidence.digest,
        "verified_frameworks": sorted(
            framework
            for framework in WEB_NATIVE_LIFECYCLE_SUITE_BY_FRAMEWORK
            if all(
                any(
                    result.category == category
                    and result.subject == f"web/{framework}"
                    and result.passed
                    and result.evidence_status == "verified"
                    for result in imported
                )
                for category in FRAMEWORK_RUNTIME_CATEGORIES
            )
        ),
        "errors": evidence.errors[:20],
    }
    imported.append(
        Result(
            "framework-lifecycle-evidence-source", "global", complete,
            json.dumps(summary, sort_keys=True), 0 if complete else 1,
            "verified" if complete else "failed",
        )
    )
    return imported

__all__ = ["build_lifecycle_results"]
