#!/usr/bin/env python3
"""Reject dispatch inputs expanded directly inside workflow shell blocks.
Từ chối input dispatch được nội suy thẳng vào khối shell của workflow.
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


RUN_KEY = re.compile(r"^(?P<indent> *)(?:-\s+)?run:\s*(?P<body>.*)$")
FLOW_MAPPING_RUN = re.compile(r"^[ \t]*-\s*\{.*\brun\s*:")
BLOCK_SCALAR = re.compile(
    r"^[|>](?:(?:[1-9][+-]?)|(?:[+-][1-9]?))?(?:\s+#.*)?$"
)
UNTRUSTED_INPUT = re.compile(
    r"\$\{\{[^}]*\b(?:inputs|github\.event\.inputs)\b[^}]*\}\}"
)


@dataclass(frozen=True)
class Finding:
    path: Path
    line: int
    expression: str


def _iter_run_bodies(lines: list[str]):
    """Yield start line and source text for each YAML ``run`` value."""
    index = 0
    while index < len(lines):
        match = RUN_KEY.match(lines[index])
        if match is None:
            index += 1
            continue

        body = match.group("body").strip()
        base_indent = len(match.group("indent"))
        if BLOCK_SCALAR.fullmatch(body):
            cursor = index + 1
            body_lines: list[str] = []
            while cursor < len(lines):
                candidate = lines[cursor]
                if not candidate.strip():
                    body_lines.append(candidate)
                    cursor += 1
                    continue

                indent = len(candidate) - len(candidate.lstrip(" "))
                if indent <= base_indent:
                    break
                body_lines.append(candidate)
                cursor += 1
            yield index + 2, "\n".join(body_lines)
            index = cursor
            continue

        yield index + 1, body
        index += 1


def audit_text(text: str, path: Path | str) -> list[Finding]:
    """Find untrusted workflow-dispatch expressions in YAML shell commands."""
    source_path = Path(path)
    findings: list[Finding] = []
    lines = text.splitlines()
    for body_start_line, source_body in _iter_run_bodies(lines):
        for match in UNTRUSTED_INPUT.finditer(source_body):
            line_number = body_start_line + source_body.count("\n", 0, match.start())
            findings.append(
                Finding(source_path, line_number, match.group(0))
            )
    for line_number, source_line in enumerate(lines, start=1):
        if FLOW_MAPPING_RUN.search(source_line):
            for match in UNTRUSTED_INPUT.finditer(source_line):
                findings.append(
                    Finding(source_path, line_number, match.group(0))
                )
    return findings


def audit_workflows(root: Path) -> list[Finding]:
    """Audit every workflow YAML file below ``root`` in stable order."""
    workflow_paths = sorted(set(root.rglob("*.yml")) | set(root.rglob("*.yaml")))
    findings: list[Finding] = []
    for workflow_path in workflow_paths:
        try:
            text = workflow_path.read_text(encoding="utf-8")
        except OSError as error:
            findings.append(Finding(workflow_path, 1, f"could not read workflow: {error}"))
            continue
        display_path = workflow_path.relative_to(root)
        findings.extend(audit_text(text, display_path))
    return findings


def main() -> int:
    parser = argparse.ArgumentParser(description="Audit workflow shell input interpolation")
    parser.add_argument(
        "workflows_dir",
        nargs="?",
        type=Path,
        default=Path(__file__).resolve().parents[1] / ".github" / "workflows",
        help="directory containing GitHub Actions workflow YAML files",
    )
    args = parser.parse_args()

    if not args.workflows_dir.is_dir():
        print(f"ERROR: workflow directory does not exist: {args.workflows_dir}", file=sys.stderr)
        return 2

    findings = audit_workflows(args.workflows_dir)
    if findings:
        for finding in findings:
            print(
                "ERROR: untrusted workflow input is interpolated in a shell run block: "
                f"{finding.path}:{finding.line}: {finding.expression}; pass it through step env",
                file=sys.stderr,
            )
        return 1

    print("PASS: workflow dispatch inputs are not interpolated in shell run blocks")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
