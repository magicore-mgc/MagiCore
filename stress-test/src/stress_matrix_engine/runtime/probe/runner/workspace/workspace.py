"""Owns isolated language probe workspace creation.
Sở hữu việc tạo workspace cách ly cho language probe.
"""

from __future__ import annotations

import shutil
from pathlib import Path

from .....result.result import Result

def create_probe_workspace(
    project: Path, env: dict[str, str], core: str, framework: str, language: str
) -> tuple[Path, Result | None]:
    """Create the fixture directory and copy its MagiCore identity files.
    Tạo fixture và sao chép các file nhận diện dự án MagiCore.
    """
    fixture_root = Path(env.get("MGC_DATA_DIR", project.parent))
    language_key = language.lower().replace("+", "p").replace("#", "sharp")
    probe_project = fixture_root / "language-probes" / core / framework / language_key
    probe_project.mkdir(parents=True, exist_ok=True)
    for config_name in ("mgc.toml", ".mgc.core"):
        source_config = project / config_name
        if source_config.is_file():
            shutil.copy2(source_config, probe_project / config_name)
    if not (probe_project / "mgc.toml").is_file():
        return probe_project, Result(
            "framework-language-runtime-probe",
            f"{core}/{framework}/{language}",
            False,
            "generated scaffold has no mgc.toml to identify the runtime probe project",
            evidence_status="unverified",
        )
    return probe_project, None

__all__ = ["create_probe_workspace"]
