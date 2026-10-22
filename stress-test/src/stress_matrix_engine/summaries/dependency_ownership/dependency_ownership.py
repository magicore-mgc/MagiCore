"""Owns the dependency ownership responsibility in the stress-matrix runner.
Sở hữu trách nhiệm dependency ownership trong stress-matrix runner.
"""

from __future__ import annotations

from collections import Counter, defaultdict

from typing import Any

from ...constants.dependency_operations.dependency_operations import DEPENDENCY_OPERATIONS, DEPENDENCY_OWNERS

def dependency_ownership_summary(rows: list[dict[str, Any]]) -> dict[str, Any]:
    """Summarize declared package-operation owners without treating them as runtime proof.
    Tóm lược owner do CLI khai báo, không biến metadata thành bằng chứng runtime.
    """
    counts: dict[str, dict[str, Counter[str]]] = defaultdict(
        lambda: {
            operation: Counter({owner: 0 for owner in DEPENDENCY_OWNERS})
            for operation in DEPENDENCY_OPERATIONS
        }
    )
    frameworks: list[dict[str, Any]] = []
    for row in sorted(rows, key=lambda item: (item["core"], item["framework"])):
        ownership = row.get("dependency_ownership")
        if not isinstance(ownership, dict) or set(ownership) != set(DEPENDENCY_OPERATIONS):
            raise ValueError(
                f"framework {row.get('core')}/{row.get('framework')} has incomplete dependency ownership"
            )
        operation_owners: dict[str, dict[str, Any]] = {}
        for operation in DEPENDENCY_OPERATIONS:
            cell = ownership[operation]
            owner = cell.get("owner") if isinstance(cell, dict) else None
            if not isinstance(owner, str) or owner not in DEPENDENCY_OWNERS:
                raise ValueError(
                    f"framework {row.get('core')}/{row.get('framework')} "
                    f"operation {operation} has invalid owner {owner!r}"
                )
            counts[row["core"]][operation][owner] += 1
            operation_owners[operation] = dict(cell)
        frameworks.append(
            {
                "core": row["core"],
                "framework": row["framework"],
                "qualification_status": row["status"],
                "operations": operation_owners,
            }
        )

    return {
        "source": "mgc capabilities --json",
        "evidence_scope": "declaration-only; does not verify runtime behavior",
        "runtime_verified": False,
        "operations": list(DEPENDENCY_OPERATIONS),
        "by_core_operation": {
            core: {
                operation: dict(sorted(operation_counts.items()))
                for operation, operation_counts in sorted(operation_counts.items())
            }
            for core, operation_counts in sorted(counts.items())
        },
        "frameworks": frameworks,
    }

__all__ = ['dependency_ownership_summary']
