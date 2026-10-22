"""Owns parsing of clap help command sections.
Sở hữu phân tích danh sách command trong clap help.
"""

from __future__ import annotations

import re

def parse_help_commands(output: str) -> list[str]:
    """Read clap's immediate child commands from a help section.
    Đọc các lệnh con trực tiếp từ phần help do clap sinh.
    """
    lines = output.splitlines()
    try:
        start = next(
            index
            for index, line in enumerate(lines)
            if line.strip() == "Commands:"
        ) + 1
    except StopIteration:
        return []

    commands: list[str] = []
    for line in lines[start:]:
        if not line.strip():
            break
        match = re.match(r"^  ([a-z][a-z0-9-]*)(?:\s{2,}.*)?$", line)
        if match:
            commands.append(match.group(1))
    return commands

__all__ = ["parse_help_commands"]
