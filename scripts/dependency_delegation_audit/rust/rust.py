"""Rust implementation."""

from __future__ import annotations

import os
import re

from ..rules.common import (
    ALLOWED_OPS,
    DEPENDENCY_OPS,
    ECOSYSTEM_OWNED_OPS,
    MARKER,
    NON_EXECUTABLE_SENTINELS,
    NON_EXECUTABLE_SQL_PREFIXES,
)

from ..rules.rust import (
    AUDITED_DIRECT_PROCESS_ROUTES,
    AUDITED_EXECUTOR_ROUTES,
    DESCRIPTOR_PATTERNS,
    DIRECT_PROCESS_ROUTE_PATTERNS,
    DYNAMIC_SPAWN_PATTERNS,
    EXCLUDE_PATTERNS,
    EXECUTOR_ROUTE_PATTERNS,
    PLATFORM_PROCESS_BOUNDARY_REVIEWS,
    SPAWN_PATTERNS,
    TEST_ONLY_MODULE_REVIEWS,
)

from .context import (
    _const_array_spans,
    _doc_block,
    _enclosing_function,
    _functions,
    _inside_spans,
    _is_external_harness_path,
    _is_test_function,
    _message_spans,
)

# These exact error constructors contain a tool name, not an executable route.
# Các constructor lỗi chính xác này chứa tên tool, không phải route thực thi.
_TOOLCHAIN_ERROR_OWNERS = {
    ("cli/src/commands/build/build/ai/ai.rs", "build_ai"): "AiBuildError",
    ("cli/src/commands/build/build/iot/iot.rs", "build_iot"): "IotBuildError",
    ("cli/src/commands/build/build/cloud/cloud.rs", "build_cloud"): "CloudBuildError",
    ("cli/src/commands/build/build/lib/lib.rs", "build_lib"): "LibBuildError",
}


def _is_toolchain_error_metadata(text, start, end, rel_path, fn_name):
    """Exclude only the sole literal field of a reviewed missing-tool error.
    Chỉ loại field literal duy nhất của lỗi thiếu tool đã review.
    """
    owner = _TOOLCHAIN_ERROR_OWNERS.get((rel_path.replace(os.sep, "/"), fn_name))
    if owner is None:
        return False
    prefix = text[max(0, start - 120):start - 1]
    suffix = text[end + 1:end + 80]
    return bool(
        re.search(r"\bErr\(\s*" + re.escape(owner)
                  + r"::ToolchainMissing\s*\{\s*tool\s*:\s*$", prefix)
        and re.match(r"(?:\.to_string\(\))?\s*,?\s*\}", suffix)
    )


def _classify(rel_path: str, fn_name: str):
    """(op_class, status) for one finding.
    ((op_class, status) cho một finding.)"""
    parts = rel_path.replace(os.sep, "/").split("/")
    path_parts = [p.lower() for p in parts[:-1]]
    fn_lower = fn_name.lower()
    fn_parts = [p for p in fn_lower.split("_") if p]

    # Exact non-mutating OS/tool-version probes owned by `mgc doctor`.
    # This exception is path+symbol scoped; a directory name alone is not
    # enough to waive arbitrary external execution.
    # (Chỉ probe OS/version chính xác của doctor; không miễn cả directory.)
    if rel_path.replace(os.sep, "/") in {
        "cli/src/commands/doctor/toolchain/toolchain.rs",
        "cli/src/commands/doctor/storage/storage.rs",
    } and fn_name in {
        "tool_version",
        "fs_avail",
    }:
        return "doctor", "allowed"

    # Only integration/bench roots outside any `src` tree are path-exempt.
    # Singular `test` and source subdirectories named `tests` are not.
    # (Chỉ integration/bench root ngoài mọi cây `src` mới được miễn theo
    # path; `test` đơn lẻ và `src/tests` không được miễn.)
    if _is_external_harness_path(path_parts):
        return next(
            segment for segment in path_parts
            if segment in {"tests", "bench", "benches"}
        ), "allowed"

    # Function-level dependency scope wins over runtime/deploy paths:
    # `run_install` inside a dev module remains a dependency operation.
    # Conversely, `dev_command` inside an install module stays a runtime
    # boundary and is classified by its own function.
    # (Tên hàm lifecycle thắng path runtime/deploy; helper runtime riêng
    # trong module install vẫn theo tên hàm của chính nó.)
    for op in DEPENDENCY_OPS:
        if op in fn_parts or op in fn_lower:
            # A declaration records debt; it never grants dependency ownership.
            # Khai báo ghi nhận khoản nợ; không biến spawn ngoài thành native.
            return op, "violation"

    # A top-level spawn has no function context; a dependency path is the
    # conservative fallback, never a runtime exception.
    if fn_name == "<top-level>":
        for op in DEPENDENCY_OPS:
            if op in path_parts:
                return op, "violation"

    # Production workflow names do not grant permission to delegate that
    # workflow to an external package/build/runtime CLI. A test/bench path
    # was already classified above; all other matching tool spawns block.
    # (Tên workflow production không cấp quyền giao việc cho CLI ngoài;
    # test/bench đã được phân loại riêng phía trên.)
    for op in ECOSYSTEM_OWNED_OPS:
        if op in fn_parts or fn_lower.startswith(op):
            return op, "violation"

    # Diagnostic-only tool probes are non-mutating and may be reported as
    # such; this is not a native capability verdict.
    # (Probe doctor không mutation; đây không phải verdict native.)
    for op in ALLOWED_OPS:
        if op in fn_parts or fn_lower.startswith(op):
            return op, "allowed"

    # Unclassified spawn is a blocker; a marker or directory never grants
    # an exception.
    #    (Spawn chưa phân loại luôn bị chặn; marker không tạo ngoại lệ.)
    return "unclassified", "violation"
def _line_excluded(line: str) -> bool:
    """True when the physical line is a non-spawn context (comment, label).
    (True khi dòng vật lý là ngữ cảnh không-spawn (chú thích, nhãn).)"""
    return any(pattern.search(line) for pattern in EXCLUDE_PATTERNS)
def scan_file(rel_path: str, abs_path: str) -> list:
    """All findings in one file.
    (Mọi finding trong một file.)"""
    try:
        with open(abs_path, "r", encoding="utf-8", errors="replace") as handle:
            text = handle.read()
    except OSError as error:
        return [{
            "file": rel_path,
            "line": 1,
            "tool": "<unreadable-rust>",
            "evidence_kind": "scan_error",
            "op_class": "unscanned",
            "function": "<scan-error>",
            "status": "violation",
            "declared_delegation": False,
            "scan_error": str(error),
        }]
    lines = text.splitlines()
    spans = _functions(lines)
    message_macro_spans = _message_spans(text)
    const_array_spans = _const_array_spans(lines)
    findings = []

    all_patterns = SPAWN_PATTERNS + [
        (pattern, True) for pattern in DYNAMIC_SPAWN_PATTERNS
    ]
    for pattern, matches_inside_message in all_patterns:
        for match in pattern.finditer(text):
            if not matches_inside_message and _inside_spans(
                message_macro_spans, match.start(1)
            ):
                continue
            # Report/deduplicate at the executable literal, not the start
            # of a multi-line wrapper call (which may be on another line).
            line_no = text.count("\n", 0, match.start(1)) + 1
            span = _enclosing_function(spans, line_no - 1)
            if (
                not matches_inside_message
                and span is None
                and _inside_spans(const_array_spans, match.start(1))
            ):
                # Top-level deny/allow-list declaration, not a spawn.
                # (Khai báo deny/allow-list top-level, không phải spawn.)
                continue
            tool = match.group(1)
            if tool in NON_EXECUTABLE_SENTINELS:
                continue
            if (
                pattern in DESCRIPTOR_PATTERNS
                and tool.split(None, 1)[0].upper() in NON_EXECUTABLE_SQL_PREFIXES
            ):
                # SQL APIs commonly look like ("statement", []) and must
                # not become process-command descriptors.
                # (SQL API thường có dạng ("câu lệnh", []), không phải
                # descriptor để chạy process.)
                continue
            if pattern in DYNAMIC_SPAWN_PATTERNS:
                tool = f"<dynamic:{tool}>"
                evidence_kind = "dynamic_spawn_call"
            elif pattern in DESCRIPTOR_PATTERNS:
                evidence_kind = "command_descriptor"
            else:
                evidence_kind = "spawn_or_wrapper_call"
            line = lines[line_no - 1] if line_no <= len(lines) else ""
            if _line_excluded(line):
                continue
            span = _enclosing_function(spans, line_no - 1)
            if span is None:
                fn_name, fn_lines, doc_text = "<top-level>", [line], ""
            else:
                name, start, end = span
                fn_name = name
                fn_lines = lines[start:end + 1]
                doc_text = _doc_block(lines, start)
            # Metadata cannot authorize a process call in the same function.
            # Metadata không miễn process call khác trong cùng function.
            if evidence_kind == "command_descriptor" and _is_toolchain_error_metadata(
                text, match.start(1), match.end(1), rel_path, fn_name
            ):
                continue
            if span is not None and _is_test_function(lines, span[1]):
                op_class, status = "test-fixture", "allowed"
                classification_reason = None
            elif rel_path.replace(os.sep, "/") in TEST_ONLY_MODULE_REVIEWS:
                op_class, status = "test-fixture", "allowed"
                classification_reason = None
            else:
                classification_reason = PLATFORM_PROCESS_BOUNDARY_REVIEWS.get(
                    (rel_path.replace(os.sep, "/"), fn_name, tool)
                )
                if classification_reason is not None:
                    op_class, status = "platform-process-boundary", "review-required"
                else:
                    route_key = (rel_path.replace(os.sep, "/"), fn_name, tool)
                    if pattern in EXECUTOR_ROUTE_PATTERNS:
                        classification_reason = AUDITED_EXECUTOR_ROUTES.get(route_key)
                    elif pattern in DIRECT_PROCESS_ROUTE_PATTERNS:
                        classification_reason = AUDITED_DIRECT_PROCESS_ROUTES.get(route_key)
                    if classification_reason is not None:
                        op_class, status = "audited-executor-route", "allowed"
                    else:
                        op_class, status = _classify(rel_path, fn_name)
            # A literal command descriptor proves a route can select an
            # executable, not that this source line spawns it. Keep unknown
            # production routes release-blocking until linked to a launcher,
            # but do not report them as observed process executions.
            if evidence_kind == "command_descriptor" and status == "violation":
                status = "review-required"
            findings.append({
                "file": rel_path,
                "line": line_no,
                "tool": tool,
                "evidence_kind": evidence_kind,
                "op_class": op_class,
                "function": fn_name,
                "status": status,
                "blocking": status in {"violation", "review-required"},
                "classification_reason": classification_reason,
                "declared_delegation": MARKER in "\n".join(fn_lines) or MARKER in doc_text,
            })

    # Deduplicate: one finding per (file, line, tool) — a line matching two
    # shapes is still one spawn.
    # (Khử trùng: một finding mỗi (file, dòng, tool) — dòng khớp hai dạng
    # vẫn chỉ là một spawn.)
    unique = {}
    for finding in findings:
        key = (finding["file"], finding["line"], finding["tool"])
        unique.setdefault(key, finding)
    return sorted(unique.values(), key=lambda f: (f["file"], f["line"], f["tool"]))

__all__ = ['_classify', '_line_excluded', 'scan_file']
