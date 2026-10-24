#!/usr/bin/env python3
"""Compatibility CLI and import alias for the decomposed audit package."""

from __future__ import annotations

import sys
from pathlib import Path

# The packages live directly under `scripts/`; this facade sits one level
# deeper so the import root must be added explicitly.
# Package nằm ngay dưới `scripts/`; facade nằm sâu một cấp nên phải thêm
# import root tường minh.
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from dependency_delegation_audit import dependency_delegation_audit as _audit

if __name__ == "__main__":
    sys.exit(_audit.main())
else:
    sys.modules[__name__] = _audit
