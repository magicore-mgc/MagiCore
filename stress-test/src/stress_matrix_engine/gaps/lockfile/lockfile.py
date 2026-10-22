"""Owns the lockfile responsibility in the stress-matrix runner.
Sở hữu trách nhiệm lockfile trong stress-matrix runner.
"""

from __future__ import annotations

from collections import defaultdict

from typing import Any

from ...constants.core_inventory.core_inventory import V4_INSTALL_CORES

from ...constants.evidence_categories.evidence_categories import FULL_EVIDENCE_CATEGORIES

from ...result.result import Result

def lockfile_evidence_gaps(results: list[Result], rows: list[dict[str, Any]]) -> list[str]:
    """Require one honest V3/V4 result per framework and verified claimed writers.
    Bắt buộc mỗi framework có một kết quả V3/V4 trung thực và writer đã claim phải xanh.
    """
    by_result: dict[tuple[str, str], list[Result]] = defaultdict(list)
    for result in results:
        if result.category in {"mgc-lock-v3-empty-graph", "mgc-lock-v4-empty-graph"}:
            by_result[(result.category, result.subject)].append(result)

    gaps: list[str] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        native_install = row["dependency_ownership"]["install"]["owner"] == "mgc-native"
        expected = {
            "mgc-lock-v3-empty-graph": {"verified"} if native_install else {"unsupported"},
            "mgc-lock-v4-empty-graph": {"verified"}
            if native_install and row["core"] in V4_INSTALL_CORES
            else {"unsupported"},
        }
        for category, expected_status in expected.items():
            matches = by_result.get((category, subject), [])
            if len(matches) != 1:
                gaps.append(f"{category}:{subject} expected one result, found {len(matches)}")
                continue
            result = matches[0]
            valid_evidence = result.evidence_status in expected_status and result.passed
            if not valid_evidence:
                gaps.append(
                    f"{category}:{subject} expected one of {sorted(expected_status)}, got "
                    f"{result.evidence_status} (passed={result.passed})"
                )
    return gaps

def full_evidence_gaps(results: list[Result]) -> list[Result]:
    """List unresolved optimizer/security evidence covered by the normal CI gate.
    Liệt kê evidence optimizer/security chưa giải quyết thuộc gate CI thông thường.
    """
    return [
        result
        for result in results
        if result.category in FULL_EVIDENCE_CATEGORIES
        and result.evidence_status == "unverified"
    ]

__all__ = ['lockfile_evidence_gaps', 'full_evidence_gaps']
