"""Owns the scaffold language files responsibility in the stress-matrix runner.
Sở hữu trách nhiệm scaffold language files trong stress-matrix runner.
"""

from __future__ import annotations

from collections import Counter

from pathlib import Path

from ...constants.language_catalog.language_catalog import AUTOMATION_PATHS, SOURCE_LANGUAGE_BY_SUFFIX

def scaffold_language_files(project: Path, core: str) -> tuple[dict[str, int], dict[str, int]]:
    """Inventory source files emitted by one scaffold, without executing them.
    Lập danh mục file nguồn do scaffold tạo, không thực thi code.
    """
    language_counts: Counter[str] = Counter()
    suffix_counts: Counter[str] = Counter()
    ignored_directories = {".git", ".magicore", ".mgc", "node_modules", "target", "dist"}
    for path in project.rglob("*"):
        relative_parts = path.relative_to(project).parts
        if not path.is_file() or any(
            part in ignored_directories or part.startswith(".mgc")
            for part in relative_parts
        ):
            continue
        relative = path.relative_to(project).as_posix().lower()
        suffix = path.suffix.lower()
        language = SOURCE_LANGUAGE_BY_SUFFIX.get(suffix)
        if core == "cicd" and (
            relative in AUTOMATION_PATHS
            or (relative.startswith(".github/workflows/") and suffix in {".yaml", ".yml"})
        ):
            language = "CI workflow YAML"
        elif core == "hardware" and path.name in {"bench.json", "optimizer.json"}:
            language = "JSON hardware descriptor"
        if language is None:
            continue
        language_counts[language] += 1
        suffix_counts[suffix or "<no extension>"] += 1
    return dict(sorted(language_counts.items())), dict(sorted(suffix_counts.items()))

__all__ = ['scaffold_language_files']
