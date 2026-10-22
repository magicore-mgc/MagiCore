"""Owns the read project identity responsibility in the stress-matrix runner.
Sở hữu trách nhiệm read project identity trong stress-matrix runner.
"""

from __future__ import annotations

import re

from pathlib import Path

from typing import Any

from ...manifest.parse_toml_string.parse_toml_string import parse_toml_string

from ...manifest.parse_toml_string_array.parse_toml_string_array import parse_toml_string_array

def read_project_identity(config_path: Path) -> dict[str, Any]:
    """Read the generated top-level identity fields without third-party TOML packages.
    Đọc các trường identity đầu file mà không cần package TOML bên ngoài.
    """
    lines = config_path.read_text(encoding="utf-8").splitlines()
    top_level: list[str] = []
    for line in lines:
        if line.lstrip().startswith("["):
            break
        top_level.append(line)

    values: dict[str, Any] = {}
    for index, line in enumerate(top_level):
        match = re.match(r"^\s*(ecosystem|mode|frameworks)\s*=\s*(.*?)\s*$", line)
        if not match:
            continue
        key, raw = match.groups()
        if key in values:
            raise ValueError(f"duplicate {key} field")
        if key in {"ecosystem", "mode"}:
            values[key] = parse_toml_string(raw)
            continue
        while raw.count("[") > raw.count("]"):
            index += 1
            if index >= len(top_level):
                raise ValueError("unterminated frameworks array")
            raw += top_level[index].strip()
        values[key] = parse_toml_string_array(raw)
    section: str | None = None
    for line in lines:
        header = re.match(r"^\s*\[([^]]+)\]\s*$", line)
        if header:
            section = header.group(1)
            continue
        if section != "iot":
            continue
        field = re.match(r"^\s*(framework|board)\s*=\s*(.*?)\s*$", line)
        if field:
            key, raw = field.groups()
            target = "iot_framework" if key == "framework" else "iot_board"
            if target in values:
                raise ValueError(f"duplicate {target} field")
            values[target] = parse_toml_string(raw)
    if not values:
        raise ValueError("top-level ecosystem/frameworks fields are missing")
    return values

__all__ = ['read_project_identity']
