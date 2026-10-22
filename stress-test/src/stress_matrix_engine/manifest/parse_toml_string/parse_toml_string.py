"""Owns the parse toml string responsibility in the stress-matrix runner.
Sở hữu trách nhiệm parse toml string trong stress-matrix runner.
"""

from __future__ import annotations

import json

def parse_toml_string(raw: str) -> str:
    """Decode a basic TOML single- or double-quoted string.
    Giải mã chuỗi TOML cơ bản dùng nháy đơn hoặc nháy kép.
    """
    value = raw.strip()
    if len(value) < 2 or value[0] not in {"'", '"'} or value[-1] != value[0]:
        raise ValueError("expected a quoted TOML string")
    if value[0] == "'":
        return value[1:-1]
    decoded = json.loads(value)
    if not isinstance(decoded, str):
        raise ValueError("expected a TOML string")
    return decoded

__all__ = ['parse_toml_string']
