"""Owns the lockfile responsibility in the stress-matrix runner.
Sở hữu trách nhiệm lockfile trong stress-matrix runner.
"""

from __future__ import annotations

import subprocess

from pathlib import Path

from typing import Any

from ...constants.core_inventory.core_inventory import V4_INSTALL_CORES

from ...manifest.lockfile_version.lockfile_version import lockfile_version

from ...manifest.set_lock_schema.set_lock_schema import set_lock_schema

from ...process.run_command.run_command import run_command

from ...result.result import Result

def run_lockfile_contracts(
    binary: Path,
    row: dict[str, Any],
    project: Path,
    env: dict[str, str],
    install_succeeded: bool,
) -> list[Result]:
    """Verify native lock replay per framework and V4 where its writer is wired.
    Kiểm tra replay lock native theo từng framework và V4 tại core đã nối writer.
    """
    core = row["core"]
    framework = row["framework"]
    subject = f"{core}/{framework}"
    owner = row["dependency_ownership"]["install"]["owner"]
    if owner != "mgc-native":
        return [
            Result(
                "mgc-lock-v3-empty-graph",
                subject,
                True,
                f"not applicable: install owner is {owner}",
                evidence_status="unsupported",
            ),
            Result(
                "mgc-lock-v4-empty-graph",
                subject,
                True,
                f"not applicable: install owner is {owner}",
                evidence_status="unsupported",
            ),
        ]

    lock_path = project / "mgc.lock"
    config_path = project / "mgc.toml"
    if not install_succeeded:
        v3 = Result(
            "mgc-lock-v3-empty-graph",
            subject,
            True,
            "not verified because the declared native install did not complete",
            evidence_status="unverified",
        )
    elif lockfile_version(lock_path) != 3:
        v3 = Result(
            "mgc-lock-v3-empty-graph",
            subject,
            False,
            f"native V3 install did not produce a version 3 mgc.lock (found {lockfile_version(lock_path)!r})",
        )
    else:
        original_v3 = lock_path.read_bytes()
        try:
            replay = run_command(
                [str(binary), f"install-{core}", "--frozen"],
                cwd=project,
                env=env,
                timeout_seconds=120,
            )
            replay_stable = lock_path.read_bytes() == original_v3
            passed = replay.returncode == 0 and replay_stable
            detail = (
                "empty-graph V3 frozen install replayed and preserved lock bytes"
                if passed
                else "empty-graph V3 frozen replay failed or changed lock bytes: " + replay.stdout[-1200:]
            )
            v3 = Result("mgc-lock-v3-empty-graph", subject, passed, detail, replay.returncode)
        except (OSError, subprocess.TimeoutExpired) as error:
            v3 = Result("mgc-lock-v3-empty-graph", subject, False, str(error))

    if core not in V4_INSTALL_CORES:
        v4 = Result(
            "mgc-lock-v4-empty-graph",
            subject,
            True,
            f"unsupported: no native V4 install writer is wired for the {core} core",
            evidence_status="unsupported",
        )
        return [v3, v4]
    if not install_succeeded:
        return [
            v3,
            Result(
                "mgc-lock-v4-empty-graph",
                subject,
                True,
                "not verified because the declared native install did not complete",
                evidence_status="unverified",
            ),
        ]

    try:
        set_lock_schema(config_path, "v4")
        write = run_command(
            [str(binary), f"install-{core}"],
            cwd=project,
            env=env,
            timeout_seconds=120,
        )
        version = lockfile_version(lock_path)
        if write.returncode != 0 or version != 4:
            v4 = Result(
                "mgc-lock-v4-empty-graph",
                subject,
                False,
                f"V4 install exited {write.returncode}, lock version is {version!r}: {write.stdout[-1200:]}",
                write.returncode,
            )
        else:
            original_v4 = lock_path.read_bytes()
            replay = run_command(
                [str(binary), f"install-{core}", "--frozen"],
                cwd=project,
                env=env,
                timeout_seconds=120,
            )
            replay_stable = lock_path.read_bytes() == original_v4
            passed = replay.returncode == 0 and replay_stable and lockfile_version(lock_path) == 4
            detail = (
                "empty-graph V4 frozen install replayed and preserved lock bytes"
                if passed
                else "empty-graph V4 frozen replay failed or changed lock bytes: " + replay.stdout[-1200:]
            )
            v4 = Result("mgc-lock-v4-empty-graph", subject, passed, detail, replay.returncode)
    except (OSError, UnicodeError, ValueError, subprocess.TimeoutExpired) as error:
        v4 = Result("mgc-lock-v4-empty-graph", subject, False, str(error))
    return [v3, v4]

__all__ = ['run_lockfile_contracts']
