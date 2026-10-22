"""Owns the project slug responsibility in the stress-matrix runner.
Sở hữu trách nhiệm project slug trong stress-matrix runner.
"""

from __future__ import annotations

import re

def project_slug(core: str, framework: str) -> str:
    """Make a portable, single-segment project name.
    Tạo tên project một segment, dùng được trên các hệ điều hành.
    """
    safe = re.sub(r"[^a-zA-Z0-9]+", "-", framework).strip("-").lower()
    return f"mgc-{core}-{safe or 'framework'}"

__all__ = ['project_slug']
