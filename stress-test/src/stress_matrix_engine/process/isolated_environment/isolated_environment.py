"""Owns the isolated environment responsibility in the stress-matrix runner.
Sở hữu trách nhiệm isolated environment trong stress-matrix runner.
"""

from __future__ import annotations

import os

from pathlib import Path

def isolated_environment(root: Path) -> dict[str, str]:
    """Keep all test writes inside the temporary matrix root.
    Giữ mọi dữ liệu test trong thư mục ma trận tạm.
    """
    home = root / "home"
    original_home = Path(os.environ.get("HOME") or Path.home()).expanduser()
    data = root / "system-data"
    cache = root / "cache"
    templates = root / "templates"
    temporary = root / "tmp"
    cargo_home = data / "cargo-home"
    go_cache = cache / "go-build"
    for directory in (
        home,
        data,
        cache,
        templates,
        temporary,
        cargo_home,
        go_cache,
        data / "go-mod-cache",
    ):
        directory.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    # The lifecycle sidecar is consumed by the parent stress runner only.
    # Sidecar chỉ dành cho runner cha, không được truyền vào script của project.
    env.pop("MGC_STRESS_LIFECYCLE_EVIDENCE", None)
    env.update(
        {
            "HOME": str(home),
            "USERPROFILE": str(home),
            # Rustup reads toolchain metadata but these probes do not mutate it.
            # (Rustup chỉ đọc metadata toolchain; các probe không sửa thư mục này.)
            "RUSTUP_HOME": os.environ.get("RUSTUP_HOME", str(original_home / ".rustup")),
            "MGC_DATA_DIR": str(data),
            "MGC_CACHE_DIR": str(cache),
            "MGC_TEMPLATES_DIR": str(templates),
            "MGC_STRESS_TEST": "1",
            "CI": "1",
            "TMPDIR": str(temporary),
            "TEMP": str(temporary),
            "TMP": str(temporary),
            "XDG_CACHE_HOME": str(cache),
            "CARGO_HOME": str(cargo_home),
            "CARGO_TARGET_DIR": str(cache / "cargo-target"),
            "GOCACHE": str(go_cache),
            "GOMODCACHE": str(data / "go-mod-cache"),
            "DOTNET_CLI_HOME": str(data / "dotnet-home"),
            "NUGET_PACKAGES": str(data / "nuget-packages"),
            "DOTNET_CLI_TELEMETRY_OPTOUT": "1",
            "DOTNET_SKIP_FIRST_TIME_EXPERIENCE": "1",
        }
    )
    return env

__all__ = ['isolated_environment']
