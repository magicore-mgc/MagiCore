"""Immutable inputs captured before lifecycle matrix collection."""

from __future__ import annotations

from dataclasses import dataclass

@dataclass(frozen=True)
class MatrixRunContext:
    mgc_bin: str
    commit: str
    source_tree_clean_before_run: bool
    generated_at: str

__all__ = ["MatrixRunContext"]
