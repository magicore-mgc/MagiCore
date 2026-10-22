#!/usr/bin/env python3
"""Exercise the built CLI against every framework in its capability manifest.
Chạy CLI đã build trên mọi framework được khai báo trong capability manifest.
"""

from __future__ import annotations

# Retain the original top-level imports for legacy import consumers and test patches.
# Giữ các import cấp module cũ cho caller và patch test tương thích.
import argparse
import datetime as dt
import hashlib
import importlib.util
import json
import os
import platform
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
from collections import Counter, defaultdict
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

_ENGINE_NAME = "_mgc_stress_matrix_engine"
_ENGINE_INIT = Path(__file__).resolve().parent / "stress_matrix_engine" / "__init__.py"
_ENGINE_SPEC = importlib.util.spec_from_file_location(
    _ENGINE_NAME,
    _ENGINE_INIT,
    submodule_search_locations=[str(_ENGINE_INIT.parent)],
)
if _ENGINE_SPEC is None or _ENGINE_SPEC.loader is None:
    raise ImportError("cannot load the stress-matrix engine package")
_ENGINE_MODULE = importlib.util.module_from_spec(_ENGINE_SPEC)
sys.modules[_ENGINE_NAME] = _ENGINE_MODULE
_ENGINE_SPEC.loader.exec_module(_ENGINE_MODULE)
__all__ = _ENGINE_MODULE.__all__
globals().update({name: getattr(_ENGINE_MODULE, name) for name in __all__})

if __name__ == "__main__":
    raise SystemExit(main())
