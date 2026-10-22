"""Owns the evidence responsibility in the stress-matrix runner.
Sở hữu trách nhiệm evidence trong stress-matrix runner.
"""

from __future__ import annotations

from collections import defaultdict

from typing import Any

from ...constants.evidence_categories.evidence_categories import FRAMEWORK_RUNTIME_CATEGORIES

from ...result.result import Result

def evidence_category_gaps(
    results: list[Result],
    rows: list[dict[str, Any]],
    categories: tuple[str, ...],
) -> list[str]:
    """Require exactly one verified runtime result for every lane/category.
    Bắt buộc mỗi lane/chiều có đúng một kết quả runtime đã xác minh.
    """
    by_key: dict[tuple[str, str], list[Result]] = defaultdict(list)
    for result in results:
        by_key[(result.category, result.subject)].append(result)

    gaps: list[str] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        for category in categories:
            matches = by_key.get((category, subject), [])
            if len(matches) != 1:
                gaps.append(f"{category}:{subject} expected exactly one runtime result, found {len(matches)}")
                continue
            result = matches[0]
            if not result.passed or result.evidence_status != "verified":
                gaps.append(
                    f"{category}:{subject} is {result.evidence_status} (passed={result.passed})"
                )
    return gaps

def framework_runtime_verified(results: list[Result], rows: list[dict[str, Any]]) -> bool:
    """Report runtime support only after every framework CLI operation is verified.
    Chỉ ghi runtime đã xác minh khi mọi framework có evidence CLI đủ chiều.
    """
    return bool(rows) and not evidence_category_gaps(
        results,
        rows,
        FRAMEWORK_RUNTIME_CATEGORIES,
    )

__all__ = ['evidence_category_gaps', 'framework_runtime_verified']
