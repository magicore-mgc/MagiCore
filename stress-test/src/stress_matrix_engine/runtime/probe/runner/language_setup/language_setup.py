"""Owns language-specific runtime probe files and environment.
Sở hữu file và môi trường riêng cho từng language runtime probe.
"""

from __future__ import annotations

import shlex
from pathlib import Path


def prepare_language_files(
    core: str, language: str, probe_project: Path, filename: str,
    source: str, commands: tuple[str, ...], env: dict[str, str], marker: str,
) -> tuple[dict[str, str], tuple[str, ...]]:
    """Write the isolated probe fixture and return its child environment.
    Ghi fixture probe cách ly và trả môi trường tiến trình con.
    """
    if language in {"TypeScript", "JavaScript"}:
        (probe_project / "package.json").write_text(
            '{"name":"mgc-stress-language-probe","dependencies":{},"devDependencies":{}}\n',
            encoding="utf-8",
        )
    elif language == "Python":
        (probe_project / "pyproject.toml").write_text(
            '[project]\nname = "mgc-stress-language-probe"\n'
            'version = "0.0.0"\nrequires-python = ">=3.9"\n'
            "dependencies = []\n",
            encoding="utf-8",
        )
    elif language == "Go":
        (probe_project / "go.mod").write_text(
            "module example.com/mgc-stress-language-probe\n\ngo 1.27.0\n",
            encoding="utf-8",
        )
    elif language == "Dart":
        (probe_project / "pubspec.yaml").write_text(
            "name: mgc_stress_language_probe\nversion: 0.0.0\n",
            encoding="utf-8",
        )
    elif language == "Swift":
        (probe_project / "Package.swift").write_text(
            "// swift-tools-version: 5.9\n",
            encoding="utf-8",
        )

    probe_path = probe_project / filename
    probe_path.parent.mkdir(parents=True, exist_ok=True)
    probe_path.write_text(source, encoding="utf-8")
    if language in {"C", "C++", "Objective-C", "Objective-C++"}:
        compiler_temp = probe_project / ".mgc-compiler-tmp"
        compiler_temp.mkdir(parents=True, exist_ok=True)
        commands = (
            f"TMPDIR={shlex.quote(str(compiler_temp))} {commands[0]}",
            *commands[1:],
        )
    if language == "GDScript":
        (probe_project / ".mgc-stress-godot/project.godot").write_text(
            "config_version=5\n\n[application]\nconfig/name=\"MGC Stress Probe\"\n",
            encoding="utf-8",
        )
    if language == "Rust":
        rust_source = probe_path.parent / "src" / "main.rs"
        rust_source.parent.mkdir(parents=True, exist_ok=True)
        rust_source.write_text(
            f'fn main() {{ println!("{marker}"); }}\n', encoding="utf-8"
        )
        (probe_path.parent / "Cargo.lock").write_text(
            'version = 4\n\n[[package]]\n'
            'name = "mgc-stress-runtime-probe"\nversion = "0.0.0"\n',
            encoding="utf-8",
        )
    if language in {"Java", "Kotlin"}:
        (probe_project / ".mgc-java-tmp").mkdir(parents=True, exist_ok=True)

    probe_env = env.copy()
    if language == "Go":
        # Pin the build cache to this isolated fixture, never the user cache.
        # Giữ build cache trong fixture cách ly, không dùng cache của người dùng.
        go_cache = probe_project / ".mgc-go-build-cache"
        go_cache.mkdir(parents=True, exist_ok=True)
        commands = tuple(
            f"GOCACHE={shlex.quote(str(go_cache))} {command}"
            for command in commands
        )
        probe_env.update(
            {"GOPROXY": "off", "GOSUMDB": "off", "GOTOOLCHAIN": "local"}
        )
    if language == "Swift":
        swift_cache = probe_project / ".mgc-swift-module-cache"
        swift_sdk_cache = probe_project / ".mgc-swift-sdk-module-cache"
        swift_cache.mkdir(parents=True, exist_ok=True)
        swift_sdk_cache.mkdir(parents=True, exist_ok=True)
        probe_env["CLANG_MODULE_CACHE_PATH"] = str(swift_cache)
        probe_env["SWIFT_MODULECACHE_PATH"] = str(swift_cache)
    return probe_env, commands

__all__ = ["prepare_language_files"]
