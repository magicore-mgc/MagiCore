"""Owns the create frameworks responsibility in the stress-matrix runner.
Sở hữu trách nhiệm create frameworks trong stress-matrix runner.
"""

from __future__ import annotations

import json

import subprocess

from pathlib import Path

from typing import Any

from ..scaffold_language_files.scaffold_language_files import scaffold_language_files

from ...process.run_command.run_command import run_command

from ...project_identity.project_slug.project_slug import project_slug

from ...project_identity.validate_scaffold.validate_scaffold import validate_scaffold

from ...result.result import Result

def create_frameworks(
    binary: Path,
    rows: list[dict[str, Any]],
    root: Path,
    env: dict[str, str],
    timeout_seconds: int,
) -> tuple[list[Result], list[tuple[str, str, Path]]]:
    """Create every advertised framework in its own isolated directory.
    Tạo mọi framework được công bố trong thư mục cách ly riêng.
    """
    results: list[Result] = []
    successful_projects: list[tuple[str, str, Path]] = []
    for row in rows:
        core = row["core"]
        framework = row["framework"]
        project_name = project_slug(core, framework)
        core_root = root / "projects" / core
        core_root.mkdir(parents=True, exist_ok=True)
        argv = [str(binary), f"create-{core}", framework, project_name]
        if core == "web":
            argv.append("--yes")
        try:
            completed = run_command(
                argv,
                cwd=core_root,
                env=env,
                timeout_seconds=timeout_seconds,
            )
            project = core_root / project_name
            detail = validate_scaffold(project, core, framework) if completed.returncode == 0 else None
            passed = completed.returncode == 0 and detail is None
            subject = f"{core}/{framework}"
            if passed:
                successful_projects.append((core, framework, project))
                summary = f"created {project_name} ({row['status']})"
                language_files, suffix_counts = scaffold_language_files(project, core)
                language_detail = json.dumps(
                    {
                        "languages_or_source_formats": sorted(language_files),
                        "source_files_by_language_or_format": language_files,
                        "source_suffix_counts": suffix_counts,
                        "runtime_verified": False,
                    },
                    sort_keys=True,
                )
                language_passed = bool(language_files)
                results.append(
                    Result(
                        "framework-language-scaffold",
                        subject,
                        language_passed,
                        language_detail
                        if language_passed
                        else "scaffold passed but no recognized source-language file was found",
                        completed.returncode,
                        "verified" if language_passed else "unverified",
                    )
                )
            else:
                summary = detail or f"create command exited {completed.returncode}: {completed.stdout[-1400:]}"
            results.append(Result("framework-scaffold", subject, passed, summary, completed.returncode))
        except subprocess.TimeoutExpired as error:
            results.append(Result("framework-scaffold", f"{core}/{framework}", False, f"timed out after {timeout_seconds}s: {error}"))
        except (OSError, UnicodeError) as error:
            results.append(Result("framework-scaffold", f"{core}/{framework}", False, str(error)))
    return results, successful_projects

__all__ = ['create_frameworks']
