"""Owns the source provenance responsibility in the stress-matrix runner.
Sở hữu trách nhiệm source provenance trong stress-matrix runner.
"""

from __future__ import annotations

import hashlib

import re

import subprocess

from pathlib import Path

from typing import Any

from ...constants.evidence_categories.evidence_categories import SOURCE_PROVENANCE_TIMEOUT_SECONDS

def collect_source_provenance(binary: Path, repo_root: Path | None = None) -> dict[str, Any]:
    """Bind stress evidence to the source revision and exact CLI binary.
    Gắn stress evidence với revision source và chính xác binary CLI.
    """
    source_root = repo_root or Path(__file__).resolve().parents[2]
    digest = hashlib.sha256()
    try:
        with binary.open("rb") as executable:
            while chunk := executable.read(1024 * 1024):
                digest.update(chunk)
    except OSError:
        binary_sha256 = None
    else:
        binary_sha256 = digest.hexdigest()

    try:
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=source_root,
            text=True,
            encoding="utf-8",
            errors="replace",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=SOURCE_PROVENANCE_TIMEOUT_SECONDS,
            check=False,
        )
        status = subprocess.run(
            ["git", "status", "--porcelain=v1"],
            cwd=source_root,
            text=True,
            encoding="utf-8",
            errors="replace",
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=SOURCE_PROVENANCE_TIMEOUT_SECONDS,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        git_commit = None
        working_tree_clean = None
    else:
        commit_text = revision.stdout.strip()
        git_commit = (
            commit_text
            if revision.returncode == 0 and re.fullmatch(r"[0-9a-fA-F]{40,64}", commit_text)
            else None
        )
        working_tree_clean = (
            not status.stdout.strip()
            if status.returncode == 0
            else None
        )

    return {
        "git_commit": git_commit,
        "working_tree_clean": working_tree_clean,
        "binary_sha256": binary_sha256,
    }

__all__ = ['collect_source_provenance']
