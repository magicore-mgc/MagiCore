"""Owns the source language responsibility in the stress-matrix runner.
Sở hữu trách nhiệm source language trong stress-matrix runner.
"""

from __future__ import annotations

import json

from collections import defaultdict

from typing import Any

from ...result.result import Result

def source_language_inventory_summary(results: list[Result]) -> dict[str, Any]:
    """Summarize generated source-language evidence, never runtime claims.
    Tóm tắt bằng chứng ngôn ngữ trong scaffold, không gán thành runtime claim.
    """
    frameworks: list[dict[str, Any]] = []
    by_language: dict[str, dict[str, int]] = defaultdict(
        lambda: {"frameworks": 0, "source_files": 0}
    )
    for result in results:
        if result.category != "framework-language-scaffold":
            continue
        try:
            evidence = json.loads(result.detail)
        except json.JSONDecodeError:
            evidence = {}
        languages = evidence.get("languages_or_source_formats", [])
        files_by_language = evidence.get("source_files_by_language_or_format", {})
        if not isinstance(languages, list):
            languages = []
        if not isinstance(files_by_language, dict):
            files_by_language = {}
        frameworks.append(
            {
                "subject": result.subject,
                "evidence_status": result.evidence_status,
                "languages_or_source_formats": languages,
                "source_files_by_language_or_format": files_by_language,
                "runtime_verified": False,
            }
        )
        for language in languages:
            if not isinstance(language, str):
                continue
            by_language[language]["frameworks"] += 1
            count = files_by_language.get(language, 0)
            if isinstance(count, int) and not isinstance(count, bool) and count >= 0:
                by_language[language]["source_files"] += count
    return {
        "evidence_scope": "generated scaffold files only; file extensions are not runtime proof",
        "runtime_verified": False,
        "frameworks_with_inventory": len(frameworks),
        "by_language_or_format": dict(sorted(by_language.items())),
        "frameworks": frameworks,
    }

__all__ = ['source_language_inventory_summary']
