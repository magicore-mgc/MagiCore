"""Owns the matrix responsibility in the stress-matrix runner.
Sở hữu trách nhiệm matrix trong stress-matrix runner.
"""

from __future__ import annotations

from pathlib import Path

from ....constants.language_catalog.language_catalog import RUNTIME_PROBE_LANGUAGE_ORDER

from ....framework_matrix.scaffold_language_files.scaffold_language_files import scaffold_language_files

from ....result.result import Result

from ..runner.runner import _run_framework_language_probe_for_language

def run_framework_language_runtime_probe(
    binary: Path,
    core: str,
    framework: str,
    project: Path,
    env: dict[str, str],
) -> list[Result]:
    """Run each standalone source-language probe through mgc run.
    """
    subject = f"{core}/{framework}"
    language_files, _ = scaffold_language_files(project, core)
    languages = [
        candidate
        for candidate in RUNTIME_PROBE_LANGUAGE_ORDER
        if language_files.get(candidate, 0) > 0
    ]
    if not languages:
        return [
            Result(
                "framework-language-runtime-probe",
                subject,
                True,
                "scaffold emits only non-standalone source formats; framework runtime remains unverified",
                evidence_status="not_applicable",
            )
        ]

    results: list[Result] = []
    for language in languages:
        results.append(
            _run_framework_language_probe_for_language(
                binary, core, framework, project, env, language
            )
        )
    return results

__all__ = ['run_framework_language_runtime_probe']
