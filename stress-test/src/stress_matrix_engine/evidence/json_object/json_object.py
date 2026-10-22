"""Owns the json object responsibility in the stress-matrix runner.
Sở hữu trách nhiệm json object trong stress-matrix runner.
"""

from __future__ import annotations

import json

from typing import Any

def _parse_json_object(output: str) -> dict[str, Any] | None:
    """Find a JSON object after optional CLI warnings.
    Tìm JSON object sau các warning tùy chọn của CLI.
    """
    for index, character in enumerate(output):
        if character != "{":
            continue
        try:
            payload, _end = json.JSONDecoder().raw_decode(output[index:])
        except json.JSONDecodeError:
            continue
        if isinstance(payload, dict):
            return payload
    return None

def _is_json_object(output: str) -> bool:
    return _parse_json_object(output) is not None

__all__ = ['_parse_json_object', '_is_json_object']
