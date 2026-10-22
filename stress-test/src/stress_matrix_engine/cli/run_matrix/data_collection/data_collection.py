"""Owns stress-matrix project execution and evidence collection.
Sở hữu chạy project matrix và thu thập bằng chứng stress-matrix.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import tempfile
import sys
from argparse import Namespace
from pathlib import Path

from ....contracts.cli.cli import run_cli_contracts
from ....contracts.project_dimensions.project_dimensions import run_project_dimensions
from ....evidence.lifecycle_sidecar.lifecycle_sidecar import read_web_lifecycle_evidence
from ....evidence.report.report import write_report
from ....evidence.source_provenance.source_provenance import collect_source_provenance
from ....framework_matrix.create_frameworks.create_frameworks import create_frameworks
from ....gaps.lockfile.lockfile import full_evidence_gaps
from ....gaps.language_runtime.language_runtime import language_runtime_evidence_gaps
from ....gaps.lockfile.lockfile import lockfile_evidence_gaps
from ....gaps.universal_support.universal_support import universal_support_gaps
from ....inventory.capability_rows.capability_rows import capability_rows
from ....inventory.check_framework_inventory.check_framework_inventory import check_framework_inventory
from ....process.isolated_environment.isolated_environment import isolated_environment
from ....result.result import Result
from ..types.types import MatrixRun

def collect_matrix_data(args: Namespace, binary: Path, report_path: Path) -> MatrixRun | None:
    """Run the matrix and save its JSON evidence report.
    Chạy matrix và lưu JSON báo cáo bằng chứng.
    """
    results: list[Result] = []
    keep_path: Path | None = None
    with tempfile.TemporaryDirectory(prefix="mgc-stress-matrix-") as temporary:
        root = Path(temporary)
        env = isolated_environment(root)
        try:
            version, rows = capability_rows(binary, root, env)
        except (RuntimeError, OSError, subprocess.TimeoutExpired) as error:
            print(f"FAIL capability manifest: {error}", file=sys.stderr)
            return None
        try:
            inventory_count = check_framework_inventory(rows, args.inventory_manifest)
        except RuntimeError as error:
            print(f"FAIL framework inventory baseline: {error}", file=sys.stderr)
            return None

        results.extend(run_cli_contracts(binary, root, env))
        scaffold_results, projects = create_frameworks(
            binary, rows, root, env, args.framework_timeout
        )
        results.extend(scaffold_results)
        results.extend(run_project_dimensions(binary, projects, rows, root, env))
        results.extend(
            read_web_lifecycle_evidence(
                os.environ.get("MGC_STRESS_LIFECYCLE_EVIDENCE"), binary
            )
        )
        support_gaps = universal_support_gaps(results, rows)
        write_report(
            report_path, version, results, rows,
            universal_gate_enforced=args.require_universal_support,
            support_gaps=support_gaps,
            source_provenance=collect_source_provenance(binary),
        )
        failed = [result for result in results if not result.passed]
        unverified = [result for result in results if result.evidence_status == "unverified"]
        full_evidence = full_evidence_gaps(results)
        language_runtime_gaps = language_runtime_evidence_gaps(results)
        unsupported = [result for result in results if result.evidence_status == "unsupported"]
        not_applicable = [result for result in results if result.evidence_status == "not_applicable"]
        lock_gaps = lockfile_evidence_gaps(results, rows)
        if failed and args.keep_workspace:
            keep_path = Path(args.keep_workspace).expanduser().resolve()
            if keep_path.exists():
                shutil.rmtree(keep_path)
            shutil.copytree(root, keep_path)
    return MatrixRun(
        version=version, rows=rows, results=results, failed=failed, unverified=unverified,
        full_evidence=full_evidence, language_runtime_gaps=language_runtime_gaps,
        unsupported=unsupported, not_applicable=not_applicable, lock_gaps=lock_gaps,
        support_gaps=support_gaps, inventory_count=inventory_count, report_path=report_path,
        keep_workspace=keep_path,
    )

__all__ = ["collect_matrix_data"]
