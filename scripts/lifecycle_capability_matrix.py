#!/usr/bin/env python3
"""Stable CLI/import facade for the lifecycle capability matrix package."""

from __future__ import annotations

import sys

from lifecycle_matrix import api as _api

if __name__ == "__main__":
    sys.exit(_api.main())
else:
    sys.modules[__name__] = _api
