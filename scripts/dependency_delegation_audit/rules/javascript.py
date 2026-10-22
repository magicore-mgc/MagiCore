"""Javascript implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations

JS_PROCESS_APIS = {"exec", "execFile", "execFileSync", "execSync", "fork", "spawn", "spawnSync"}

__all__ = []
