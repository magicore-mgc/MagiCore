"""Owns the set lock schema responsibility in the stress-matrix runner.
Sở hữu trách nhiệm set lock schema trong stress-matrix runner.
"""

from __future__ import annotations

import json

import re

from pathlib import Path

def set_lock_schema(config_path: Path, schema: str) -> None:
    """Pin a generated project to one lock writer for deterministic evidence.
    Ghim project sinh ra vào một lock writer để bằng chứng xác định.
    """
    if schema not in {"3", "v4"}:
        raise ValueError(f"unsupported stress lock schema: {schema}")

    lines = config_path.read_text(encoding="utf-8").splitlines()
    lock_headers = [
        index
        for index, line in enumerate(lines)
        if re.fullmatch(r"\s*\[lock\]\s*(?:#.*)?", line)
    ]
    if len(lock_headers) > 1:
        raise ValueError("mgc.toml contains more than one [lock] table")
    if not lock_headers:
        if re.search(r"^\s*lock\s*=", "\n".join(lines), re.MULTILINE):
            raise ValueError("mgc.toml uses an inline lock value that the stress runner cannot extend")
        if lines and lines[-1].strip():
            lines.append("")
        lines.extend(("[lock]", f"schema = {json.dumps(schema)}"))
        config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
        return

    table_start = lock_headers[0]
    table_end = next(
        (
            index
            for index in range(table_start + 1, len(lines))
            if re.match(r"\s*\[[^]]+\]", lines[index])
        ),
        len(lines),
    )
    schema_keys = [
        index
        for index in range(table_start + 1, table_end)
        if re.match(r"\s*schema\s*=", lines[index])
    ]
    if len(schema_keys) > 1:
        raise ValueError("mgc.toml contains duplicate [lock].schema entries")
    replacement = f"schema = {json.dumps(schema)}"
    if schema_keys:
        lines[schema_keys[0]] = replacement
    else:
        lines.insert(table_end, replacement)
    config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

__all__ = ['set_lock_schema']
