"""Owns the universal support responsibility in the stress-matrix runner.
Sở hữu trách nhiệm universal support trong stress-matrix runner.
"""

from __future__ import annotations

from typing import Any

from ...constants.dependency_operations.dependency_operations import DEPENDENCY_OPERATIONS

from ...constants.framework_catalog.framework_catalog import UNIVERSAL_FRAMEWORK_CATEGORIES, UNIVERSAL_GLOBAL_CATEGORIES

from ..evidence.evidence import evidence_category_gaps

from ..language_runtime.language_runtime import framework_language_runtime_gaps

from ...result.result import Result

def universal_support_gaps(results: list[Result], rows: list[dict[str, Any]]) -> list[str]:
    """Fail closed until every framework/core dimension has runtime evidence.
    Fail closed đến khi mọi framework/core có evidence runtime cho các chiều yêu cầu.
    """
    if not rows:
        return ["framework-inventory is empty"]

    gaps = evidence_category_gaps(results, rows, UNIVERSAL_FRAMEWORK_CATEGORIES)
    gaps.extend(framework_language_runtime_gaps(results, rows))
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        ownership = row.get("dependency_ownership")
        if not isinstance(ownership, dict):
            gaps.append(f"dependency-owner:{subject} ownership declaration is missing")
            continue
        for operation in DEPENDENCY_OPERATIONS:
            cell = ownership.get(operation)
            owner = cell.get("owner") if isinstance(cell, dict) else None
            if owner != "mgc-native":
                gaps.append(f"dependency-owner:{subject}:{operation} is {owner or 'missing'}")

    for category in UNIVERSAL_GLOBAL_CATEGORIES:
        matches = [result for result in results if result.category == category]
        if not matches:
            gaps.append(f"{category}: global command contract has no evidence")
            continue
        for result in matches:
            if not result.passed or result.evidence_status != "verified":
                gaps.append(
                    f"{category}:{result.subject} is {result.evidence_status} (passed={result.passed})"
                )
    return gaps

__all__ = ['universal_support_gaps']
