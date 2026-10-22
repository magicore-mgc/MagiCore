"""Context implementation."""

from __future__ import annotations

import re

from ..rules.common import (
    MESSAGE_MACROS,
)

from ..rules.rust import (
    CONST_ARRAY_RE,
    FN_RE,
)

def _message_spans(text: str) -> list:
    """Paren spans of message-macro invocations (`format!(…)`, …).
    Heuristic by design: plain `"…"` strings (with backslash escapes) are
    tracked; raw strings (`r#"…"#`) are NOT — a raw string carrying
    unbalanced parens can only ever size a span wrong, and only the weak
    bare-literal pattern consults these spans, never the call-shaped ones.
    (Span ngoặc của lời gọi macro-message. Cố ý heuristic: chỉ theo dõi
    chuỗi thường (escape backslash); chuỗi raw không — literal raw mang
    ngoặc lệch chỉ làm sai kích thước span, mà chỉ pattern bare-literal
    yếu mới dùng các span này, pattern dạng-call không bao giờ dùng.)"""
    spans = []
    for macro in MESSAGE_MACROS:
        needle = macro + "("
        search_from = 0
        while True:
            call = text.find(needle, search_from)
            if call < 0:
                break
            depth = 0
            in_string = None
            escaped = False
            cursor = call + len(needle) - 1  # at the opening paren
            end = len(text)
            while cursor < len(text):
                char = text[cursor]
                if in_string is not None:
                    if escaped:
                        escaped = False
                    elif char == "\\":
                        escaped = True
                    elif char == in_string:
                        in_string = None
                elif char in ("\"", "'"):
                    in_string = char
                elif char == "(":
                    depth += 1
                elif char == ")":
                    depth -= 1
                    if depth <= 0:
                        end = cursor + 1
                        break
                cursor += 1
            spans.append((call, end))
            search_from = call + len(needle)
    return spans
def _inside_spans(spans: list, offset: int) -> bool:
    """True when the text offset sits inside any span.
    (True khi offset text nằm trong span bất kỳ.)"""
    return any(start <= offset < end for start, end in spans)
def _const_array_spans(lines: list) -> list:
    """(start_offset, end_offset) of multi-line top-level `&[&str]` const
    initializers, measured in text offsets.
    (Span các khởi tạo const array nhiều dòng top-level, đo bằng offset.)"""
    text = "\n".join(lines)
    line_starts = [0]
    for line in lines:
        line_starts.append(line_starts[-1] + len(line) + 1)
    spans = []
    idx = 0
    while idx < len(lines):
        if CONST_ARRAY_RE.match(lines[idx]):
            depth = 0
            cursor = idx
            started = False
            while cursor < len(lines):
                depth += lines[cursor].count("[") - lines[cursor].count("]")
                if "[" in lines[cursor]:
                    started = True
                if started and depth <= 0:
                    spans.append((line_starts[idx], line_starts[cursor + 1]))
                    break
                cursor += 1
            idx = cursor + 1
        else:
            idx += 1
    return spans
def _functions(lines: list) -> list:
    """Naive function spans: (name, start_idx, end_idx) via a brace walk.
    Heuristic by design — a Rust file whose string literals carry
    unbalanced braces can only ever make a span too long, never invent a
    function out of thin air.
    (Span hàm thô: (tên, dòng_đầu, dòng_cuối) qua đếm ngoặc. Cố ý heuristic
    — literal có ngoặc lệch chỉ làm span dài quá, không bao giờ sinh hàm
    từ hư không.)"""
    spans = []
    for idx, line in enumerate(lines):
        match = FN_RE.match(line)
        if not match:
            continue
        depth = 0
        started = False
        end = idx
        for cursor in range(idx, len(lines)):
            depth += lines[cursor].count("{") - lines[cursor].count("}")
            if "{" in lines[cursor]:
                started = True
            if started and depth <= 0:
                end = cursor
                break
        spans.append((match.group(1), idx, end))
    return spans
def _enclosing_function(spans: list, line_idx: int):
    """The INNERMOST function span containing the line, or None.
    (Span hàm TRONG CÙNG chứa dòng đó, hoặc None.)"""
    best = None
    for name, start, end in spans:
        if start <= line_idx <= end:
            if best is None or (end - start) < (best[2] - best[1]):
                best = (name, start, end)
    return best
def _is_test_function(lines: list, start_idx: int) -> bool:
    """Whether a function has a Rust test or test-only configuration attribute.
    (Xác định hàm có thuộc tính test Rust hoặc chỉ biên dịch khi test.)"""
    cursor = start_idx - 1
    while cursor >= 0:
        stripped = lines[cursor].strip()
        if not stripped or stripped.startswith("///") or stripped.startswith("//!"):
            cursor -= 1
            continue
        if stripped.startswith("#["):
            if re.match(r"#\[(?:tokio::)?test(?:\s*\(|\s*\])", stripped):
                return True
            if re.fullmatch(r"#\[cfg\(\s*test\s*\)\]", stripped):
                return True
            cursor -= 1
            continue
        break
    return False
def _is_external_harness_path(path_parts: list) -> bool:
    """Only non-`src` integration/bench roots get a path-level exemption.
    (Chỉ root integration/bench ngoài `src` mới được miễn theo đường dẫn.)"""
    for idx, segment in enumerate(path_parts):
        if segment in {"tests", "bench", "benches"} and "src" not in path_parts[:idx]:
            return True
    return False
def _doc_block(lines: list, start_idx: int) -> str:
    """Contiguous comment lines directly above a function start, skipping
    attribute lines (`#[allow(...)]`) that sit between the docs and the
    fn — they are part of the same declaration.
    (Các dòng chú thích liền trên điểm bắt đầu hàm, bỏ qua dòng attribute
    (`#[allow(...)]`) nằm giữa chú thích và fn — attribute thuộc cùng một
    khai báo.)"""
    collected = []
    cursor = start_idx - 1
    while cursor >= 0:
        stripped = lines[cursor].strip()
        if stripped.startswith("//"):
            collected.append(lines[cursor])
        elif stripped.startswith("#[") or stripped.startswith("#!"):
            pass  # attribute between doc and fn — keep walking (bỏ qua)
        else:
            break
        cursor -= 1
    return "\n".join(reversed(collected))

__all__ = ['_message_spans', '_inside_spans', '_const_array_spans', '_functions', '_enclosing_function', '_is_test_function', '_is_external_harness_path', '_doc_block']
