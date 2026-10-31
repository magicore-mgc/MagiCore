"""Javascript implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations

JS_PROCESS_APIS = {"exec", "execFile", "execFileSync", "execSync", "fork", "spawn", "spawnSync"}

# Reviewed JavaScript process calls, keyed by file then (api, source sha256).
# A reviewed entry means a human audited this exact spawn site; the npm CLI
# shim is the ONE intentional boundary that runs the resolved platform binary.
# (Cuộc gọi process JS đã review: file → (api, sha256 nguồn). Shim npm là ranh
# giới DUY NHẤT cố ý chạy binary nền tảng đã resolve.)
JS_PROCESS_REVIEWS = {
    "npm/cli/bin/mgc.js": {
        ("child_process.spawnSync", "b5398ee482f7d2b18e07b8330644e60bbccbb17250eed106021054281c5e3c27"): (
            2,
            "Executes the user's own mgc argv via the @magicore/cli-<os>-<arch> "
            "optional dependency resolved from node_modules; no shell, no "
            "network, argv inherited from the caller.",
        ),
    },
}

__all__ = ['JS_PROCESS_APIS', 'JS_PROCESS_REVIEWS']
