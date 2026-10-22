"""Bounded command diagnostics and GitHub annotations."""

from __future__ import annotations

import re

def github_actions_annotation_escape(value: object) -> str:
    """Escape untrusted text before writing a GitHub workflow command.
    (Escape text không tin cậy trước khi ghi thành workflow command GitHub.)
    """
    return str(value).replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")

def github_actions_error_annotations(failures: list[str]) -> list[str]:
    """Format at most ten actionable failure annotations for one step.
    (Tạo tối đa mười annotation lỗi có ích cho một step.)
    """
    if len(failures) > 10:
        messages = failures[:9] + [
            f"{len(failures) - 9} additional gate failures; full details are in this step log"
        ]
    else:
        messages = failures
    return [
        "::error::" + github_actions_annotation_escape(message)
        for message in messages
    ]

COMMAND_OUTPUT_EXCERPT_LIMIT = 2000

def command_output_excerpt(output: str, limit: int = COMMAND_OUTPUT_EXCERPT_LIMIT) -> str:
    """Keep a bounded head, failure context, and tail from command output.
    (Giữ phần đầu, ngữ cảnh lỗi và phần cuối trong giới hạn của output lệnh.)"""
    if limit <= 0:
        return ""
    text = output or ""
    if len(text) <= limit:
        return text
    if limit <= 32:
        head_size = (limit + 1) // 2
        tail_size = limit - head_size
        return text[:head_size] + (text[-tail_size:] if tail_size else "")

    diagnostic = re.compile(
        r"\b(error|exception|caused by|fatal|failed|failure|unable|cannot|could not|"
        r"no such file|not found|timed out|assertionerror|nosuchmethoderror|"
        r"processexception)\b",
        re.IGNORECASE,
    )
    wrapper = re.compile(
        r"\b(test runner|build runner|exited with code|process completed with exit code|"
        r"command failed|lifecycle step failed)\b",
        re.IGNORECASE,
    )
    lines = text.splitlines()
    matching = [
        index for index, line in enumerate(lines)
        if diagnostic.search(line) and not wrapper.search(line)
    ]
    selected: list[str] = []
    if matching:
        indexes = set()
        for index in (*matching[:3], *matching[-3:]):
            indexes.update(range(max(0, index - 1), min(len(lines), index + 2)))
        selected = [lines[index] for index in sorted(indexes)]

    head_budget = min(400, limit // 4)
    tail_budget = min(400, limit // 4)
    failure_marker = "\n...[failure context]...\n"
    tail_marker = "\n...[tail]...\n"
    context_budget = max(
        0,
        limit - head_budget - tail_budget - len(tail_marker)
        - (len(failure_marker) if selected else 0),
    )
    head = text[:head_budget]
    tail = text[-tail_budget:]
    context = "\n".join(selected)[:context_budget]

    excerpt = head
    if context:
        excerpt += failure_marker + context
    return excerpt + tail_marker + tail

def lifecycle_failure_excerpt(lane: dict, output: str) -> str:
    """Expand diagnostics only for the two lanes with a confirmed CI failure.
    (Chỉ mở rộng log cho hai lane đang có bằng chứng lỗi CI.)"""
    if (lane.get("core"), lane.get("language")) in {
        ("app", "flutter"),
        ("clo", "pulumi"),
    }:
        return command_output_excerpt(output)
    return (output or "")[-COMMAND_OUTPUT_EXCERPT_LIMIT:]

__all__ = ['COMMAND_OUTPUT_EXCERPT_LIMIT', 'command_output_excerpt', 'github_actions_annotation_escape', 'github_actions_error_annotations', 'lifecycle_failure_excerpt']
