"""Owns the report responsibility in the stress-matrix runner.
Sở hữu trách nhiệm report trong stress-matrix runner.
"""

from __future__ import annotations

import datetime as dt

import json

import platform

from dataclasses import asdict

from pathlib import Path

from typing import Any

from ...constants.dependency_operations.dependency_operations import DEPENDENCY_OPERATIONS

from ...constants.framework_catalog.framework_catalog import UNIVERSAL_FRAMEWORK_CATEGORIES, UNIVERSAL_GLOBAL_CATEGORIES

from ...gaps.evidence.evidence import framework_runtime_verified

from ...result.result import Result

from ...summaries.dependency_ownership.dependency_ownership import dependency_ownership_summary

from ...summaries.lifecycle.lifecycle import framework_lifecycle_evidence_summary

from ...summaries.source_language.source_language import source_language_inventory_summary

def write_report(
    path: Path,
    version: str,
    results: list[Result],
    rows: list[dict[str, Any]],
    *,
    universal_gate_enforced: bool = False,
    support_gaps: list[str] | None = None,
    source_provenance: dict[str, Any] | None = None,
) -> None:
    """Write machine-readable run evidence.
    Ghi bằng chứng lần chạy ở định dạng máy đọc.
    """
    runtime_verified = framework_runtime_verified(results, rows)
    payload = {
        "schema_version": 7,
        "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
        "mgc_version": version,
        "platform": platform.platform(),
        "source_provenance": (
            source_provenance
            if source_provenance is not None
            else {
                "git_commit": None,
                "working_tree_clean": None,
                "binary_sha256": None,
            }
        ),
        "framework_inventory": len(rows),
        "evidence_contract": {
            "framework_runtime_verified": runtime_verified,
            "run_test": "synthetic Python sentinel is orchestration-only; universal proof requires per-framework CLI runtime evidence",
            "language_runtime": "universal proof requires one verified probe per standalone source language emitted by each scaffold; framework-only source formats rely on framework lifecycle evidence",
            "native_install": "dependency manifests are rewritten to empty graphs before install",
            "lockfile": "V3/V4 checks verify empty-graph write and frozen replay only",
            "framework_lifecycle": "real native lifecycle evidence is imported from an E2E sidecar for the 18 listed web lanes only; absent or incomplete sidecars do not establish support",
            "language_inventory": "generated source extensions only; no language runtime or framework command is executed",
        },
        "framework_lifecycle_evidence": framework_lifecycle_evidence_summary(results),
        "source_language_inventory": source_language_inventory_summary(results),
        "universal_support_gate": {
            "enforced": universal_gate_enforced,
            "status": (
                "not-evaluated"
                if support_gaps is None
                else "ready"
                if not support_gaps
                else "blocked"
            ),
            "blocker_count": None if support_gaps is None else len(support_gaps),
            "blockers": [] if support_gaps is None else support_gaps,
            "required_framework_categories": list(UNIVERSAL_FRAMEWORK_CATEGORIES),
            "required_language_runtime_evidence": "one verified runtime/parser probe per standalone language emitted by each framework scaffold",
            "required_global_categories": list(UNIVERSAL_GLOBAL_CATEGORIES),
            "dependency_operations_require_native": list(DEPENDENCY_OPERATIONS),
        },
        "dependency_ownership_declaration": dependency_ownership_summary(rows),
        "passed": sum(result.passed for result in results),
        "failed": sum(not result.passed for result in results),
        "verified_evidence": sum(
            result.passed and result.evidence_status == "verified" for result in results
        ),
        "unverified_evidence": sum(result.evidence_status == "unverified" for result in results),
        "unsupported_evidence": sum(result.evidence_status == "unsupported" for result in results),
        "not_applicable_evidence": sum(result.evidence_status == "not_applicable" for result in results),
        "results": [asdict(result) for result in results],
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

__all__ = ['write_report']
