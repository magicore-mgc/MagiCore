"""Owns the run command responsibility in the stress-matrix runner.
Sở hữu trách nhiệm run command trong stress-matrix runner.
"""

from __future__ import annotations

import subprocess

from pathlib import Path

def run_command(
    argv: list[str],
    *,
    cwd: Path,
    env: dict[str, str],
    timeout_seconds: int,
    input_text: str = "",
) -> subprocess.CompletedProcess[str]:
    """Run one command without masking the child's exit code.
    Chạy một lệnh và giữ nguyên exit code của tiến trình con.
    """
    return subprocess.run(
        argv,
        cwd=cwd,
        env=env,
        input=input_text,
        text=True,
        encoding="utf-8",
        errors="replace",
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=timeout_seconds,
        check=False,
    )

__all__ = ['run_command']
