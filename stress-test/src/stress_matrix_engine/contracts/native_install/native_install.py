"""Owns the native install responsibility in the stress-matrix runner.
Sở hữu trách nhiệm native install trong stress-matrix runner.
"""

from __future__ import annotations

import json

from pathlib import Path

from typing import Any

from ...manifest.set_lock_schema.set_lock_schema import set_lock_schema

from ...manifest.smoke_manifest.smoke_manifest import prepare_empty_dependency_manifest

from ...process.run_command.run_command import run_command

from ...result.result import Result

def run_native_install_contract(
    binary: Path,
    row: dict[str, Any],
    project: Path,
    env: dict[str, str],
) -> tuple[Result | None, bool, str | None]:
    """Run install only where capability data assigns MGC native ownership.
    Chỉ chạy install ở lane mà capability giao quyền cho MGC native.
    """
    core = row["core"]
    framework = row["framework"]
    subject = f"{core}/{framework}"
    owner = row["dependency_ownership"]["install"]["owner"]
    if owner != "mgc-native":
        return (
            Result(
                "framework-install-ownership",
                subject,
                True,
                f"install is declared {owner}; native execution not claimed",
                evidence_status="unsupported",
            ),
            False,
            None,
        )

    try:
        manifest = prepare_empty_dependency_manifest(project, core, framework)
        set_lock_schema(project / "mgc.toml", "3")
        completed = run_command(
            [str(binary), f"install-{core}"],
            cwd=project,
            env=env,
            timeout_seconds=120,
        )
        passed = completed.returncode == 0 and "All dependencies installed" in completed.stdout
        detail = (
            f"native install completed with an empty {manifest} dependency set"
            if passed
            else completed.stdout[-1400:]
        )
        return (
            Result("mgc-install-empty-graph", subject, passed, detail, completed.returncode),
            passed,
            manifest,
        )
    except (OSError, UnicodeError, ValueError, json.JSONDecodeError) as error:
        return Result("mgc-install-empty-graph", subject, False, str(error)), False, None

__all__ = ['run_native_install_contract']
