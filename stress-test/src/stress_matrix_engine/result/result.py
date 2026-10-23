"""Owns the result responsibility in the stress-matrix runner.
Sở hữu trách nhiệm result trong stress-matrix runner.
"""

from __future__ import annotations

from dataclasses import dataclass

@dataclass
class Result:
    """One auditable command result — một kết quả lệnh có thể rà soát."""

    category: str
    subject: str
    passed: bool
    detail: str
    returncode: int | None = None
    evidence_status: str = "verified"

__all__ = ['Result']
