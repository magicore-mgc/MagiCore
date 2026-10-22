"""Commands implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
import re
import shlex

def _shell_command_segments(text: str, _line_offset: int = 0):
    """Yield logical shell command segments with their first physical line.
    (Tách command shell theo control operator, giữ dòng bắt đầu để audit.)"""
    pending = ""
    start_line = 1
    pending_continuation = False
    heredoc_delimiters = []
    case_states = []

    def split_tokens(tokens, line_number):
        segment = []
        in_double_bracket_test = False
        arithmetic_depth = 0
        for token in tokens:
            lowered = token.lower()
            if lowered == "case":
                case_states.append("selector")
                segment.append(token)
                continue
            if case_states and case_states[-1] == "selector":
                segment.append(token)
                if lowered == "in":
                    case_states[-1] = "pattern"
                    # A case selector is syntax, not an executable head.
                    # (Selector của case là cú pháp, không phải command.)
                    segment = []
                continue
            if case_states and case_states[-1] == "pattern":
                if lowered == "esac":
                    case_states.pop()
                    segment = []
                elif ")" in token:
                    case_states[-1] = "body"
                    segment = []
                continue
            if lowered == "esac" and case_states:
                if segment:
                    yield line_number, segment, None
                case_states.pop()
                segment = []
                continue
            if lowered == "case":
                case_states.append("selector")
                segment.append(token)
                continue
            if case_states and case_states[-1] == "body" and token in {";;", ";&", ";;&"}:
                if segment:
                    yield line_number, segment, None
                    segment = []
                case_states[-1] = "pattern"
                continue
            if token == "[[":
                in_double_bracket_test = True
                segment.append(token)
            elif token == "]]":
                in_double_bracket_test = False
                segment.append(token)
            elif arithmetic_depth or token.startswith("(("):
                arithmetic_depth += token.count("((") - token.count("))")
                segment.append(token)
            elif token and all(char in ";&|" for char in token) and not in_double_bracket_test:
                if segment:
                    yield line_number, segment, None
                    segment = []
            else:
                segment.append(token)
        if segment:
            yield line_number, segment, None

    heredoc_content = []
    heredoc_content_start_line = None
    for local_line_no, physical in enumerate(text.splitlines(), start=1):
        line_no = local_line_no + _line_offset
        if heredoc_delimiters:
            delimiter, strip_tabs, expandable = heredoc_delimiters[0]
            candidate = physical.lstrip("\t") if strip_tabs else physical
            if candidate.rstrip("\r\n") == delimiter:
                if expandable and heredoc_content:
                    body = "\n".join(heredoc_content)
                    for nested_source in _shell_execution_substitution_bodies(
                        body,
                        include_process_substitution=False,
                        recognize_comments=False,
                    ):
                        yield from _shell_command_segments(
                            nested_source,
                            (heredoc_content_start_line or line_no) - 1,
                        )
                heredoc_delimiters.pop(0)
                heredoc_content = []
                heredoc_content_start_line = None
            else:
                if heredoc_content_start_line is None:
                    heredoc_content_start_line = line_no
                heredoc_content.append(physical)
            continue
        if not pending:
            start_line = line_no
        stripped = physical.rstrip()
        continued = stripped.endswith("\\")
        pending_continuation = continued
        piece = stripped[:-1] if continued else physical
        pending += piece + (" " if continued else "\n")
        if continued:
            continue
        try:
            lexer = shlex.shlex(pending, posix=True, punctuation_chars=";&|")
            lexer.whitespace_split = True
            lexer.commenters = "#"
            tokens = list(lexer)
        except ValueError as error:
            if "No closing quotation" in str(error):
                # Shell single/double-quoted strings may legally span physical
                # lines; retain the source until shlex can parse the complete
                # command instead of turning valid multiline input red.
                # (Quote shell hợp lệ có thể qua nhiều dòng; giữ input đến khi
                # parse được cả command, không biến cú pháp đúng thành lỗi.)
                continue
            yield start_line, None, str(error)
            pending = ""
            continue
        yield from split_tokens(tokens, start_line)
        for nested_source in _shell_execution_substitution_bodies(pending):
            yield from _shell_command_segments(nested_source, start_line - 1)
        for strip_tabs, escaped_delimiter, quote, delimiter in re.findall(
            r"(?<!<)<<(?!<)(-)?\s*(?:\\([^\s;]+)|(['\"]?)([^\s'\";]+)\3)",
            pending,
        ):
            heredoc_delimiters.append((
                escaped_delimiter or delimiter,
                bool(strip_tabs),
                not escaped_delimiter and not quote,
            ))
        pending = ""
        pending_continuation = False
    if heredoc_delimiters:
        yield line_no if text.splitlines() else 1, None, "unterminated here-document"
        return
    if pending:
        if pending_continuation:
            yield start_line, None, "line continuation reaches end of file"
            return
        try:
            lexer = shlex.shlex(pending, posix=True, punctuation_chars=";&|")
            lexer.whitespace_split = True
            lexer.commenters = "#"
            tokens = list(lexer)
        except ValueError as error:
            yield start_line, None, str(error)
            return
        yield from split_tokens(tokens, start_line)
        for nested_source in _shell_execution_substitution_bodies(pending):
            yield from _shell_command_segments(nested_source, start_line - 1)

def _shell_execution_substitution_bodies(
    value: str,
    *,
    include_process_substitution: bool = True,
    recognize_comments: bool = True,
) -> list:
    """Extract executable shell substitutions from raw command text.
    (Tách command substitution đang thực thi từ shell source thô.)"""
    bodies = []
    index = 0
    quote = None
    escaped = False
    while index < len(value):
        char = value[index]
        if escaped:
            escaped = False
            index += 1
            continue
        if char == "\\" and quote != "'":
            escaped = True
            index += 1
            continue
        if quote == "'":
            if char == "'":
                quote = None
            index += 1
            continue
        if recognize_comments and quote is None and char == "#" and (
            index == 0 or value[index - 1].isspace()
        ):
            newline = value.find("\n", index)
            if newline < 0:
                break
            index = newline + 1
            continue
        if quote is None and char in {"'", '"'}:
            quote = char
            index += 1
            continue
        if quote == '"' and char == '"':
            quote = None
            index += 1
            continue
        if value.startswith("$((", index):
            # Arithmetic expansion is not a command substitution; keep scanning
            # its body so nested executable `$()` calls remain visible.
            # (Arithmetic expansion không phải command substitution; vẫn rà
            # phần thân để thấy `$()` executable lồng bên trong.)
            index += 3
            continue

        marker = None
        if value.startswith("$(", index):
            marker = "$"
        elif include_process_substitution and quote is None and value.startswith("<(", index):
            marker = "<"
        elif include_process_substitution and quote is None and value.startswith(">(", index):
            marker = ">"
        if marker is not None:
            start = index + 2
            cursor = start
            depth = 1
            nested_quote = None
            escaped = False
            while cursor < len(value):
                nested_char = value[cursor]
                if escaped:
                    escaped = False
                    cursor += 1
                    continue
                if nested_char == "\\" and nested_quote != "'":
                    escaped = True
                    cursor += 1
                    continue
                if nested_quote == "'":
                    if nested_char == "'":
                        nested_quote = None
                    cursor += 1
                    continue
                if nested_quote is None and nested_char in {"'", '"'}:
                    nested_quote = nested_char
                    cursor += 1
                    continue
                if nested_quote == '"' and nested_char == '"':
                    nested_quote = None
                    cursor += 1
                    continue
                if recognize_comments and nested_quote is None and nested_char == "#" and (
                    cursor == 0 or value[cursor - 1].isspace()
                ):
                    newline = value.find("\n", cursor)
                    if newline < 0:
                        cursor = len(value)
                        break
                    cursor = newline + 1
                    continue
                nested_prefixes = ("$(",) if nested_quote or not include_process_substitution else ("$(", "<(", ">(")
                if any(value.startswith(prefix, cursor) for prefix in nested_prefixes):
                    depth += 1
                    cursor += 2
                    continue
                if nested_quote is None and nested_char == "(":
                    depth += 1
                elif nested_quote is None and nested_char == ")":
                    depth -= 1
                    if depth == 0:
                        bodies.append(value[start:cursor])
                        index = cursor + 1
                        break
                cursor += 1
            else:
                index += len(marker) + 1
            continue
        if char == "`":
            start = index + 1
            cursor = start
            escaped = False
            while cursor < len(value):
                if escaped:
                    escaped = False
                elif value[cursor] == "\\":
                    escaped = True
                elif value[cursor] == "`":
                    bodies.append(value[start:cursor])
                    index = cursor + 1
                    break
                cursor += 1
            else:
                index += 1
            continue
        index += 1
    return bodies

__all__ = ['_shell_command_segments', '_shell_execution_substitution_bodies']
