"""Swift toolchain environment setup."""

from __future__ import annotations

import os

def swift_toolchain_environment(
    lane: dict,
    step: str,
    lane_env: dict[str, str],
    windows=None,
) -> dict[str, str]:
    """Use Swift's pinned Windows Python only for Swift lifecycle commands.
    (Chỉ dùng Python Windows đã ghim của Swift cho các lệnh Swift.)"""
    if windows is None:
        windows = os.name == "nt"
    if (
        not windows
        or (lane.get("core"), lane.get("language")) != ("app", "swift")
        or step not in {"install", "test", "build"}
    ):
        return lane_env

    python_bin = os.environ.get("MGC_SWIFT_PYTHON_BIN", "")
    if not python_bin:
        if os.environ.get("GITHUB_ACTIONS", "").lower() == "true":
            raise RuntimeError("Windows Swift lifecycle requires the Python 3.10 path")
        return lane_env
    if not os.path.isfile(os.path.join(python_bin, "python.exe")):
        raise RuntimeError("MGC_SWIFT_PYTHON_BIN does not contain python.exe")

    step_env = lane_env.copy()
    step_env["PATH"] = python_bin + os.pathsep + step_env.get("PATH", "")
    return step_env

__all__ = ['swift_toolchain_environment']
