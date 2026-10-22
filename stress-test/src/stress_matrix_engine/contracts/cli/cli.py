"""Owns the cli responsibility in the stress-matrix runner.
Sở hữu trách nhiệm cli trong stress-matrix runner.
"""
from __future__ import annotations
import subprocess
from pathlib import Path
from ...constants.core_inventory.core_inventory import MAX_CLI_HELP_COMMANDS
from ...process.run_command.run_command import run_command
from ...result.result import Result
from .help_commands.help_commands import parse_help_commands
def run_cli_contracts(binary: Path, root: Path, env: dict[str, str]) -> list[Result]:
    """Check every discoverable command/subcommand help path and bad options.
    Kiểm tra help của mọi lệnh/subcommand được công bố và option sai.
    """
    results: list[Result] = []
    try:
        root_help = run_command(
            [str(binary), "--help"], cwd=root, env=env, timeout_seconds=10
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        root_help = None
        root_detail = str(error)
        root_returncode = None
    else:
        root_detail = root_help.stdout[-1000:] or "root help returned no Usage section"
        root_returncode = root_help.returncode

    if root_help is None or root_help.returncode != 0 or "Usage:" not in root_help.stdout:
        results.append(
            Result(
                "cli-help-inventory",
                "mgc --help",
                False,
                root_detail,
                root_returncode,
            )
        )
        command_paths: list[tuple[str, ...]] = []
    else:
        command_paths = [(name,) for name in parse_help_commands(root_help.stdout)]
        if not command_paths:
            results.append(
                Result(
                    "cli-help-inventory",
                    "mgc --help command inventory",
                    False,
                    "root help has no discoverable Commands section",
                    root_help.returncode,
                )
            )

    visited: set[tuple[str, ...]] = set()
    while command_paths:
        path = command_paths.pop(0)
        if path in visited:
            continue
        if len(visited) >= MAX_CLI_HELP_COMMANDS:
            results.append(
                Result(
                    "cli-help-inventory",
                    "command tree limit",
                    False,
                    f"more than {MAX_CLI_HELP_COMMANDS} unique command paths were advertised",
                )
            )
            break
        visited.add(path)
        is_help_command = path[-1] == "help"
        argv = [str(binary), *path]
        if not is_help_command:
            argv.append("--help")
        label = "mgc " + " ".join(path)
        if not is_help_command:
            label += " --help"
        try:
            completed = run_command(
                argv, cwd=root, env=env, timeout_seconds=10
            )
            passed = completed.returncode == 0 and "Usage:" in completed.stdout
            detail = "help returned successfully" if passed else completed.stdout[-1000:]
            results.append(Result("cli-command-help", label, passed, detail, completed.returncode))
            if passed and not is_help_command:
                command_paths.extend(
                    path + (name,) for name in parse_help_commands(completed.stdout)
                )
        except (OSError, subprocess.TimeoutExpired) as error:
            results.append(Result("cli-command-help", label, False, str(error)))

        invalid_argv = [str(binary), *path, "--definitely-invalid-mgc-option"]
        invalid_label = "mgc " + " ".join(path) + " rejects unknown option"
        try:
            invalid = run_command(
                invalid_argv, cwd=root, env=env, timeout_seconds=10
            )
            output_lower = invalid.stdout.lower()
            passed = (
                invalid.returncode != 0
                and "--definitely-invalid-mgc-option" in invalid.stdout
                and any(
                    marker in output_lower
                    for marker in ("unexpected argument", "unrecognized subcommand")
                )
            )
            results.append(
                Result(
                    "security-cli-command",
                    invalid_label,
                    passed,
                    "unknown option rejected before command execution"
                    if passed
                    else invalid.stdout[-1000:],
                    invalid.returncode,
                )
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            results.append(Result("security-cli-command", invalid_label, False, str(error)))

    try:
        completed = run_command(
            [str(binary), "--definitely-invalid-mgc-option"],
            cwd=root,
            env=env,
            timeout_seconds=10,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        results.append(
            Result("security-cli", "unknown global option is rejected", False, str(error))
        )
    else:
        passed = completed.returncode != 0 and "Usage:" in completed.stdout
        results.append(
            Result(
                "security-cli",
                "unknown global option is rejected",
                passed,
                "invalid option rejected" if passed else completed.stdout[-1000:],
                completed.returncode,
            )
        )
    return results

__all__ = ["run_cli_contracts", "parse_help_commands"]
