"""Traps implementation."""

from __future__ import annotations

import re

from .commands import _shell_command_segments
from .primitives import _shell_alias_variable, _shell_strip_comment

from ..rules.shell import (
    SHELL_ASSIGNMENT_RE,
    SHELL_CONTROL_WORDS,
)

from .aliases import (
    _shell_command_aliases,
    _shell_static_script_aliases,
)

from .execution import (
    _shell_executable,
    _shell_function_defined_before,
)

def _shell_trap_action_is_static(text: str, line_no: int, segment: list) -> bool:
    """Accept only empty traps, prior local functions, or literal non-tool actions.
    (Chỉ chấp nhận trap rỗng, function nội bộ có trước, hoặc action literal không gọi tool.)"""
    action_index = 1
    if action_index < len(segment) and segment[action_index] == "--":
        action_index += 1
    if action_index >= len(segment) or segment[action_index] in {"", "-"}:
        return True
    action = segment[action_index]
    if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", action):
        return _shell_function_defined_before(text, action, line_no) and not (
            _shell_trap_callback_mutated_after(text, action, line_no)
        )
    nested_segments = list(_shell_command_segments(action))
    if not nested_segments:
        return False
    aliases = _shell_command_aliases(text)
    static_aliases = _shell_static_script_aliases(text, "trap-action.sh")
    for _, nested, parse_error in nested_segments:
        if parse_error:
            return False
        executable = _shell_executable(nested, aliases, static_aliases)
        if executable is not None:
            return False
    return True
def _shell_trap_callback_mutated_after(text: str, name: str, line_no: int) -> bool:
    """Reject a static trap review if later code can remove or replace its callback.
    (Bỏ review trap tĩnh nếu code sau đó có thể xóa hoặc thay callback.)"""
    function_header = re.compile(
        r"^\s*(?:function\s+)?" + re.escape(name) + r"\s*(?:\(\s*\))?\s*\{"
    )
    for command_line, segment, parse_error in _shell_command_segments(text):
        if command_line <= line_no or parse_error or not segment:
            continue
        visible_command = " ".join(segment)
        if not function_header.match(_shell_strip_comment(visible_command)):
            continue
        return True

    for command_line, segment, parse_error in _shell_command_segments(text):
        if command_line <= line_no or parse_error or not segment:
            continue
        command_index = 0
        while command_index < len(segment) and (
            segment[command_index] in SHELL_CONTROL_WORDS
            or SHELL_ASSIGNMENT_RE.match(segment[command_index])
        ):
            command_index += 1
        if command_index >= len(segment):
            continue
        command = segment[command_index].replace("\\", "/").rsplit("/", 1)[-1].lower()
        if command != "unset":
            continue
        arguments = segment[command_index + 1 :]
        function_unset = any(
            argument == "-f" or (argument.startswith("-") and "f" in argument[1:])
            for argument in arguments
        )
        if not function_unset:
            continue
        names = [argument for argument in arguments if not argument.startswith("-")]
        if not names or any("$" in argument or "`" in argument for argument in names):
            return True
        if any(_shell_alias_variable(argument) == name for argument in names):
            return True
    return False

__all__ = ['_shell_trap_action_is_static', '_shell_trap_callback_mutated_after']
