"""Owns stress-matrix console summary and exit gate.
Sở hữu tóm tắt console và cổng exit của stress-matrix.
"""

from __future__ import annotations

import platform
from argparse import Namespace

from ....constants.core_inventory.core_inventory import V4_INSTALL_CORES
from ....constants.evidence_categories.evidence_categories import MAX_SUPPORT_GAP_DETAILS
from ..types.types import MatrixRun

def render_matrix_summary(args: Namespace, run: MatrixRun) -> int:
    """Print the existing report summary and return its original exit status.
    In tóm tắt hiện có và trả exit status ban đầu.
    """
    print(f"MagiCore {run.version} on {platform.platform()}")
    print(
        f"Framework inventory baseline matched: {run.inventory_count} rows "
        f"({args.inventory_manifest})"
    )
    print(
        f"Frameworks: {len(run.rows)}; checks: {len(run.results)}; "
        f"contract passes: {len(run.results) - len(run.failed)}; "
        f"failed: {len(run.failed)}; unverified evidence: {len(run.unverified)}"
    )
    print(
        f"Unsupported evidence: {len(run.unsupported)}; not applicable: {len(run.not_applicable)}; "
        "expected refusals are not support claims"
    )
    print(
        f"Verified evidence checks: {sum(r.passed and r.evidence_status == 'verified' for r in run.results)}; "
        "framework runtime is not verified by this runner"
    )
    print(f"Unverified optimizer/security evidence: {len(run.full_evidence)}")
    print(f"Unverified standalone language runtime/parser probes: {len(run.language_runtime_gaps)}")
    print(
        f"Lockfile evidence gaps: {len(run.lock_gaps)} "
        f"(empty-graph replay; V4 writers: {','.join(sorted(V4_INSTALL_CORES))})"
    )
    native_install = sum(
        row["dependency_ownership"]["install"]["owner"] == "mgc-native" for row in run.rows
    )
    print(
        f"Declared native install ownership: {native_install}/{len(run.rows)} frameworks; "
        "ownership is capability metadata, not runtime verification"
    )
    print(
        f"Universal support gate: {'ENFORCED' if args.require_universal_support else 'diagnostic'}; "
        f"{len(run.support_gaps)} evidence/ownership gap(s)"
    )
    for gap in run.support_gaps[:MAX_SUPPORT_GAP_DETAILS]:
        print(f"UNIVERSAL SUPPORT GAP: {gap}")
    if len(run.support_gaps) > MAX_SUPPORT_GAP_DETAILS:
        print(f"... and {len(run.support_gaps) - MAX_SUPPORT_GAP_DETAILS} more; see the JSON evidence report")
    print(f"Evidence: {run.report_path}")
    if run.failed and args.keep_workspace:
        print(f"Failed fixtures: {run.keep_workspace}")
    for result in run.failed:
        print(f"FAIL [{result.category}] {result.subject}: {result.detail}")
    for gap in run.lock_gaps:
        print(f"LOCKFILE EVIDENCE GAP: {gap}")
    if run.unverified:
        print("UNVERIFIED evidence: " + ", ".join(f"{r.category}:{r.subject}" for r in run.unverified[:20]))
        if len(run.unverified) > 20:
            print(f"... and {len(run.unverified) - 20} more; see the JSON report")
    return 1 if (
        run.failed
        or (args.require_full_evidence and run.full_evidence)
        or (args.require_language_runtime_evidence and run.language_runtime_gaps)
        or (args.require_lockfile_evidence and run.lock_gaps)
        or (args.require_universal_support and run.support_gaps)
    ) else 0

__all__ = ["render_matrix_summary"]
