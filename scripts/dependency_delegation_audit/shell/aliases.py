"""Aliases implementation."""

from __future__ import annotations

import posixpath
import re

from .primitives import _shell_assignment_items

from ..rules.shell import (
    SHELL_SCRIPT_SUFFIXES,
)

def _shell_command_aliases(text: str) -> dict:
    """Resolve shell executable variables only from fixed source assignments.
    Chỉ phân giải biến executable dựa trên assignment cố định trong source.
    """
    assignments = {}
    for name, value in _shell_assignment_items(text):
        assignments.setdefault(name, []).append(value)

    aliases = {}
    for name in ("MGC", "MGC_BIN", "MGC_BINARY", "MGC_PATH"):
        values = assignments.get(name, [])
        if values and all(
            value.strip().strip("\"'") == "mgc"
            or
            re.search(r"(?:^|[/\\])mgc(?:$|[\s\"'}/])", value)
            or re.search(r"\b(?:command\s+-v|which)\s+mgc\b", value)
            for value in values
        ):
            aliases[name] = "mgc"
    if re.search(
        r'\[\[\s+-f\s+"\$MGC_BIN"\s+&&\s+-x\s+"\$MGC_BIN"\s+\]\]',
        text,
    ) or re.search(r'\[\s+-x\s+"\$MGC_BIN"\s+\]', text):
        # The environment-selected binary is executable only after an exact
        # regular-file and executable-bit guard in this script.
        # (Binary từ môi trường chỉ chạy sau guard chính xác file + quyền chạy.)
        aliases.setdefault("MGC_BIN", "mgc")
    if re.search(r'\[\s+-x\s+"\$MGC_BINARY"\s+\]', text) or re.search(
        r'if\s+\[\s+!\s+-x\s+"\$MGC_BINARY"\s+\]\s*;?\s*then[\s\S]{0,240}\bexit\s+1',
        text,
    ):
        # The binary override is restricted to an executable path before use.
        # (Binary override chỉ được dùng khi đường dẫn có quyền thực thi.)
        aliases.setdefault("MGC_BINARY", "mgc")

    pnpm_commands = assignments.get("PNPM_BIN", [])
    if pnpm_commands and all(
        re.fullmatch(
            r"\s*\$\(command\s+-v\s+pnpm\)\s*",
            value.strip().strip("\"'"),
        )
        for value in pnpm_commands
    ):
        aliases["PNPM_BIN"] = "pnpm"

    hash_commands = assignments.get("HASH_CMD", [])
    if hash_commands and all(
        re.match(r"\s*[\"']?(?:sha256sum|shasum)(?:\s|[\"']|$)", value)
        for value in hash_commands
    ):
        aliases["HASH_CMD"] = "checksum-tool"

    contract_paths = assignments.get("CONTRACT", [])
    if contract_paths and all("release-artifact-contract.sh" in value for value in contract_paths):
        aliases["CONTRACT"] = "shell-script"

    for name in ("SCRIPT_DIR", "ROOT", "REPO_ROOT", "PROJECT_ROOT"):
        values = assignments.get(name, [])
        if values and all(
            "BASH_SOURCE" in value or "$0" in value or "$SCRIPT_DIR" in value
            for value in values
        ):
            aliases[name] = "shell-script-dir" if name == "SCRIPT_DIR" else "shell-script-root"

    matrix_commands = assignments.get("MATRIX_BIN", [])
    if matrix_commands and all(
        re.match(r"\s*[\"']?cargo\s+run\s+--bin\s+bench_matrix\b", value)
        for value in matrix_commands
    ):
        aliases["MATRIX_BIN"] = "cargo"

    dry_run_values = assignments.get("DRY_RUN", [])
    if dry_run_values and all(re.match(r"\s*(?:true|false)\s*$", value) for value in dry_run_values):
        aliases["DRY_RUN"] = "boolean-selector"

    pm_commands = assignments.get("PM_CMD", [])
    if pm_commands and re.search(r"case\s+\"\$PM_NAME\"\s+in", text):
        resolved = []
        for value in pm_commands:
            if "MGC_BINARY" in value and re.search(r"\binstall-web\b", value):
                resolved.append("mgc")
                continue
            match = re.match(r"\s*[\"']?(npm|pnpm|bun|yarn)\b", value)
            if not match:
                resolved = []
                break
            resolved.append(match.group(1))
        if resolved and set(resolved) <= {"mgc", "npm", "pnpm", "bun", "yarn"}:
            aliases["PM_CMD"] = "bounded-package-manager-selector"
    return aliases
def _shell_static_script_aliases(text: str, rel_path: str) -> dict:
    """Resolve only source-defined paths to scripts in this repository.
    (Chỉ phân giải đường dẫn script cố định trong source của repository.)"""
    assignments = {}
    for name, value in _shell_assignment_items(text):
        assignments.setdefault(name, []).append(value)
    source_dir = posixpath.dirname(rel_path)
    aliases = {}
    for _ in range(4):
        changed = False
        for name, values in assignments.items():
            if not values:
                continue
            resolved_values = []
            for raw_value in values:
                candidates = _shell_directory_assignment_paths(
                    raw_value, source_dir, aliases
                )
                if not candidates:
                    resolved_values = []
                    break
                resolved_values.append(candidates)
            if resolved_values:
                common = set.intersection(*resolved_values)
                if common and aliases.get(name) != common:
                    aliases[name] = common
                    changed = True
        if not changed:
            break
    file_aliases = {name: set(paths) for name, paths in aliases.items()}
    for _ in range(3):
        changed = False
        for name, values in assignments.items():
            if name in aliases or not values:
                continue
            resolved_values = []
            for raw_value in values:
                value = raw_value.strip()
                if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
                    value = value[1:-1]
                match = re.fullmatch(r"\$\{?([A-Za-z_][A-Za-z0-9_]*)\}?/(.*)", value)
                if not match or match.group(1) not in file_aliases:
                    resolved_values = []
                    break
                suffix = match.group(2)
                if (
                    not suffix.endswith(SHELL_SCRIPT_SUFFIXES)
                    or any(marker in suffix for marker in ("$", "`", ";", "(", ")"))
                ):
                    resolved_values = []
                    break
                candidates = {
                    posixpath.normpath(posixpath.join(base, suffix))
                    for base in file_aliases[match.group(1)]
                }
                if any(path in {"..", "."} or path.startswith("../") for path in candidates):
                    resolved_values = []
                    break
                resolved_values.append(candidates)
            if resolved_values:
                common = set.intersection(*resolved_values)
                if common:
                    file_aliases[name] = common
                    changed = True
        if not changed:
            break
    return file_aliases
def _shell_directory_assignment_paths(value: str, source_dir: str, aliases: dict) -> set:
    """Resolve only exact source-anchored directory expressions.
    (Chỉ phân giải biểu thức thư mục khớp chính xác và neo theo source.)"""
    value = value.strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
        value = value[1:-1]
    source_match = re.fullmatch(
        r'\$\(\s*cd\s+"\$\(\s*dirname\s+"(?:\$\{BASH_SOURCE\[0\]\}|\$0)"\)'
        r'(?P<up>(?:/\.\.)*)"\s*&&\s*pwd\s*\)',
        value,
    )
    if source_match:
        base = source_dir
        for _ in range(source_match.group("up").count("/..")):
            base = posixpath.dirname(base)
        normalized = posixpath.normpath(base)
        return {"" if normalized == "." else normalized}

    relative_match = re.fullmatch(
        r'\$\(\s*cd\s+"\$\{?([A-Za-z_][A-Za-z0-9_]*)\}?'
        r'(?P<up>(?:/\.\.)*)"\s*&&\s*pwd\s*\)',
        value,
    )
    if not relative_match:
        relative_match = re.fullmatch(
            r'\$\{?([A-Za-z_][A-Za-z0-9_]*)\}?(?P<up>(?:/\.\.)*)', value
        )
    if not relative_match:
        return set()
    bases = aliases.get(relative_match.group(1), set())
    paths = set()
    for base in bases:
        path = base
        for _ in range(relative_match.group("up").count("/..")):
            path = posixpath.dirname(path)
        normalized = posixpath.normpath(path)
        paths.add("" if normalized == "." else normalized)
    return paths
def _shell_script_argument_targets(argument: str, aliases: dict) -> set:
    """Resolve a shell script variable only to normalized repository paths.
    (Chỉ phân giải biến script về đường dẫn đã chuẩn hóa trong repository.)"""
    variable = re.fullmatch(r"\$\{?([A-Za-z_][A-Za-z0-9_]*)\}?(?:/(.*))?", argument)
    if not variable:
        return set()
    base_paths = aliases.get(variable.group(1), set())
    suffix = variable.group(2)
    if suffix is None:
        return {path for path in base_paths if path.endswith(SHELL_SCRIPT_SUFFIXES)}
    if (
        suffix.startswith("/")
        or any(marker in suffix for marker in ("$", "`", "*", "?", "[", "]"))
        or any(part == ".." for part in suffix.split("/"))
    ):
        return set()
    candidates = {
        posixpath.normpath(posixpath.join(base, suffix))
        for base in base_paths
    }
    return {
        path for path in candidates
        if path not in {"..", "."} and not path.startswith("../")
    }

__all__ = ['_shell_command_aliases', '_shell_static_script_aliases', '_shell_directory_assignment_paths', '_shell_script_argument_targets']
