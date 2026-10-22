"""Owns stress-matrix command-line argument definitions.
Sở hữu định nghĩa tham số dòng lệnh stress-matrix.
"""

from __future__ import annotations

import argparse
from pathlib import Path

from ....constants.core_inventory.core_inventory import DEFAULT_INVENTORY_BASELINE

def build_argument_parser() -> argparse.ArgumentParser:
    """Build the existing stress-matrix CLI parser and defaults.
    Tạo parser và giá trị mặc định hiện có của stress-matrix CLI.
    """
    parser = argparse.ArgumentParser(
        description=(
            "Exercise the built CLI against every framework in its capability manifest.\n"
            "Chạy CLI đã build trên mọi framework được khai báo trong capability manifest."
        )
    )
    parser.add_argument("--binary", default="target/debug/mgc", help="Path to the built mgc executable")
    parser.add_argument("--report", default="stress-test-report.json", help="Path for the JSON evidence report")
    parser.add_argument("--framework-timeout", type=int, default=90, help="Timeout per framework create command")
    parser.add_argument("--keep-workspace", help="Copy the isolated fixture tree here when any check fails")
    parser.add_argument(
        "--inventory-manifest", type=Path, default=DEFAULT_INVENTORY_BASELINE,
        help="Versioned expected framework/status/owner inventory",
    )
    parser.add_argument(
        "--require-full-evidence", action="store_true",
        help="Exit non-zero when any optimizer or security-audit result has only unverified evidence",
    )
    parser.add_argument(
        "--require-language-runtime-evidence", action="store_true",
        help="Exit non-zero when any standalone source-language probe is missing, unverified, or failed",
    )
    parser.add_argument(
        "--require-lockfile-evidence", action="store_true",
        help="Require empty-graph V3/V4 lock replay for every framework with a wired native install owner",
    )
    parser.add_argument(
        "--require-universal-support", action="store_true",
        help=(
            "Fail unless every framework has verified language runtime and CLI install/run/test/build/dev, optimizer, cache, "
            "security, shared-store, and V3/V4 evidence plus native ownership of all dependency operations"
        ),
    )
    return parser

__all__ = ["build_argument_parser"]
