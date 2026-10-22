"""Alias Mutation implementation."""

from __future__ import annotations

import posixpath
import re

from .primitives import (
    _shell_alias_variable,
    _shell_arithmetic_writes_alias,
    _shell_assignment_items,
)

from ..rules.shell import (
    SHELL_ASSIGNMENT_RE,
    SHELL_CONTROL_WORDS,
)

from .aliases import (
    _shell_directory_assignment_paths,
    _shell_script_argument_targets,
)

def _shell_script_alias_mutation_index(
    segments: list, aliases: dict, rel_path: str, text: str
):
    """Find the first command that can rewrite a static script-path alias.
    (Tìm command đầu tiên có thể ghi đè alias đường dẫn script tĩnh.)"""
    alias_names = set(aliases)
    if not alias_names:
        return None
    source_dir = posixpath.dirname(rel_path)
    safe_assignment_lines = set()
    for line_no, line in enumerate(text.splitlines(), start=1):
        for name, value in _shell_assignment_items(line):
            if name not in aliases:
                continue
            safe_directory = _shell_directory_assignment_paths(
                value, source_dir, aliases
            ) == aliases[name]
            normalized_value = value.strip()
            if (
                len(normalized_value) >= 2
                and normalized_value[0] == normalized_value[-1]
                and normalized_value[0] in "\"'"
            ):
                normalized_value = normalized_value[1:-1]
            safe_file = _shell_script_argument_targets(
                normalized_value, aliases
            ) == aliases[name]
            if safe_directory or safe_file:
                safe_assignment_lines.add(line_no)

    for segment_index, (line_no, segment, parse_error) in enumerate(segments):
        if parse_error:
            return segment_index
        if not segment:
            continue
        if _shell_arithmetic_writes_alias(segment, alias_names):
            return segment_index
        for token in segment:
            assignment = re.match(
                r"^([A-Za-z_][A-Za-z0-9_]*)(?:\[[^]]+\])?\+?=", token
            )
            if assignment and assignment.group(1) in alias_names:
                if line_no not in safe_assignment_lines:
                    return segment_index
            parameter_assignment = re.search(
                r"\$\{([A-Za-z_][A-Za-z0-9_]*)(?:\[[^]]+\])?(?::?=)",
                token,
            )
            if parameter_assignment and parameter_assignment.group(1) in alias_names:
                return segment_index
        for index, token in enumerate(segment[:-1]):
            if token.lower() in {"for", "select"} and _shell_alias_variable(
                segment[index + 1]
            ) in alias_names:
                return segment_index

        command_index = 0
        while command_index < len(segment):
            token = segment[command_index]
            if token in SHELL_CONTROL_WORDS or SHELL_ASSIGNMENT_RE.match(token):
                command_index += 1
                continue
            if token in {"!", "{"}:
                command_index += 1
                continue
            break
        if command_index >= len(segment):
            continue
        command = segment[command_index].replace("\\", "/").rsplit("/", 1)[-1].lower()
        args = segment[command_index + 1 :]
        if command in {"builtin", "command"} and args:
            command = args[0].replace("\\", "/").rsplit("/", 1)[-1].lower()
            args = args[1:]
        if command in {"eval", "source", "."}:
            return segment_index
        if command == "read":
            destinations = []
            index = 0
            value_options = {"-d", "-i", "-n", "-N", "-p", "-t", "-u"}
            while index < len(args):
                arg = args[index]
                if arg == "-a":
                    if index + 1 < len(args):
                        destinations.append(_shell_alias_variable(args[index + 1]))
                    index += 2
                elif arg in value_options:
                    index += 2
                elif arg.startswith("-"):
                    index += 1
                else:
                    destinations.append(_shell_alias_variable(arg))
                    index += 1
            if not destinations:
                destinations = ["REPLY"]
            if any(name in alias_names for name in destinations) or any(
                "$" in arg
                for index, arg in enumerate(args)
                if not arg.startswith("-")
                and not (index > 0 and args[index - 1] in value_options | {"-a"})
            ):
                return segment_index
        elif command in {"mapfile", "readarray"}:
            destinations = [
                _shell_alias_variable(arg)
                for arg in args
                if not arg.startswith("-")
            ]
            if not destinations:
                destinations = ["MAPFILE"]
            if any(name in alias_names for name in destinations):
                return segment_index
        elif command == "printf":
            for index, arg in enumerate(args[:-1]):
                if arg == "-v":
                    destination = args[index + 1]
                    if _shell_alias_variable(destination) in alias_names or "$" in destination:
                        return segment_index
        elif command == "unset":
            if any(_shell_alias_variable(arg) in alias_names or "$" in arg for arg in args):
                return segment_index
        elif command in {"declare", "typeset", "local"}:
            if any(arg == "-n" or arg.startswith("-n") for arg in args):
                return segment_index
            if any(
                _shell_alias_variable(arg.partition("=")[0]) in alias_names
                for arg in args
                if not arg.startswith("-")
            ):
                return segment_index
    return None

__all__ = ['_shell_script_alias_mutation_index']
