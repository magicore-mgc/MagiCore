"""Repository and source-revision evidence."""

from __future__ import annotations

from ..paths.paths import (
    REPOSITORY_ROOT,
)
from ..process.process import (
    run_text_capture,
)
import platform

def lifecycle_platform_name() -> str:
    """Return the host OS name recorded in generated lifecycle evidence."""
    return platform.system()

def lifecycle_working_tree_clean() -> bool:
    """Return true only when Git can prove there are no tracked or untracked edits.
    Chỉ trả true khi Git chứng minh không có thay đổi tracked hoặc untracked.
    """
    try:
        result = run_text_capture(
            ["git", "status", "--porcelain", "--untracked-files=all"],
            capture_output=True,
            text=True,
            cwd=REPOSITORY_ROOT,
            check=False,
        )
    except OSError:
        return False
    return result.returncode == 0 and not result.stdout.strip()

def lifecycle_source_matches(commit: str, clean_before_run: bool) -> bool:
    """Require clean source both before and after collection at one commit.
    Bắt buộc source sạch trước/sau khi thu thập và giữ cùng một commit.
    """
    if not clean_before_run or not commit or not lifecycle_working_tree_clean():
        return False
    try:
        result = run_text_capture(
            ["git", "rev-parse", "HEAD"],
            capture_output=True,
            text=True,
            cwd=REPOSITORY_ROOT,
            check=False,
        )
    except OSError:
        return False
    return result.returncode == 0 and result.stdout.strip() == commit

__all__ = ['lifecycle_platform_name', 'lifecycle_source_matches', 'lifecycle_working_tree_clean']
