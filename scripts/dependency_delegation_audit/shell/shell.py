"""Shell implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
from ..rules.shell import SHELL_ARGV_DISPATCH_REVIEWS, SHELL_DEPENDENCY_COMMAND_REVIEWS, SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS, SHELL_EXPLICIT_DYNAMIC_REVIEWS, SHELL_INTERPRETERS, SHELL_SOURCE_DYNAMIC_COMMAND_REVIEWS, SHELL_SOURCE_INSPECTION_REVIEWS
from .aliases import _shell_command_aliases, _shell_script_argument_targets, _shell_static_script_aliases
from .alias_mutation import _shell_script_alias_mutation_index
from .commands import _shell_command_segments
from .execution import _shell_bash_shebang_is_login, _shell_dependency_lifecycle_action, _shell_environment_unset_before, _shell_executable, _shell_interpreter_script_target
from .primitives import _shell_control_context
from .traps import _shell_trap_action_is_static
import hashlib
import json
import re

def scan_shell_text(rel_path: str, text: str, scanned_shell_paths=None) -> list:
    """Inventory direct known-tool shell commands; malformed input is unreviewed.
    (Kiểm kê command gọi tool đã biết; cú pháp lỗi không được xem là sạch.)"""
    findings = []
    seen_dependency_fingerprints = set()
    seen_exact_dynamic_fingerprints = set()
    command_aliases = _shell_command_aliases(text)
    static_script_aliases = _shell_static_script_aliases(text, rel_path)
    command_segments = list(_shell_command_segments(text))
    alias_mutation_index = _shell_script_alias_mutation_index(
        command_segments, static_script_aliases, rel_path, text
    )
    scanned_shell_sources = (
        scanned_shell_paths if isinstance(scanned_shell_paths, dict) else {}
    )
    scanned_shell_paths = set(scanned_shell_paths or ())
    for segment_index, (line_no, segment, parse_error) in enumerate(command_segments):
        active_script_aliases = (
            static_script_aliases
            if alias_mutation_index is None or segment_index <= alias_mutation_index
            else {}
        )
        if parse_error is not None:
            # An unreviewed unparsed segment stays blocking, but a source line
            # an explicit inspection review already reviewed (SHELL_SOURCE_
            # INSPECTION_REVIEWS) is exempt: the parser could not segment it,
            # yet a human has audited that exact line. Without this an
            # unparsed segment could never be cleared by any review.
            # (Đoạn không parse được vẫn chặn nếu chưa review; nếu dòng nguồn
            # đã có review kiểm tra tường minh thì được miễn — parser không tách
            # được nhưng người đã soi đúng dòng đó.)
            unparsed_source_line = text.splitlines()[line_no - 1].strip()
            if (
                rel_path in SHELL_SOURCE_INSPECTION_REVIEWS
                and unparsed_source_line in SHELL_SOURCE_INSPECTION_REVIEWS[rel_path]
            ):
                continue
            findings.append({
                "file": rel_path,
                "line": line_no,
                "api": "shell-command",
                "executable": "<unparsed-shell>",
                "review_status": "unreviewed",
                "parse_error": parse_error,
            })
            continue
        source_line = text.splitlines()[line_no - 1].strip()
        if (
            rel_path in SHELL_ARGV_DISPATCH_REVIEWS
            and source_line in SHELL_ARGV_DISPATCH_REVIEWS[rel_path]
            and any("$@" in token for token in segment)
        ):
            # These test helpers execute a quoted argv vector supplied by
            # literal call sites; they never reconstruct or re-evaluate shell.
            # (Helper test chạy argv được truyền trực tiếp; không ghép/eval shell.)
            findings.append({
                "file": rel_path,
                "line": line_no,
                "api": "shell-command",
                "executable": "<reviewed-argv-dispatch>",
                "review_status": "reviewed",
                "review_reason": "quoted argv dispatch is restricted to literal test call sites; no shell re-evaluation",
                "parse_error": None,
            })
            continue
        reviewed_dynamic = (
            rel_path in SHELL_EXPLICIT_DYNAMIC_REVIEWS
            and source_line in SHELL_EXPLICIT_DYNAMIC_REVIEWS[rel_path]
            and "$MGC_PATH" in segment
        )
        reviewed_source_inspection = (
            rel_path in SHELL_SOURCE_INSPECTION_REVIEWS
            and source_line in SHELL_SOURCE_INSPECTION_REVIEWS[rel_path]
        )
        reviewed_exact_shell = reviewed_dynamic or reviewed_source_inspection
        if segment and segment[0].replace("\\", "/").rsplit("/", 1)[-1] == "trap":
            executable = (
                "shell-trap-static"
                if _shell_trap_action_is_static(text, line_no, segment)
                else "<dynamic-shell-command>"
            )
        elif reviewed_exact_shell:
            executable = "<reviewed-dynamic-command>"
        else:
            executable = _shell_executable(
                segment,
                command_aliases,
                active_script_aliases,
                scanned_shell_paths,
            )
        if executable in SHELL_INTERPRETERS:
            targets = _shell_interpreter_script_target(
                segment, executable, active_script_aliases
            )
            verified_targets = targets.intersection(scanned_shell_paths)
            interpreter_index = next(
                (
                    index
                    for index, token in enumerate(segment)
                    if token.replace("\\", "/").rsplit("/", 1)[-1].lower() == executable
                ),
                len(segment),
            )
            startup_safe = True
            if executable == "bash":
                startup_safe = _shell_environment_unset_before(
                    segment, "BASH_ENV", interpreter_index
                )
            elif executable == "zsh":
                startup_safe = any(
                    re.fullmatch(r"-[A-Za-z]*f[A-Za-z]*", token)
                    for token in segment[interpreter_index + 1 :]
                )
            else:
                startup_safe = False
            executable = (
                "shell-script"
                if startup_safe and len(verified_targets) == 1 and len(targets) == 1
                else "<dynamic-shell-command>"
            )
        elif executable == "shell-script" and scanned_shell_sources:
            direct_targets = [
                (index, targets)
                for index, token in enumerate(segment)
                if (targets := _shell_script_argument_targets(token, active_script_aliases))
            ]
            script_index, target_paths = direct_targets[0] if direct_targets else (None, set())
            if len(direct_targets) != 1 or len(target_paths) != 1:
                executable = "<dynamic-shell-command>"
            else:
                target_source = scanned_shell_sources.get(next(iter(target_paths)), "")
                shebang = target_source.splitlines()[0].lower() if target_source else ""
                if "bash" in shebang:
                    if (
                        _shell_bash_shebang_is_login(shebang)
                        or not _shell_environment_unset_before(
                            segment, "BASH_ENV", script_index
                        )
                    ):
                        executable = "<dynamic-shell-command>"
                elif "zsh" in shebang:
                    executable = "<dynamic-shell-command>"
        if executable is None:
            continue
        dynamic = executable in {"<dynamic-shell-command>", "<unparsed-shell>"}
        dependency_action = (
            _shell_dependency_lifecycle_action(executable, segment, command_aliases)
            if not dynamic
            else None
        )
        fingerprint_payload = json.dumps(
            {
                "file": rel_path,
                "line": line_no,
                "source_sha256": hashlib.sha256(text.encode("utf-8")).hexdigest(),
                "control_context": _shell_control_context(text, line_no),
                "segment": segment,
            },
            ensure_ascii=False,
            separators=(",", ":"),
        )
        fingerprint = hashlib.sha256(fingerprint_payload.encode("utf-8")).hexdigest()
        exact_dynamic_review = SHELL_EXACT_DYNAMIC_COMMAND_REVIEWS.get(
            rel_path, {}
        ).get(fingerprint)
        source_review = SHELL_SOURCE_DYNAMIC_COMMAND_REVIEWS.get(rel_path, {})
        source_review_reason = (
            source_review.get("lines", {}).get(line_no)
            if source_review.get("source_sha256")
            == hashlib.sha256(text.encode("utf-8")).hexdigest()
            else None
        )
        reviewed_exact_dynamic = (
            dynamic
            and (exact_dynamic_review is not None or source_review_reason is not None)
            and fingerprint not in seen_exact_dynamic_fingerprints
        )
        if exact_dynamic_review is not None:
            seen_exact_dynamic_fingerprints.add(fingerprint)
        if dependency_action is not None:
            reviewed = (
                fingerprint in SHELL_DEPENDENCY_COMMAND_REVIEWS.get(rel_path, set())
                and fingerprint not in seen_dependency_fingerprints
            )
            seen_dependency_fingerprints.add(fingerprint)
            review_status = "reviewed" if reviewed else "unreviewed"
            review_reason = (
                f"exact dependency lifecycle command reviewed: {dependency_action}"
                if reviewed
                else f"dependency lifecycle command requires exact source review: {dependency_action}"
            )
        elif reviewed_exact_dynamic:
            review_status = "reviewed"
            review_reason = (
                exact_dynamic_review[0]
                if exact_dynamic_review is not None
                else source_review_reason
            )
        else:
            if reviewed_exact_shell:
                review_status = "reviewed"
                if reviewed_dynamic:
                    review_reason = (
                        "exact version/help argv is reviewed at the shell boundary; caller-selected binary behavior is outside this inventory"
                    )
                else:
                    review_reason = (
                        "exact source-inspection command reviewed; fixed find/grep pipeline inspects repository source without executing discovered names"
                    )
            else:
                review_status = "unreviewed" if dynamic else "observed"
                review_reason = (
                    None
                    if dynamic
                    else "literal process command is inventoried; no dependency lifecycle action matched"
                )
        findings.append({
            "file": rel_path,
            "line": line_no,
            "api": "shell-command",
            "executable": executable,
            "review_status": review_status,
            "review_reason": review_reason,
            "dependency_action": dependency_action,
            "command_fingerprint": fingerprint,
            "parse_error": None,
        })
    return findings

__all__ = ['scan_shell_text']
