"""Owns the capability rows responsibility in the stress-matrix runner.
Sở hữu trách nhiệm capability rows trong stress-matrix runner.
"""

from __future__ import annotations

import json

from pathlib import Path

from typing import Any

from ...constants.core_inventory.core_inventory import CORE_NAMES

from ...constants.dependency_operations.dependency_operations import DEPENDENCY_OPERATIONS, DEPENDENCY_OWNERS

from ...process.run_command.run_command import run_command

def capability_rows(binary: Path, root: Path, env: dict[str, str]) -> tuple[str, list[dict[str, Any]]]:
    """Read and validate the CLI's own framework inventory.
    Đọc và kiểm tra danh mục framework do chính CLI công bố.
    """
    completed = run_command(
        [str(binary), "capabilities", "--json"],
        cwd=root,
        env=env,
        timeout_seconds=30,
    )
    if completed.returncode != 0:
        raise RuntimeError(f"mgc capabilities --json failed ({completed.returncode}): {completed.stdout[-1200:]}")
    try:
        payload = json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(f"mgc capabilities emitted invalid JSON: {error}") from error
    if not isinstance(payload, dict):
        raise RuntimeError("mgc capabilities JSON root must be an object")
    version = payload.get("version")
    core_records = payload.get("cores")
    if not isinstance(version, str) or not isinstance(core_records, list):
        raise RuntimeError("capability manifest must contain version and cores")

    actual_cores: set[str] = set()
    rows: list[dict[str, Any]] = []
    for core_record in core_records:
        if not isinstance(core_record, dict):
            raise RuntimeError("capability manifest contains a malformed core row")
        core = core_record.get("core")
        frameworks = core_record.get("framework_qualification")
        if not isinstance(core, str) or not isinstance(frameworks, list):
            raise RuntimeError("capability core row is missing framework_qualification")
        if core in actual_cores:
            raise RuntimeError(f"duplicate core in capability manifest: {core}")
        actual_cores.add(core)
        for framework_row in frameworks:
            if not isinstance(framework_row, dict):
                raise RuntimeError(f"core {core} contains a malformed framework row")
            framework = framework_row.get("framework")
            status = framework_row.get("status")
            if not isinstance(framework, str) or not framework.strip():
                raise RuntimeError(f"invalid framework row for core {core}")
            if not isinstance(status, str) or not status:
                raise RuntimeError(f"framework {core}/{framework} has no qualification status")
            ownership = framework_row.get("dependency_ownership")
            if not isinstance(ownership, dict):
                raise RuntimeError(f"framework {core}/{framework} has no dependency ownership table")
            missing_operations = sorted(set(DEPENDENCY_OPERATIONS) - set(ownership))
            unknown_operations = sorted(set(ownership) - set(DEPENDENCY_OPERATIONS))
            if missing_operations or unknown_operations:
                details = []
                if missing_operations:
                    details.append("missing " + ", ".join(missing_operations))
                if unknown_operations:
                    details.append("unknown " + ", ".join(unknown_operations))
                raise RuntimeError(
                    f"framework {core}/{framework} dependency ownership operations are incomplete: "
                    + "; ".join(details)
                )
            normalized_ownership: dict[str, dict[str, Any]] = {}
            for operation in DEPENDENCY_OPERATIONS:
                cell = ownership[operation]
                owner = cell.get("owner") if isinstance(cell, dict) else None
                if not isinstance(owner, str) or owner not in DEPENDENCY_OWNERS:
                    raise RuntimeError(
                        f"framework {core}/{framework} operation {operation} has invalid owner {owner!r}"
                    )
                normalized_ownership[operation] = dict(cell)
            rows.append(
                {
                    "core": core,
                    "framework": framework,
                    "status": status,
                    "dependency_ownership": normalized_ownership,
                }
            )

    missing = sorted(set(CORE_NAMES) - actual_cores)
    if missing:
        raise RuntimeError(f"capability manifest omitted core(s): {', '.join(missing)}")
    unexpected = sorted(actual_cores - set(CORE_NAMES))
    if unexpected:
        raise RuntimeError(
            f"capability manifest introduced unsupported core(s): {', '.join(unexpected)}"
        )
    if not rows:
        raise RuntimeError("capability manifest contains no framework rows")
    unique = {(row["core"], row["framework"]) for row in rows}
    if len(unique) != len(rows):
        raise RuntimeError("capability manifest contains duplicate core/framework rows")
    return version, rows

__all__ = ['capability_rows']
