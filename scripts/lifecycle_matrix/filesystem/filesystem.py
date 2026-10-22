"""Filesystem marker and directory evidence helpers."""

from __future__ import annotations

import os

def _dir_nonempty(path: str) -> bool:
    """True when `path` is a directory containing at least one entry.
    (True khi `path` là thư mục có ít nhất một mục con.)"""
    try:
        with os.scandir(path) as entries:
            return any(entries)
    except OSError:
        return False

__all__ = ['_dir_nonempty']
