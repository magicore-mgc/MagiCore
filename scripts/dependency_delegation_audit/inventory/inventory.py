"""Inventory implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
from ..paths.paths import REPO_ROOT
from ..python.python import scan_python_text
from ..rules.common import SCAN_ROOTS
from ..rules.python import PYTHON_SCAN_ROOTS
from ..rules.shell import SHELL_SCAN_ROOTS
from ..shell.shell import scan_shell_text
import os

def _iter_rust_files() -> list:
    """Every .rs file under the scan roots (absolute, sorted).
    (Mọi file .rs dưới scan roots (tuyệt đối, đã sắp xếp).)"""
    repo_root = REPO_ROOT
    files = []
    for root in SCAN_ROOTS:
        base = os.path.join(repo_root, root)
        if os.path.isfile(base) and base.endswith(".rs"):
            files.append(base)
            continue
        if not os.path.isdir(base):
            raise FileNotFoundError(f"required Rust scan root is missing: {root}")

        def raise_walk_error(error):
            raise error

        for dirpath, dirnames, filenames in os.walk(base, onerror=raise_walk_error):
            dirnames[:] = sorted(
                name for name in dirnames
                if name not in {".git", ".venv", "deps", "node_modules", "target", "vendor"}
            )
            for name in sorted(filenames):
                if name.endswith(".rs"):
                    files.append(os.path.join(dirpath, name))
    return sorted(files)

def _iter_python_files() -> list:
    """Every first-party Python source under audited source roots.
    (Mọi source Python first-party trong các root được kiểm toán.)"""
    repo_root = REPO_ROOT
    files = []
    for root in PYTHON_SCAN_ROOTS:
        base = os.path.join(repo_root, root)
        if not os.path.isdir(base):
            raise FileNotFoundError(f"required Python scan root is missing: {root}")

        def raise_walk_error(error):
            raise error

        for dirpath, dirnames, filenames in os.walk(base, onerror=raise_walk_error):
            dirnames[:] = sorted(
                name for name in dirnames
                if name not in {
                    ".git", ".gitnexus", ".kilo", "__pycache__", ".venv", ".pytest_cache",
                    "build", "deps", "dist", "node_modules", "target", "vendor",
                }
            )
            for name in sorted(filenames):
                if name.endswith(".py") and "gitnexus" not in name.lower():
                    files.append(os.path.join(dirpath, name))
    return sorted(files)

def scan_python_file(abs_path: str, repo_root: str) -> list:
    """Read and inventory one Python file, failing closed on read errors.
    (Đọc và kiểm kê một file Python; lỗi đọc được giữ thành finding.)"""
    rel_path = os.path.relpath(abs_path, repo_root).replace(os.sep, "/")
    try:
        with open(abs_path, "r", encoding="utf-8") as handle:
            return scan_python_text(rel_path, handle.read())
    except (OSError, UnicodeError) as error:
        return [{
            "file": rel_path,
            "line": 1,
            "api": "<read-error>",
            "executable": "<unreadable-python>",
            "review_status": "unreviewed",
            "parse_error": str(error),
        }]

def _iter_shell_files() -> list:
    """Walk shell sources in first-party and workflow trees deterministically.
    (Quét file shell trong cây first-party/workflow, bỏ build/cache/vendor.)"""
    repo_root = REPO_ROOT
    excluded = {
        ".git", ".gitnexus", ".kilo", ".venv", "__pycache__", ".pytest_cache",
        "build", "deps", "dist", "node_modules", "target", "vendor",
    }
    files = []
    for root in SHELL_SCAN_ROOTS:
        base = os.path.join(repo_root, root)
        if not os.path.isdir(base):
            raise FileNotFoundError(f"required shell scan root is missing: {root}")

        def raise_walk_error(error):
            raise error

        for dirpath, dirnames, filenames in os.walk(base, onerror=raise_walk_error):
            dirnames[:] = sorted(name for name in dirnames if name not in excluded)
            for filename in sorted(filenames):
                if (
                    filename.endswith((".sh", ".bash", ".zsh"))
                    and "gitnexus" not in filename.lower()
                ):
                    files.append(os.path.join(dirpath, filename))
    return sorted(set(files))

def scan_shell_file(abs_path: str, repo_root: str, scanned_shell_paths=None) -> list:
    """Read and inventory one shell file, failing closed on read errors.
    (Đọc và kiểm kê file shell; lỗi đọc phải xuất hiện trong ledger.)"""
    rel_path = os.path.relpath(abs_path, repo_root).replace(os.sep, "/")
    try:
        with open(abs_path, "r", encoding="utf-8") as handle:
            return scan_shell_text(rel_path, handle.read(), scanned_shell_paths)
    except (OSError, UnicodeError) as error:
        return [{
            "file": rel_path,
            "line": 1,
            "api": "<read-error>",
            "executable": "<unreadable-shell>",
            "review_status": "unreviewed",
            "parse_error": str(error),
        }]

def _iter_files_with_suffixes(suffixes: tuple) -> list:
    """Enumerate first-party source files for supplemental process scans.
    (Liệt kê source first-party cho các lượt quét process bổ sung.)"""
    repo_root = REPO_ROOT
    excluded = {
        ".git", ".gitnexus", ".kilo", ".venv", "__pycache__", ".pytest_cache",
        "build", "deps", "dist", "node_modules", "target", "vendor",
    }
    files = []
    for dirpath, dirnames, filenames in os.walk(repo_root):
        dirnames[:] = sorted(name for name in dirnames if name not in excluded)
        for filename in sorted(filenames):
            if filename.endswith(suffixes) and "gitnexus" not in filename.lower():
                files.append(os.path.join(dirpath, filename))
    return sorted(files)

__all__ = ['_iter_rust_files', '_iter_python_files', 'scan_python_file', '_iter_shell_files', 'scan_shell_file', '_iter_files_with_suffixes']
