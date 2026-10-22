"""Stable in-process API for the dependency delegation audit."""

from __future__ import annotations

import os

from .paths.paths import REPO_ROOT, OUTPUT_PATH
from .rules.rules import *
from .rust.rust import (
    _message_spans, _inside_spans, _const_array_spans, _functions,
    _enclosing_function, _is_test_function, _is_external_harness_path,
    _classify, _line_excluded, scan_file,
)
from .rust.context import (
    _message_spans, _inside_spans, _const_array_spans, _functions,
    _enclosing_function, _is_test_function, _is_external_harness_path, _doc_block,
)
from .inventory.inventory import (
    _iter_rust_files, _iter_python_files, scan_python_file, _iter_shell_files,
    scan_shell_file, _iter_files_with_suffixes,
)
from .shell.commands import _shell_command_segments, _shell_execution_substitution_bodies
from .shell.primitives import (
    _shell_strip_comment, _shell_assignment_items, _shell_alias_variable,
    _shell_arithmetic_writes_alias, _shell_control_context,
)
from .shell.aliases import (
    _shell_command_aliases, _shell_static_script_aliases,
    _shell_directory_assignment_paths, _shell_script_argument_targets,
)
from .shell.alias_mutation import _shell_script_alias_mutation_index
from .shell.execution import (
    _shell_environment_unset_before, _shell_function_defined_before,
    _shell_interpreter_script_target, _shell_bash_shebang_is_login,
    _shell_executable, _shell_dependency_lifecycle_action,
)
from .shell.shell import scan_shell_text
from .shell.traps import _shell_trap_action_is_static, _shell_trap_callback_mutated_after
from .javascript.javascript import _mask_js_comments, _mask_js_strings, scan_javascript_text
from .powershell.powershell import _mask_powershell_comments_and_strings, scan_powershell_text
from .supplemental.supplemental import scan_supplemental_file
from .python.process_calls import _python_call_api, _python_executable, review_python_process_calls
from .python.helper_review import (
    _python_binding_root, _python_ast_fingerprint_dump,
    _python_process_helper_fingerprint, _python_aliases_reviewed_namespace,
    _python_references_module_registry, review_python_process_helpers,
)
from .python.python import _portable_ast_dump, scan_python_text
from .runner.runner import run_audit

LANE_GATE_ROOTS = [
    'cli/src/commands/core/install',
    'cli/src/commands/core/add',
    'cli/src/commands/core/remove',
    'cli/src/commands/core/update',
    'cli/src/commands/core/list',
]

def check_lane_gate(repo_root: str) -> tuple:
    """Every dependency-lane module with logic references `dep_gate::`
    or declares `GATE-EXEMPT:`. Pure routers (no function definitions)
    are exempt structurally — there is no logic to gate.
    (Mọi module lane dependency có logic phải nhắc `dep_gate::` hoặc khai
    `GATE-EXEMPT:`. Router thuần (không định nghĩa hàm) được miễn theo cấu
    trúc — không có logic để gate.)
    Returns (checked_count, missing_list)."""
    checked = 0
    missing = []
    for root in LANE_GATE_ROOTS:
        base = os.path.join(repo_root, root)

        if not os.path.isdir(base):
            checked += 1
            missing.append(f"<scan-error:missing-root> {os.path.normpath(root)}")
            continue

        def record_walk_error(error):
            nonlocal checked
            checked += 1
            error_path = getattr(error, "filename", None) or base
            missing.append(
                f"<scan-error:walk> {os.path.relpath(error_path, repo_root)}"
            )

        for dirpath, _dirnames, filenames in os.walk(base, onerror=record_walk_error):
            rel_dir = os.path.relpath(dirpath, repo_root)
            segments = [p.lower() for p in rel_dir.replace(os.sep, "/").split("/")]
            if any(seg in ("test", "tests", "bench", "benches") for seg in segments):
                continue
            for name in sorted(filenames):
                if not name.endswith(".rs"):
                    continue
                abs_path = os.path.join(dirpath, name)
                rel_path = os.path.relpath(abs_path, repo_root)
                try:
                    with open(abs_path, "r", encoding="utf-8", errors="replace") as handle:
                        text = handle.read()
                except OSError:
                    checked += 1
                    missing.append(f"<scan-error:unreadable> {rel_path}")
                    continue
                if "dep_gate::" in text or "GATE-EXEMPT:" in text:
                    checked += 1
                    continue
                if not any(FN_RE.match(line) for line in text.splitlines()):
                    continue  # pure router — no logic to gate
                checked += 1
                missing.append(rel_path)
    return checked, sorted(set(missing))


def main() -> int:
    """Run the audit from the active checkout root."""
    return run_audit(REPO_ROOT, check_lane_gate)

__all__ = ['ALLOWED_OPS', 'AUDITED_DIRECT_PROCESS_ROUTES', 'AUDITED_EXECUTOR_ROUTES', 'CONST_ARRAY_RE', 'DEPENDENCY_OPS', 'DESCRIPTOR_PATTERNS', 'DIRECT_PROCESS_ROUTE_PATTERNS', 'DYNAMIC_SPAWN_PATTERNS', 'ECOSYSTEM_OWNED_OPS', 'EXCLUDE_PATTERNS', 'EXECUTOR_ROUTE_PATTERNS', 'FN_RE', 'JS_PROCESS_APIS', 'LANE_GATE_ROOTS', 'MARKER', 'MESSAGE_MACROS', 'NON_EXECUTABLE_SENTINELS', 'NON_EXECUTABLE_SQL_PREFIXES', 'OUTPUT_PATH', 'PLATFORM_PROCESS_BOUNDARY_REVIEWS', 'PS_PROCESS_ALIASES', 'PS_PROCESS_APIS', 'PYTHON_PROCESS_FUNCTIONS', 'PYTHON_PROCESS_HELPER_DEPENDENCIES', 'PYTHON_PROCESS_HELPER_REVIEWS', 'PYTHON_PROCESS_REVIEWS', 'PYTHON_SCAN_ROOTS', 'SCAN_ROOTS', 'SHELL_ABSOLUTE_ENV_WRAPPERS', 'SHELL_APPLETS', 'SHELL_ARGV_DISPATCH_REVIEWS', 'SHELL_ASSIGNMENT_RE', 'SHELL_CONTROL_WORDS', 'SHELL_DEPENDENCY_COMMAND_REVIEWS', 'SHELL_DEPENDENCY_MANAGER_ACTIONS', 'SHELL_DYNAMIC_EXECUTORS', 'SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS', 'SHELL_EXPLICIT_DYNAMIC_REVIEWS', 'SHELL_INTERPRETERS', 'SHELL_REDIRECTION_RE', 'SHELL_SCAN_ROOTS', 'SHELL_SCRIPT_SUFFIXES', 'SHELL_SOURCE_INSPECTION_REVIEWS', 'SHELL_TRUSTED_SYSTEM_BIN_DIRS', 'SHELL_WRAPPERS', 'SHELL_WRAPPER_POSITIONAL_COUNTS', 'SHELL_WRAPPER_VALUE_OPTIONS', 'SPAWN_PATTERNS', 'TOOLS', 'UNSCANNED_PROCESS_SURFACES', '_TOOL_ALT', '_classify', '_const_array_spans', '_doc_block', '_enclosing_function', '_functions', '_inside_spans', '_is_external_harness_path', '_is_test_function', '_iter_files_with_suffixes', '_iter_python_files', '_iter_rust_files', '_iter_shell_files', '_line_excluded', '_mask_js_comments', '_mask_js_strings', '_mask_powershell_comments_and_strings', '_message_spans', '_portable_ast_dump', '_python_aliases_reviewed_namespace', '_python_ast_fingerprint_dump', '_python_binding_root', '_python_call_api', '_python_executable', '_python_process_helper_fingerprint', '_python_references_module_registry', '_shell_alias_variable', '_shell_arithmetic_writes_alias', '_shell_assignment_items', '_shell_bash_shebang_is_login', '_shell_command_aliases', '_shell_command_segments', '_shell_control_context', '_shell_dependency_lifecycle_action', '_shell_directory_assignment_paths', '_shell_environment_unset_before', '_shell_executable', '_shell_execution_substitution_bodies', '_shell_function_defined_before', '_shell_interpreter_script_target', '_shell_script_alias_mutation_index', '_shell_script_argument_targets', '_shell_static_script_aliases', '_shell_strip_comment', '_shell_trap_action_is_static', '_shell_trap_callback_mutated_after', 'check_lane_gate', 'main', 'review_python_process_calls', 'review_python_process_helpers', 'scan_file', 'scan_javascript_text', 'scan_powershell_text', 'scan_python_file', 'scan_python_text', 'scan_shell_file', 'scan_shell_text', 'scan_supplemental_file']
