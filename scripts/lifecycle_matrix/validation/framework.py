"""Framework catalog and owner summaries."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..contract.operations import (
    ALL_DEPENDENCY_OPERATIONS,
)
import json

def framework_catalog_errors(catalog, lanes):
    """Require native-qualified frameworks to have full evidence and scope-outs to explain gaps.
    (Framework claim native phải đủ evidence; scope-out phải nêu rõ phần còn thiếu.)"""
    if not isinstance(catalog, list) or not catalog:
        return ["compiled capability output has no framework catalog"]
    errors = []
    by_key = {}
    valid_statuses = {"mgc-engine-path", "scaffold-only"}
    lane_keys = {
        (lane.get("core"), lane.get("framework_id"))
        for lane in lanes
        if isinstance(lane, dict) and lane.get("framework_id")
    }
    lane_by_key = {
        (lane.get("core"), lane.get("framework_id")): lane
        for lane in lanes
        if isinstance(lane, dict) and lane.get("framework_id")
    }
    for row in catalog:
        if not isinstance(row, dict):
            errors.append("framework catalog contains a malformed row")
            continue
        core, framework = row.get("core"), row.get("framework")
        if not isinstance(core, str) or not core or not isinstance(framework, str) or not framework:
            errors.append("framework catalog row has invalid identity")
            continue
        key = (core, framework)
        if key in by_key:
            errors.append(f"duplicate framework catalog identity {core}/{framework}")
            continue
        by_key[key] = row
        status = row.get("status")
        if not isinstance(status, str) or status not in valid_statuses:
            errors.append(f"{core}/{framework} has unknown qualification status {status!r}")
        elif status == "scaffold-only":
            evidence = row.get("evidence")
            if not isinstance(evidence, str) or not evidence.strip():
                errors.append(
                    f"{core}/{framework} scaffold-only entry is missing its scope-out reason"
                )
        if status == "mgc-engine-path" and key not in lane_keys:
            errors.append(f"{core}/{framework} has no lifecycle evidence lane")
        ownership = row.get("dependency_ownership")
        if not isinstance(ownership, dict):
            errors.append(f"{core}/{framework} has no dependency ownership table")
            continue
        unknown_ops = sorted(set(ownership) - set(ALL_DEPENDENCY_OPERATIONS))
        if unknown_ops:
            errors.append(
                f"{core}/{framework} has unknown dependency operations: "
                + ", ".join(unknown_ops)
            )
        for operation in ALL_DEPENDENCY_OPERATIONS:
            cell = ownership.get(operation)
            owner = cell.get("owner") if isinstance(cell, dict) else None
            valid_owners = {"mgc-native", "scaffold-only", "unsupported"}
            if owner not in valid_owners:
                errors.append(
                    f"{core}/{framework} operation {operation} has invalid owner "
                    f"{owner!r}"
                )
                continue
            lane = lane_by_key.get(key)
            if lane is not None:
                matrix_owners = lane.get("owner_by_operation")
                matrix_owner = (
                    matrix_owners.get(operation)
                    if isinstance(matrix_owners, dict)
                    else None
                )
                matches = (
                    owner == "mgc-native"
                    and matrix_owner in {"mgc", "magicore-shared-cas"}
                ) or (
                    owner == "scaffold-only" and matrix_owner == "scaffold-only"
                ) or (
                    owner == "unsupported"
                    and isinstance(matrix_owner, str)
                    and matrix_owner.startswith("unsupported")
                )
                if not matches:
                    errors.append(
                        f"{core}/{framework} operation {operation} differs from lifecycle owner "
                        f"(catalog={owner!r}, lane={matrix_owner!r})"
                    )
    for lane in lanes:
        if not isinstance(lane, dict) or not lane.get("framework_id"):
            continue
        key = (lane.get("core"), lane.get("framework_id"))
        if key not in by_key:
            errors.append(
                f"lifecycle lane {key[0]}/{key[1]} is not present in framework catalog"
            )
    return errors

def print_dependency_owner_summary() -> None:
    """Print declared ownership without implying runtime verification.
    (In ownership đã khai, không ám chỉ đã xác minh runtime.)
    Validate-only report includes evidence requirements and JSON on
    stdout so a consumer never parses the human table.
    (Báo cáo validate-only nêu ownership và evidence cần có, kèm JSON
    trên stdout để consumer không phải parse bảng người-đọc.)"""
    evidence_label = {
        "mgc-native": "native evidence required; not a verdict",
        "delegated": "compatibility only; never native",
        "scaffold-only": "unsupported",
        "unsupported": "unsupported",
    }
    print("=== dependency_owner summary (P0-D) ===")
    print(
        f"  {'lane':<24} {'dependency_owner':<16} "
        f"{'install_owner':<20} evidence status"
    )
    for lane in LANES:
        tag = f"{lane['core']}/{lane['language']}"
        print(
            f"  {tag:<24} {lane['dependency_owner']:<16} "
            f"{lane.get('install_owner', '-'):<20} "
            f"{evidence_label[lane['dependency_owner']]}"
        )
    print(
        json.dumps(
            {
                "dependency_owner_gate": "pass",
                "native_pm_verdict": "not-evaluated-without-lifecycle-evidence",
                "lanes": [
                    {
                        "core": lane["core"],
                        "language": lane["language"],
                        "dependency_owner": lane["dependency_owner"],
                        "native_pm_verdict": "not-evaluated-without-lifecycle-evidence",
                    }
                    for lane in LANES
                ],
            },
            indent=2,
        )
    )

__all__ = ['framework_catalog_errors', 'print_dependency_owner_summary']
