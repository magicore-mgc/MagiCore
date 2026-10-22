"""Owns the parse toml string array responsibility in the stress-matrix runner.
Sở hữu trách nhiệm parse toml string array trong stress-matrix runner.
"""

from __future__ import annotations

import json

def parse_toml_string_array(raw: str) -> list[str]:
    """Decode a generated TOML array containing only quoted strings.
    Giải mã mảng TOML do MGC sinh, chỉ gồm các chuỗi được trích dẫn.
    """
    value = raw.strip()
    if not value.startswith("[") or not value.endswith("]"):
        raise ValueError("expected a TOML array")
    items: list[str] = []
    index = 1
    decoder = json.JSONDecoder()
    while index < len(value) - 1:
        while index < len(value) - 1 and (value[index].isspace() or value[index] == ","):
            index += 1
        if index >= len(value) - 1:
            break
        if value[index] == "'":
            end = value.find("'", index + 1)
            if end < 0:
                raise ValueError("unterminated literal string in frameworks array")
            items.append(value[index + 1:end])
            index = end + 1
        elif value[index] == '"':
            item, consumed = decoder.raw_decode(value[index:])
            if not isinstance(item, str):
                raise ValueError("frameworks array must contain strings")
            items.append(item)
            index += consumed
        else:
            raise ValueError("frameworks array contains a non-string value")
        while index < len(value) - 1 and value[index].isspace():
            index += 1
        if index < len(value) - 1 and value[index] != ",":
            raise ValueError("expected a comma between framework strings")
    return items

__all__ = ['parse_toml_string_array']
