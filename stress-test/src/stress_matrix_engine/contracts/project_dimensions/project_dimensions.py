"""Owns the project dimensions responsibility in the stress-matrix runner.
Sở hữu trách nhiệm project dimensions trong stress-matrix runner.
"""

from __future__ import annotations

import shutil

import subprocess

from pathlib import Path

from typing import Any

from ..framework_scripts.framework_scripts import run_framework_script_contracts

from ..lockfile.lockfile import run_lockfile_contracts

from ..native_install.native_install import run_native_install_contract

from ..shared_store.shared_store import run_shared_store_check

from ..store_scope.store_scope import run_store_scope_contract

from ...evidence.dimension_assessment.dimension_assessment import assess_dimension

from ...process.run_command.run_command import run_command

from ...result.result import Result

from ...runtime.probe.matrix.matrix import run_framework_language_runtime_probe

def run_project_dimensions(
    binary: Path,
    projects: list[tuple[str, str, Path]],
    rows: list[dict[str, Any]],
    root: Path,
    env: dict[str, str],
) -> list[Result]:
    """Exercise runtime scripts, owned installs, store scope, and quality dimensions.
    Chạy script runtime, install được MGC sở hữu, scope store và quality dimensions.
    """
    results: list[Result] = []
    rows_by_identity = {(row["core"], row["framework"]): row for row in rows}
    for core, framework, project in sorted(projects):
        row = rows_by_identity.get((core, framework))
        if row is None:
            results.append(
                Result(
                    "framework-capability-link",
                    f"{core}/{framework}",
                    False,
                    "scaffold has no matching capability row",
                )
            )
            continue

        # Run the declared native install before copying quality fixtures so
        # audit sees the lock state produced by this exact scaffold. Native
        # routes rewrite dependencies to an empty graph for this matrix.
        # (Cài theo owner native trước khi copy fixture quality để audit nhìn
        # đúng lock do scaffold tạo; lane native dùng graph rỗng có chủ đích.)
        install_result, install_succeeded, empty_dependency_manifest = run_native_install_contract(
            binary, row, project, env
        )
        if install_result is not None:
            results.append(install_result)
        empty_graph_installed = install_succeeded and empty_dependency_manifest is not None
        native_install_owned = row["dependency_ownership"]["install"]["owner"] == "mgc-native"

        # Each quality probe receives a fresh copy so neither synthetic
        # lifecycle edits nor optimizer output can contaminate another check.
        # (Mỗi probe quality dùng bản sao riêng để script giả và output
        # optimizer không làm nhiễm kết quả của chiều kiểm tra khác.)
        commands = (
            ("optimizer", ["optimizer", "--core", core]),
            ("cache", ["cache", "status", "--target", "all", "--core", core]),
            ("security-audit", ["audit", "--core", core, "--format", "json"]),
        )
        for category, args in commands:
            quality_project = (
                root / "quality-fixtures" / category / core / project.name
            )
            try:
                quality_project.parent.mkdir(parents=True, exist_ok=True)
                shutil.copytree(project, quality_project, symlinks=True)
                completed = run_command(
                    [str(binary), *args],
                    cwd=quality_project,
                    env=env,
                    timeout_seconds=90,
                )
                passed, evidence_status, detail = assess_dimension(
                    category,
                    completed,
                    empty_graph_installed=empty_graph_installed,
                    native_install_owned=native_install_owned,
                    optimizer_no_runtime_is_not_applicable=True,
                )
                results.append(
                    Result(
                        category,
                        f"{core}/{framework}",
                        passed,
                        detail,
                        completed.returncode,
                        evidence_status,
                    )
                )
            except (OSError, shutil.Error, subprocess.TimeoutExpired) as error:
                results.append(Result(category, f"{core}/{framework}", False, str(error)))

        results.extend(
            run_framework_language_runtime_probe(
                binary, core, framework, project, env
            )
        )
        results.extend(run_framework_script_contracts(binary, core, framework, project, env))
        results.extend(run_store_scope_contract(binary, row, project, env, install_succeeded))
        results.extend(
            run_lockfile_contracts(
                binary,
                row,
                project,
                env,
                install_succeeded,
            )
        )

    results.extend(run_shared_store_check(binary, root, env))
    return results

__all__ = ['run_project_dimensions']
