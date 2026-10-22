"""Owns the has payload responsibility in the stress-matrix runner.
Sở hữu trách nhiệm has payload trong stress-matrix runner.
"""

from __future__ import annotations

import json

from pathlib import Path

from ...constants.language_catalog.language_catalog import AUTOMATION_PATHS, SOURCE_SUFFIXES

def has_payload(project: Path, core: str) -> bool:
    """Require source code, or an actual CI/CD definition, beyond MGC metadata.
    Bắt buộc có source code hoặc pipeline CI/CD thực ngoài metadata MGC.
    """
    for path in project.rglob("*"):
        relative_parts = path.relative_to(project).parts
        if not path.is_file() or any(
            part in {".git", ".magicore"} or part.startswith(".mgc")
            for part in relative_parts
        ):
            continue
        if path.name in {"mgc.toml", ".mgc.core", ".mgc-provenance.json"}:
            continue
        relative = path.relative_to(project).as_posix().lower()
        is_cicd_definition = core == "cicd" and (
            relative in AUTOMATION_PATHS
            or (relative.startswith(".github/workflows/") and path.suffix.lower() in {".yaml", ".yml"})
        )
        if core == "hardware" and path.name in {"bench.json", "optimizer.json"}:
            try:
                descriptor = json.loads(path.read_text(encoding="utf-8"))
            except (OSError, UnicodeError, json.JSONDecodeError):
                continue
            expected_kind = path.stem
            if (
                isinstance(descriptor, dict)
                and descriptor.get("kind") == expected_kind
                and isinstance(descriptor.get("name"), str)
                and descriptor["name"].strip()
            ):
                return True
        if path.suffix.lower() in SOURCE_SUFFIXES or is_cicd_definition:
            try:
                if path.stat().st_size > 0:
                    return True
            except OSError:
                continue
    return False

__all__ = ['has_payload']
