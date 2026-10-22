"""Owns orchestration of one framework language runtime probe.
Sở hữu điều phối một framework language runtime probe.
"""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

from ....result.result import Result
from ...profile.language_runtime_profile.language_runtime_profile import language_runtime_profile
from .eligibility.eligibility import probe_eligibility
from .execution.execution import execute_language_probe
from .language_setup.language_setup import prepare_language_files
from .toolchain.toolchain import (
    adjust_compiler_commands,
    check_probe_executable,
    qualify_go_web_toolchain,
)
from .workspace.workspace import create_probe_workspace

def _run_framework_language_probe_for_language(
    binary: Path, core: str, framework: str, project: Path,
    env: dict[str, str], language: str,
) -> Result:
    """Run one isolated executable-language probe through mgc run.
    Chạy một probe ngôn ngữ độc lập, cách ly, qua mgc run.
    """
    subject = f"{core}/{framework}/{language}"
    ineligible = probe_eligibility(subject, language)
    if ineligible is not None:
        return ineligible
    profile = language_runtime_profile(language)
    if profile is None:
        return Result(
            "framework-language-runtime-probe", subject, True,
            f"no standalone runtime probe is defined for {language}; framework runtime remains unverified",
            evidence_status="unverified",
        )
    executable, filename, source, commands = profile
    executable, commands = adjust_compiler_commands(language, executable, commands, env)
    unavailable = check_probe_executable(subject, language, executable, project, env)
    if unavailable is not None:
        return unavailable

    marker = f"MGC_STRESS_LANGUAGE_RUNTIME:{language}"
    try:
        probe_project, workspace_error = create_probe_workspace(
            project, env, core, framework, language
        )
        if workspace_error is not None:
            return workspace_error
        probe_env, commands = prepare_language_files(
            core, language, probe_project, filename, source, commands, env, marker
        )
        commands, toolchain_error = qualify_go_web_toolchain(
            binary, subject, core, language, probe_project, probe_env, commands
        )
        if toolchain_error is not None:
            return toolchain_error
        return execute_language_probe(
            binary, subject, core, language, probe_project, probe_env, commands, marker
        )
    except (OSError, shutil.Error, subprocess.TimeoutExpired, UnicodeError) as error:
        return Result(
            "framework-language-runtime-probe", subject, False,
            f"{language} runtime probe failed: {error}",
            evidence_status="unverified",
        )

__all__ = ["_run_framework_language_probe_for_language"]
