"""MGC-mediated Python test/build-tool provisioning."""

from __future__ import annotations

from ..process.process import (
    run_text_capture,
)
import subprocess

def mgc_add_python_test_build_tools(
    mgc_bin: str, project_path: str, environment: dict[str, str], timeout_s: int
) -> tuple[subprocess.CompletedProcess, dict[str, str]]:
    """Declare pinned Python test/build packages through the MGC CLI.
    (Khai báo package test/build Python đã pin qua CLI MGC.)"""
    result = subprocess.CompletedProcess(args=[mgc_bin, "add"], returncode=0, stdout="", stderr="")
    for package in ("pytest@9.1.1", "build@1.6.1", "setuptools@84.0.0"):
        result = run_text_capture(
            [mgc_bin, "add", package],
            capture_output=True,
            text=True,
            cwd=project_path,
            env=environment,
            timeout=timeout_s,
        )
        if result.returncode != 0:
            return result, environment
    return result, environment

__all__ = ['mgc_add_python_test_build_tools']
