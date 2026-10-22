"""Owns the check framework inventory responsibility in the stress-matrix runner.
Sở hữu trách nhiệm check framework inventory trong stress-matrix runner.
"""

from __future__ import annotations

import json

from pathlib import Path

from typing import Any

from ...constants.dependency_operations.dependency_operations import DEPENDENCY_OPERATIONS, DEPENDENCY_OWNERS

def check_framework_inventory(rows: list[dict[str, Any]], baseline_path: Path) -> int:
    """Reject silent framework, qualification, or operation-owner drift.
    Từ chối thay đổi âm thầm danh sách framework, qualification hay owner.
    """
    try:
        baseline = json.loads(baseline_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise RuntimeError(f"cannot read framework inventory baseline: {error}") from error
    if not isinstance(baseline, dict):
        raise RuntimeError("framework inventory baseline has an unsupported schema")
    schema_version = baseline.get("schema_version")
    if (
        not isinstance(schema_version, int)
        or isinstance(schema_version, bool)
        or schema_version != 1
    ):
        raise RuntimeError("framework inventory baseline has an unsupported schema")
    expected_rows = baseline.get("frameworks")
    if not isinstance(expected_rows, list) or not expected_rows:
        raise RuntimeError("framework inventory baseline must contain framework rows")
    captured_from = baseline.get("captured_from")
    if not isinstance(captured_from, dict):
        raise RuntimeError("framework inventory baseline has no capture metadata")
    if (
        not isinstance(captured_from.get("mgc_version"), str)
        or not captured_from["mgc_version"].strip()
        or not isinstance(captured_from.get("git_head"), str)
        or not captured_from["git_head"].strip()
        or not isinstance(captured_from.get("captured_at"), str)
        or not captured_from["captured_at"].strip()
        or not isinstance(captured_from.get("framework_count"), int)
        or isinstance(captured_from.get("framework_count"), bool)
        or captured_from.get("framework_count") != len(expected_rows)
    ):
        raise RuntimeError("framework inventory baseline capture metadata is incomplete or inconsistent")

    def normalize(items: list[dict[str, Any]], source: str) -> dict[tuple[str, str], dict[str, Any]]:
        if not isinstance(items, list):
            raise RuntimeError(f"{source} rows must be a list")
        normalized: dict[tuple[str, str], dict[str, Any]] = {}
        for item in items:
            if not isinstance(item, dict):
                raise RuntimeError(f"{source} contains a malformed row")
            core = item.get("core")
            framework = item.get("framework")
            status = item.get("status")
            owners = item.get("dependency_ownership")
            if (
                not isinstance(core, str)
                or not core.strip()
                or not isinstance(framework, str)
                or not framework.strip()
            ):
                raise RuntimeError(f"{source} has an invalid core/framework identity")
            if not isinstance(status, str) or not status:
                raise RuntimeError(f"{source} row {core}/{framework} has no status")
            if not isinstance(owners, dict) or set(owners) != set(DEPENDENCY_OPERATIONS):
                raise RuntimeError(f"{source} row {core}/{framework} has an incomplete owner table")
            if any(
                not isinstance(owner, str) or owner not in DEPENDENCY_OWNERS
                for owner in owners.values()
            ):
                raise RuntimeError(f"{source} row {core}/{framework} has an unknown owner")
            identity = (core, framework)
            if identity in normalized:
                raise RuntimeError(f"{source} contains duplicate row {core}/{framework}")
            normalized[identity] = {"status": status, "owners": owners}
        return normalized

    expected = normalize(expected_rows, "framework inventory baseline")
    actual_rows = []
    for row in rows:
        if not isinstance(row, dict):
            raise RuntimeError("capability manifest contains a malformed framework row")
        ownership = row.get("dependency_ownership")
        if not isinstance(ownership, dict):
            raise RuntimeError("capability manifest contains a malformed owner table")
        actual_owners = {}
        for operation, cell in ownership.items():
            if not isinstance(cell, dict):
                raise RuntimeError("capability manifest contains a malformed owner cell")
            actual_owners[operation] = cell.get("owner")
        actual_rows.append(
            {
                "core": row.get("core"),
                "framework": row.get("framework"),
                "status": row.get("status"),
                "dependency_ownership": actual_owners,
            }
        )
    actual = normalize(
        actual_rows,
        "capability manifest",
    )
    missing = sorted(expected.keys() - actual.keys())
    unexpected = sorted(actual.keys() - expected.keys())
    if missing or unexpected:
        details = []
        if missing:
            details.append("missing " + ", ".join(f"{core}/{framework}" for core, framework in missing[:20]))
        if unexpected:
            details.append("unexpected " + ", ".join(f"{core}/{framework}" for core, framework in unexpected[:20]))
        raise RuntimeError("framework inventory drift: " + "; ".join(details))

    changes = []
    for identity in sorted(expected):
        before = expected[identity]
        after = actual[identity]
        label = f"{identity[0]}/{identity[1]}"
        if before["status"] != after["status"]:
            changes.append(f"{label} qualification status {before['status']} -> {after['status']}")
        for operation in DEPENDENCY_OPERATIONS:
            previous_owner = before["owners"][operation]
            current_owner = after["owners"][operation]
            if previous_owner != current_owner:
                changes.append(
                    f"{label} {operation} owner {previous_owner} -> {current_owner}"
                )
    if changes:
        raise RuntimeError("framework inventory contract changed: " + "; ".join(changes[:20]))
    return len(expected)

__all__ = ['check_framework_inventory']
