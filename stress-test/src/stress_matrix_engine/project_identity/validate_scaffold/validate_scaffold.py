"""Owns the validate scaffold responsibility in the stress-matrix runner.
Sở hữu trách nhiệm validate scaffold trong stress-matrix runner.
"""

from __future__ import annotations

from pathlib import Path

from ...constants.framework_catalog.framework_catalog import IOT_BOARD_FRAMEWORKS, WEB_FRAMEWORK_ALIASES

from ..has_payload.has_payload import has_payload

from ..read_project_identity.read_project_identity import read_project_identity

def validate_scaffold(project: Path, core: str, framework: str) -> str | None:
    """Check identity and generated project payload, not just command success.
    Kiểm tra identity và file project sinh ra, không chỉ exit code.
    """
    if not project.is_dir():
        return "project directory was not created"
    marker = project / ".mgc.core"
    if not marker.is_file() or marker.read_text(encoding="utf-8").strip() != core:
        return "missing or incorrect .mgc.core marker"
    config_path = project / "mgc.toml"
    if not config_path.is_file():
        return "mgc.toml was not created"
    try:
        config = read_project_identity(config_path)
    except (OSError, ValueError) as error:
        return f"mgc.toml identity is invalid: {error}"
    if config.get("ecosystem") != core and not (core == "clo" and config.get("ecosystem") == "cloud"):
        return f"mgc.toml ecosystem is {config.get('ecosystem')!r}, expected {core!r}"
    frameworks = config.get("frameworks")
    identity_matches = isinstance(frameworks, list) and framework in frameworks
    if core == "web" and isinstance(frameworks, list) and not identity_matches:
        canonical = WEB_FRAMEWORK_ALIASES.get(framework)
        if canonical is not None:
            identity_matches = canonical in frameworks
        elif framework == "monorepo":
            identity_matches = (
                config.get("mode") == "monorepo"
                and len(frameworks) >= 2
                and "react-vite" in frameworks
            )
    if core == "iot" and framework in IOT_BOARD_FRAMEWORKS:
        identity_matches = (
            config.get("iot_framework") == IOT_BOARD_FRAMEWORKS[framework]
            and config.get("iot_board") == framework
        )
    if not identity_matches:
        return f"mgc.toml framework identity does not include {framework!r}: {frameworks!r}"
    if not has_payload(project, core):
        return "scaffold contains no non-empty source file or CI/CD definition"
    return None

__all__ = ['validate_scaffold']
