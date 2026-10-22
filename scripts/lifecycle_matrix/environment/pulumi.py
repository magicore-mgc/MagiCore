"""Pulumi local backend environment."""

from __future__ import annotations

from pathlib import Path
import os
import secrets

PULUMI_LIFECYCLE_STACK = "lifecycle"

def pulumi_local_environment(sandbox: str, environment: dict[str, str]) -> dict[str, str]:
    """Isolate Pulumi state locally and remove inherited cloud credentials.
    (Cô lập state Pulumi trong sandbox và bỏ credential cloud từ host.)"""
    env = environment.copy()
    backend_root = os.path.join(sandbox, ".pulumi-backend")
    pulumi_home = os.path.join(sandbox, ".pulumi-home")
    os.makedirs(backend_root, exist_ok=True)
    os.makedirs(pulumi_home, exist_ok=True)
    env["PULUMI_BACKEND_URL"] = Path(backend_root).as_uri()
    env["PULUMI_HOME"] = pulumi_home
    # Select the stack explicitly for child processes; workspace selection can
    # be keyed differently after Windows path normalization.
    # Chỉ định stack cho tiến trình con; Windows có thể chuẩn hóa path khác
    # khiến lựa chọn lưu trong workspace không còn khớp project.
    env["PULUMI_STACK"] = PULUMI_LIFECYCLE_STACK
    env["PULUMI_CONFIG_PASSPHRASE"] = secrets.token_urlsafe(32)
    env["PULUMI_SKIP_UPDATE_CHECK"] = "true"
    for name in tuple(env):
        if name == "PULUMI_ACCESS_TOKEN" or name.startswith(
            ("AWS_", "AZURE_", "ARM_", "GOOGLE_", "CLOUDSDK_")
        ):
            env.pop(name, None)
    env["AWS_EC2_METADATA_DISABLED"] = "true"
    return env

__all__ = ['PULUMI_LIFECYCLE_STACK', 'pulumi_local_environment']
