"""Owns the core inventory responsibility in the stress-matrix runner.
Sở hữu trách nhiệm core inventory trong stress-matrix runner.
"""

from __future__ import annotations

from pathlib import Path

CORE_NAMES = ("web", "ai", "app", "lib", "game", "iot", "clo", "cicd", "hardware")

CREATE_COMMANDS = tuple(f"create-{core}" for core in CORE_NAMES)

MAX_CLI_HELP_COMMANDS = 256

DEFAULT_INVENTORY_BASELINE = (
    Path(__file__).resolve().parents[4] / "fixtures" / "framework-inventory.json"
)

V4_INSTALL_CORES = frozenset({"web", "app", "lib", "ai", "game", "iot", "clo"})

__all__ = ['CORE_NAMES', 'CREATE_COMMANDS', 'MAX_CLI_HELP_COMMANDS', 'DEFAULT_INVENTORY_BASELINE', 'V4_INSTALL_CORES']
