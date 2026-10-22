"""Owns the smoke manifest responsibility in the stress-matrix runner.
Sở hữu trách nhiệm smoke manifest trong stress-matrix runner.
"""

from __future__ import annotations

import json

from pathlib import Path

def prepare_runtime_smoke_manifest(project: Path, core: str, framework: str) -> str | None:
    """Remove declared dependencies before testing command orchestration alone.
    Bỏ dependency khai báo để kiểm riêng orchestration của lệnh.
    """
    manifest_names = (
        "package.json",
        "Cargo.toml",
        "pyproject.toml",
        "go.mod",
        "pom.xml",
        "pubspec.yaml",
        "Package.swift",
    )
    has_manifest = any((project / name).is_file() for name in manifest_names)
    has_manifest = has_manifest or next(project.glob("*.csproj"), None) is not None
    if not has_manifest:
        return None
    return prepare_empty_dependency_manifest(project, core, framework)

def prepare_empty_dependency_manifest(project: Path, core: str, framework: str) -> str:
    """Keep framework identity while removing network dependencies for install smoke.
    Giữ identity framework nhưng bỏ dependency mạng cho smoke install.
    """
    package_json = project / "package.json"
    if package_json.is_file():
        package = json.loads(package_json.read_text(encoding="utf-8"))
        if not isinstance(package, dict):
            raise ValueError("package.json root is not an object")
        for field in ("dependencies", "devDependencies", "optionalDependencies", "peerDependencies"):
            package[field] = {}
        package_json.write_text(json.dumps(package, indent=2) + "\n", encoding="utf-8")
        return "package.json"

    cargo_toml = project / "Cargo.toml"
    if cargo_toml.is_file():
        name = project.name.replace("_", "-")
        cargo_toml.write_text(
            f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2021"\n',
            encoding="utf-8",
        )
        return "Cargo.toml"

    pyproject = project / "pyproject.toml"
    if pyproject.is_file():
        name = project.name.replace("_", "-")
        pyproject.write_text(
            f'[project]\nname = "{name}"\nversion = "0.1.0"\n'
            'requires-python = ">=3.9"\ndependencies = []\n',
            encoding="utf-8",
        )
        return "pyproject.toml"

    go_mod = project / "go.mod"
    if go_mod.is_file():
        go_mod.write_text(
            f"module example.com/{project.name.lower()}\n\ngo 1.21\n",
            encoding="utf-8",
        )
        return "go.mod"

    pom = project / "pom.xml"
    if pom.is_file():
        pom.write_text(
            "<project><modelVersion>4.0.0</modelVersion><groupId>local</groupId>"
            f"<artifactId>{project.name}</artifactId><version>0.1.0</version></project>\n",
            encoding="utf-8",
        )
        return "pom.xml"

    csproj = next(project.glob("*.csproj"), None)
    if csproj is not None:
        csproj.write_text(
            '<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup>'
            '<TargetFramework>net8.0</TargetFramework></PropertyGroup></Project>\n',
            encoding="utf-8",
        )
        return csproj.name

    pubspec = project / "pubspec.yaml"
    if pubspec.is_file():
        pubspec.write_text(
            f'name: {project.name.lower()}\nversion: 0.1.0\n'
            'environment:\n  sdk: ">=3.0.0 <4.0.0"\ndependencies: {}\n',
            encoding="utf-8",
        )
        return "pubspec.yaml"

    swift_package = project / "Package.swift"
    if swift_package.is_file():
        swift_package.write_text(
            '// swift-tools-version: 5.9\nimport PackageDescription\n'
            f'let package = Package(name: "{project.name}", products: [], dependencies: [], targets: [])\n',
            encoding="utf-8",
        )
        return "Package.swift"

    raise ValueError(f"no supported dependency manifest found for {core}/{framework}")

__all__ = ['prepare_runtime_smoke_manifest', 'prepare_empty_dependency_manifest']
