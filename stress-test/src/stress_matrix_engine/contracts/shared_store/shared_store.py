"""Owns the shared store responsibility in the stress-matrix runner.
Sở hữu trách nhiệm shared store trong stress-matrix runner.
"""

from __future__ import annotations

import subprocess

from pathlib import Path

from ...evidence.json_object.json_object import _parse_json_object

from ...process.run_command.run_command import run_command

from ...result.result import Result

def run_shared_store_check(binary: Path, root: Path, env: dict[str, str]) -> list[Result]:
    """Initialize and inspect an isolated empty web store without network dependencies.
    Khởi tạo và kiểm tra web store tách biệt, không cần tải dependency qua mạng.
    """
    project = root / "shared-store-project"
    project.mkdir(parents=True, exist_ok=True)
    (project / ".mgc.core").write_text("web\n", encoding="utf-8")
    (project / "mgc.toml").write_text(
        'name = "shared-store-stress"\nversion = "0.1.0"\necosystem = "web"\nframeworks = ["vanilla"]\n',
        encoding="utf-8",
    )
    (project / "package.json").write_text(
        '{"name":"shared-store-stress","version":"0.1.0","dependencies":{}}\n',
        encoding="utf-8",
    )
    try:
        install = run_command(
            [str(binary), "install-web"], cwd=project, env=env, timeout_seconds=90
        )
        if install.returncode != 0:
            return [Result("shared-store", "web", False, f"empty install failed: {install.stdout[-1200:]}", install.returncode)]
        completed = run_command(
            [str(binary), "store", "doctor", "--core", "web", "--json"],
            cwd=project,
            env=env,
            timeout_seconds=45,
        )
        payload = _parse_json_object(completed.stdout)
        healthy = bool(payload and payload.get("healthy") is True)
        passed = completed.returncode == 0 and healthy
        detail = "web shared store reports healthy after install" if passed else completed.stdout[-1200:]
        return [Result("shared-store", "web", passed, detail, completed.returncode)]
    except (OSError, subprocess.TimeoutExpired) as error:
        return [Result("shared-store", "web", False, str(error))]

__all__ = ['run_shared_store_check']
