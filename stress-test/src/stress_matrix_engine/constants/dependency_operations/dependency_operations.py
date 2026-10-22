"""Owns the dependency operations responsibility in the stress-matrix runner.
Sở hữu trách nhiệm dependency operations trong stress-matrix runner.
"""

from __future__ import annotations

DEPENDENCY_OPERATIONS = (
    "install",
    "add",
    "remove",
    "update",
    "list",
    "resolve",
    "lock",
    "fetch",
    "verify",
    "store",
    "materialize",
    "frozen-install",
    "offline-reinstall",
    "gc",
)

DEPENDENCY_OWNERS = frozenset({"mgc-native", "scaffold-only", "unsupported"})

__all__ = ['DEPENDENCY_OPERATIONS', 'DEPENDENCY_OWNERS']
