"""Owns the store scope responsibility in the stress-matrix runner.
Sở hữu trách nhiệm store scope trong stress-matrix runner.
"""

from __future__ import annotations

import subprocess

from pathlib import Path

from typing import Any

from ...evidence.json_object.json_object import _parse_json_object

from ...process.run_command.run_command import run_command

from ...result.result import Result
from .runtime_health.runtime_health import evaluate_store_runtime_health

def run_store_scope_contract(
    binary: Path,
    row: dict[str, Any],
    project: Path,
    env: dict[str, str],
    install_succeeded: bool,
) -> list[Result]:
    """Verify the declared MGC store route and its runtime health evidence.
    Xác minh store route đã khai báo và bằng chứng health lúc chạy.
    """
    core = row["core"]
    framework = row["framework"]
    subject = f"{core}/{framework}"
    install_owner = row["dependency_ownership"]["install"]["owner"]
    store_owner = row["dependency_ownership"]["store"]["owner"]
    if store_owner != "mgc-native":
        status = "not_applicable" if core == "web" and install_owner != "mgc-native" else "unsupported"
        detail = (
            "capability manifest declares no MGC-owned store route for this framework"
        )
        return [
            Result("shared-store-scope", subject, True, detail, evidence_status=status),
            Result("shared-store-runtime", subject, True, detail, evidence_status=status),
        ]
    if not install_succeeded:
        detail = "MGC-owned install did not complete; store health has no runtime evidence"
        return [
            Result("shared-store-scope", subject, False, detail, evidence_status="unverified"),
            Result("shared-store-runtime", subject, False, detail, evidence_status="unverified"),
        ]

    try:
        completed = run_command(
            [
                str(binary),
                "store",
                "doctor",
                "--core",
                core,
                "--framework",
                framework,
                "--json",
            ],
            cwd=project,
            env=env,
            timeout_seconds=45,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        detail = str(error)
        return [
            Result("shared-store-scope", subject, False, detail, evidence_status="unverified"),
            Result("shared-store-runtime", subject, False, detail, evidence_status="unverified"),
        ]

    payload = _parse_json_object(completed.stdout)
    reported_scope = payload.get("scope") if payload else None
    scope_verified = bool(
        payload
        and payload.get("core") == core
        and payload.get("framework") == framework
        and reported_scope in {
            "web-cas-and-generation-integrity",
            "managed-root-path-safety-only",
        }
    )
    scope_result = Result(
        "shared-store-scope",
        subject,
        scope_verified,
        (
            f"doctor selected {reported_scope} for {core}/{framework}"
            if scope_verified
            else completed.stdout[-1200:]
        ),
        completed.returncode,
        "verified" if scope_verified else "unverified",
    )

    runtime_check_passed, runtime_detail, runtime_evidence_status = evaluate_store_runtime_health(
        payload, reported_scope, scope_verified, completed.stdout
    )
    runtime_ok = completed.returncode == 0 and runtime_check_passed and scope_verified
    runtime_result = Result(
        "shared-store-runtime",
        subject,
        runtime_ok,
        runtime_detail if scope_verified else completed.stdout[-1200:],
        completed.returncode,
        runtime_evidence_status if scope_verified else "unverified",
    )
    return [scope_result, runtime_result]

__all__ = ["run_store_scope_contract"]
