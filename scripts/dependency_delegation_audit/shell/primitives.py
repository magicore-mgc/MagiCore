"""Primitives implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
import re

def _shell_strip_comment(line: str) -> str:
    """Remove shell comments without inspecting text inside quotes.
    (Bỏ comment shell nhưng không coi `#` trong chuỗi trích dẫn là comment.)"""
    quote = None
    escaped = False
    for index, char in enumerate(line):
        if escaped:
            escaped = False
            continue
        if char == "\\" and quote != "'":
            escaped = True
            continue
        if quote:
            if char == quote:
                quote = None
            continue
        if char in {"'", '"'}:
            quote = char
            continue
        if char == "#" and (index == 0 or line[index - 1].isspace()):
            return line[:index]
    return line

def _shell_assignment_items(text: str):
    """Yield one-line assignments after shell-aware comment removal.
    (Trả assignment một dòng sau khi bỏ comment theo quy tắc quote shell.)"""
    pattern = re.compile(r"^\s*(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*?)\s*$")
    for line in text.splitlines():
        match = pattern.match(_shell_strip_comment(line))
        if match:
            yield match.group(1), match.group(2)

def _shell_alias_variable(token: str) -> str:
    value = token.strip("\"'").removeprefix("$").strip("{}")
    return value.split("[", 1)[0]

def _shell_arithmetic_writes_alias(tokens: list, alias_names: set) -> bool:
    """Detect arithmetic writes to script-path aliases.
    (Phát hiện phép toán số học ghi đè alias đường dẫn script.)"""
    source = " ".join(tokens)
    expressions = re.findall(r"(?:\$\(\(|\(\()[\s\S]*?\)\)", source)
    for expression in expressions:
        for name in alias_names:
            identifier = re.escape(name)
            prefix_or_suffix_write = re.search(
                rf"(?<![A-Za-z0-9_])(?:\+\+|--)\s*{identifier}(?![A-Za-z0-9_])"
                rf"|(?<![A-Za-z0-9_]){identifier}(?![A-Za-z0-9_])\s*(?:\+\+|--)",
                expression,
            )
            assignment_write = re.search(
                rf"(?<![A-Za-z0-9_]){identifier}(?![A-Za-z0-9_])\s*"
                r"(?:\*\*|<<|>>|[+\-*/%&|^])?=(?!=)",
                expression,
            )
            if prefix_or_suffix_write or assignment_write:
                return True
    return False

def _shell_control_context(text: str, line_no: int) -> list:
    """Capture enclosing shell control lines for source-bound review.
    (Ghi lại guard shell đang bao quanh để review gắn với ngữ cảnh source.)"""
    stack = []
    for line in text.splitlines()[: max(0, line_no - 1)]:
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        head = re.match(r"(elif|else|fi|done|esac|if|for|while|until|case)\b", stripped)
        if not head:
            continue
        keyword = head.group(1)
        if keyword in {"fi", "done", "esac"}:
            if stack:
                stack.pop()
        elif keyword in {"elif", "else"}:
            if stack:
                stack[-1] = stripped
        else:
            stack.append(stripped)
    return stack

__all__ = ['_shell_strip_comment', '_shell_assignment_items', '_shell_alias_variable', '_shell_arithmetic_writes_alias', '_shell_control_context']
