"""Recovery-probe environment setup."""

from __future__ import annotations

from .shared import (
    lifecycle_environment,
)
import os

def recovery_environment(sandbox: str, project_path: str) -> dict[str, str]:
    """Give crash-recovery probes a private store below their project.
    (Cấp store riêng bên dưới project cho probe phục hồi crash.)"""
    env = lifecycle_environment(sandbox, project_path)
    recovery_root = os.path.join(project_path, ".magicore-recovery")
    recovery_home = os.path.join(recovery_root, "home")
    env["HOME"] = recovery_home
    env["USERPROFILE"] = recovery_home
    env["MGC_CACHE_DIR"] = recovery_root
    return env

__all__ = ['recovery_environment']
