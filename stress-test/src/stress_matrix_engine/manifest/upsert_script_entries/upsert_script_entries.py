"""Owns the upsert script entries responsibility in the stress-matrix runner.
Sở hữu trách nhiệm upsert script entries trong stress-matrix runner.
"""

from __future__ import annotations

import json

import re

from pathlib import Path

def upsert_script_entries(config_path: Path, entries: dict[str, str]) -> None:
    """Add deterministic CLI smoke scripts without duplicating TOML keys.
    Thêm script smoke CLI xác định, không tạo key TOML trùng.
    """
    lines = config_path.read_text(encoding="utf-8").splitlines()
    section_starts = [
        index
        for index, line in enumerate(lines)
        if re.fullmatch(r"\s*\[scripts\]\s*(?:#.*)?", line)
    ]
    if len(section_starts) > 1:
        raise ValueError("mgc.toml contains more than one [scripts] table")

    if not section_starts:
        if re.search(r"^\s*scripts\s*=", "\n".join(lines), re.MULTILINE):
            raise ValueError("mgc.toml uses an inline scripts value that the stress runner cannot extend")
        if lines and lines[-1].strip():
            lines.append("")
        lines.append("[scripts]")
        lines.extend(f"{key} = {json.dumps(command)}" for key, command in entries.items())
        config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
        return

    section_start = section_starts[0]
    section_end = next(
        (
            index
            for index in range(section_start + 1, len(lines))
            if re.match(r"\s*\[[^]]+\]", lines[index])
        ),
        len(lines),
    )
    for key, command in entries.items():
        key_pattern = re.compile(rf"^\s*{re.escape(key)}\s*=")
        matches = [
            index
            for index in range(section_start + 1, section_end)
            if key_pattern.match(lines[index])
        ]
        replacement = f"{key} = {json.dumps(command)}"
        if len(matches) > 1:
            raise ValueError(f"mgc.toml contains duplicate [scripts].{key} entries")
        if matches:
            lines[matches[0]] = replacement
        else:
            lines.insert(section_end, replacement)
            section_end += 1
    config_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

__all__ = ['upsert_script_entries']
