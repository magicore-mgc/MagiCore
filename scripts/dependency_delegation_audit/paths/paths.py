"""Repository and report paths for the dependency delegation audit."""

from __future__ import annotations

import os

PACKAGE_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SCRIPTS_ROOT = os.path.dirname(PACKAGE_ROOT)
REPO_ROOT = os.path.dirname(SCRIPTS_ROOT)
OUTPUT_PATH = os.path.join(SCRIPTS_ROOT, os.pardir, "docs", "specs", "dependencyDelegationAudit.json")

__all__ = ["PACKAGE_ROOT", "SCRIPTS_ROOT", "REPO_ROOT", "OUTPUT_PATH"]
