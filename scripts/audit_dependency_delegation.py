#!/usr/bin/env python3
"""Compatibility CLI and import alias for the decomposed audit package."""

from __future__ import annotations

import sys

from dependency_delegation_audit import dependency_delegation_audit as _audit

if __name__ == "__main__":
    sys.exit(_audit.main())
else:
    sys.modules[__name__] = _audit
