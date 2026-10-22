"""Owns the language runtime responsibility in the stress-matrix runner.
Sở hữu trách nhiệm language runtime trong stress-matrix runner.
"""

from __future__ import annotations

import json

import sys

from collections import defaultdict

from typing import Any

from ...constants.language_catalog.language_catalog import NON_STANDALONE_RUNTIME_FORMATS

from ...result.result import Result

def framework_language_runtime_gaps(
    results: list[Result], rows: list[dict[str, Any]]
) -> list[str]:
    """Require runtime evidence for every standalone language in each scaffold.
    Bắt buộc có runtime evidence cho từng ngôn ngữ độc lập mà scaffold sinh ra.
    """
    by_key: dict[tuple[str, str], list[Result]] = defaultdict(list)
    for result in results:
        by_key[(result.category, result.subject)].append(result)

    gaps: list[str] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        subject = f"{row['core']}/{row['framework']}"
        scaffold_results = by_key.get(("framework-language-scaffold", subject), [])
        if len(scaffold_results) != 1:
            gaps.append(
                f"framework-language-scaffold:{subject} expected exactly one result, found {len(scaffold_results)}"
            )
            continue

        scaffold_result = scaffold_results[0]
        if not scaffold_result.passed or scaffold_result.evidence_status != "verified":
            gaps.append(
                f"framework-language-scaffold:{subject} is "
                f"{scaffold_result.evidence_status} (passed={scaffold_result.passed})"
            )
            continue
        try:
            evidence = json.loads(scaffold_result.detail)
        except json.JSONDecodeError:
            gaps.append(f"framework-language-scaffold:{subject} has malformed language inventory")
            continue

        languages = evidence.get("languages_or_source_formats") if isinstance(evidence, dict) else None
        if not isinstance(languages, list) or any(not isinstance(item, str) for item in languages):
            gaps.append(f"framework-language-scaffold:{subject} has malformed language inventory")
            continue

        for language in sorted(set(languages) - NON_STANDALONE_RUNTIME_FORMATS):
            probe_subject = f"{subject}/{language}"
            probes = by_key.get(("framework-language-runtime-probe", probe_subject), [])
            if len(probes) != 1:
                gaps.append(
                    f"framework-language-runtime-probe:{probe_subject} expected exactly one result, found {len(probes)}"
                )
                continue
            probe = probes[0]
            platform_only = language in {"Objective-C", "Objective-C++"} and sys.platform != "darwin"
            if platform_only and probe.passed and probe.evidence_status == "not_applicable":
                continue
            if not probe.passed or probe.evidence_status != "verified":
                gaps.append(
                    f"framework-language-runtime-probe:{probe_subject} is "
                    f"{probe.evidence_status} (passed={probe.passed})"
                )

    return gaps

def language_runtime_evidence_gaps(results: list[Result]) -> list[Result]:
    """Return source-language probes without verified execution/parser evidence.
    Liệt kê probe ngôn ngữ nguồn chưa có bằng chứng chạy hoặc phân tích cú pháp.
    """
    return [
        result
        for result in results
        if result.category == "framework-language-runtime-probe"
        and not (
            result.passed and result.evidence_status == "not_applicable"
        )
        and (not result.passed or result.evidence_status != "verified")
    ]

__all__ = ['framework_language_runtime_gaps', 'language_runtime_evidence_gaps']
