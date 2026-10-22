"""Powershell implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
import hashlib
from ..rules.powershell import POWERSHELL_SOURCE_PROCESS_REVIEWS
from ..rules.common import TOOLS
from ..rules.powershell import PS_PROCESS_ALIASES, PS_PROCESS_APIS
import re

def _mask_powershell_comments_and_strings(line: str) -> str:
    """Blank single/double-quoted values and trailing PowerShell comments.
    (Xóa chuỗi và comment cuối dòng PowerShell để tránh match ví dụ.)"""
    out = list(line)
    quote = None
    index = 0
    while index < len(line):
        char = line[index]
        if quote:
            out[index] = " "
            if char == quote:
                if quote == "'" and index + 1 < len(line) and line[index + 1] == "'":
                    out[index + 1] = " "
                    index += 2
                    continue
                if index == 0 or line[index - 1] != "`":
                    quote = None
        elif char in ("'", '"'):
            quote = char
            out[index] = " "
        elif char == "#":
            for pos in range(index, len(line)):
                out[pos] = " "
            break
        index += 1
    return "".join(out)

def scan_powershell_text(rel_path: str, text: str) -> list:
    """Inventory PowerShell process operators and known native command heads.
    (Kiểm kê toán tử process PowerShell và command native đã biết.)"""
    findings = []
    source_review = POWERSHELL_SOURCE_PROCESS_REVIEWS.get(rel_path, {})
    source_hash_matches = source_review.get("source_sha256") == hashlib.sha256(
        text.encode("utf-8")
    ).hexdigest()
    tool_alt = "|".join(re.escape(tool) for tool in sorted(TOOLS, key=len, reverse=True))
    native_re = re.compile(rf"(?:^|[|;&{{]\s*)(?:[\w./\\:-]+/)?({tool_alt})(?=\s|$)", re.IGNORECASE)
    for line_no, line in enumerate(text.splitlines(), start=1):
        code = _mask_powershell_comments_and_strings(line)
        matches = []
        for api in PS_PROCESS_APIS:
            if re.search(rf"\b{re.escape(api)}\b", code, re.IGNORECASE):
                matches.append((api, api))
        alias_alt = "|".join(re.escape(alias) for alias in sorted(PS_PROCESS_ALIASES, key=len, reverse=True))
        for match in re.finditer(
            rf"(?:^|[|;&{{}}]\s*)({alias_alt})(?=\s|$|[|;&{{}}])",
            code,
            re.IGNORECASE,
        ):
            alias = match.group(1).lower()
            matches.append(("process-alias", PS_PROCESS_ALIASES[alias]))
        if re.search(r"(?:^|\s)&\s*(?:\$|\(|\{)", code):
            matches.append(("call-operator", "<dynamic>"))
        quoted_tool = re.search(
            rf"^\s*&\s*(['\"])(?:[A-Za-z]:[\\/])?(?:[^'\"]*[\\/])?({tool_alt})\1(?=\s|$)",
            line,
            re.IGNORECASE,
        )
        if quoted_tool:
            matches.append(("call-operator", quoted_tool.group(2).lower()))
        if re.search(r"\b(?:cmd|pwsh|powershell)\s+(?:/c|-Command)\b", code, re.IGNORECASE):
            matches.append(("shell-command", "<dynamic>"))
        for match in native_re.finditer(code):
            matches.append(("native-command", match.group(1).lower()))
        for api, executable in sorted(set(matches)):
            review_reason = (
                source_review.get("lines", {}).get(line_no)
                if source_hash_matches
                else None
            )
            findings.append({
                "file": rel_path,
                "line": line_no,
                "api": api,
                "executable": executable,
                "review_status": "reviewed" if review_reason else "unreviewed",
                "review_reason": review_reason,
            })
    return findings

__all__ = ['_mask_powershell_comments_and_strings', 'scan_powershell_text']
