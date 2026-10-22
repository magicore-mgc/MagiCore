"""Owns executable qualification and explicit build tool preparation.
Sở hữu xác minh executable và chuẩn bị toolchain build rõ ràng.
"""

from __future__ import annotations

import os
import re
import shlex
import shutil
import sys
from pathlib import Path

from .....process.run_command.run_command import run_command
from .....result.result import Result

def adjust_compiler_commands(
    language: str, executable: str, commands: tuple[str, ...], env: dict[str, str]
) -> tuple[str, tuple[str, ...]]:
    """Carry the declared Apple compiler and SDK through the clean child env.
    Truyền compiler và SDK Apple đã khai báo qua môi trường con sạch.
    """
    if sys.platform == "darwin" and language in {"C", "C++", "Objective-C", "Objective-C++"}:
        compiler_key = "CXX" if language in {"C++", "Objective-C++"} else "CC"
        configured_compiler = env.get(compiler_key)
        if configured_compiler:
            commands = (shlex.quote(configured_compiler) + commands[0][len(executable):], *commands[1:])
            executable = configured_compiler
        sdk_root = env.get("SDKROOT")
        if sdk_root:
            commands = (f"SDKROOT={shlex.quote(sdk_root)} {commands[0]}", *commands[1:])
    return executable, commands

def check_probe_executable(
    subject: str, language: str, executable: str, project: Path, env: dict[str, str]
) -> Result | None:
    """Check the selected runtime and the Java version required by JVM probes.
    Kiểm tra runtime và phiên bản Java cần cho probe JVM.
    """
    search_path = env.get("PATH") or env.get("Path")
    if shutil.which(executable, path=search_path) is None:
        return Result(
            "framework-language-runtime-probe", subject, True,
            f"{language} runtime executable {executable!r} is missing; framework runtime remains unverified",
            evidence_status="unverified",
        )
    if language not in {"Java", "Kotlin"}:
        return None
    java = run_command(["java", "-version"], cwd=project, env=env, timeout_seconds=30)
    version_match = re.search(r'(?i)(?:version\s+)?"?(\d+)(?:\.(\d+))?', java.stdout)
    if java.returncode != 0 or version_match is None:
        return Result(
            "framework-language-runtime-probe", subject, True,
            f"{language} tool is present but no usable Java runtime was detected; framework runtime remains unverified: {java.stdout[-600:]}",
            java.returncode, "unverified",
        )
    major = int(version_match.group(1))
    if major == 1 and version_match.group(2):
        major = int(version_match.group(2))
    minimum = 11 if language == "Java" else 8
    if major < minimum:
        return Result(
            "framework-language-runtime-probe", subject, True,
            f"{language} probe requires Java {minimum}+; detected Java {major}; framework runtime remains unverified",
            evidence_status="unverified",
        )
    return None

def qualify_go_web_toolchain(
    binary: Path, subject: str, core: str, language: str, probe_project: Path,
    probe_env: dict[str, str], commands: tuple[str, ...],
) -> tuple[tuple[str, ...], Result | None]:
    """Install/build the Go web fixture through the MagiCore product path.
    Cài/build fixture Go web qua luồng sản phẩm MagiCore.
    """
    if language != "Go" or core != "web":
        return commands, None
    install = run_command(
        [str(binary), "install-web"], cwd=probe_project, env=probe_env, timeout_seconds=120
    )
    if install.returncode != 0:
        return commands, Result(
            "framework-language-runtime-probe", subject, False,
            f"MGC Go Web install failed: {install.stdout[-2200:]}",
            install.returncode, "unverified",
        )
    build = run_command(
        [str(binary), "build", "--core", "web"],
        cwd=probe_project, env=probe_env, timeout_seconds=120,
    )
    if build.returncode != 0:
        return commands, Result(
            "framework-language-runtime-probe", subject, False,
            f"MGC Go Web build failed: {build.stdout[-2200:]}",
            build.returncode, "unverified",
        )
    output_name = "server.exe" if os.name == "nt" else "server"
    executable_path = f"./.magicore/build/go/{output_name}"
    return (f"bash -c {shlex.quote(executable_path)}",), None

__all__ = ["adjust_compiler_commands", "check_probe_executable", "qualify_go_web_toolchain"]
