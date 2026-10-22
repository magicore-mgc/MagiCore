"""Javascript implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
from ..rules.javascript import JS_PROCESS_APIS
import re

def _mask_js_comments(text: str) -> str:
    """Blank JS comments while preserving strings and physical line numbers.
    (Xóa comment JS nhưng giữ string và số dòng vật lý.)"""
    out = list(text)
    index = 0
    quote = None
    escaped = False
    while index < len(text):
        char = text[index]
        next_char = text[index + 1] if index + 1 < len(text) else ""
        if quote:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == quote:
                quote = None
            index += 1
            continue
        if char in ("'", '"', "`"):
            quote = char
            index += 1
        elif char == "/" and next_char == "/":
            while index < len(text) and text[index] != "\n":
                out[index] = " "
                index += 1
        elif char == "/" and next_char == "*":
            out[index] = out[index + 1] = " "
            index += 2
            while index < len(text):
                if text[index] == "\n":
                    index += 1
                    continue
                if text[index] == "*" and index + 1 < len(text) and text[index + 1] == "/":
                    out[index] = out[index + 1] = " "
                    index += 2
                    break
                out[index] = " "
                index += 1
        else:
            index += 1
    return "".join(out)

def _mask_js_strings(text: str) -> str:
    """Blank JS string/template bodies while preserving newlines and code.
    (Xóa nội dung string/template JS nhưng giữ newline và phần code.)"""
    out = list(text)
    index = 0
    quote = None
    escaped = False
    while index < len(text):
        char = text[index]
        if quote:
            if char == "\n" and quote != "`":
                quote = None
                index += 1
                continue
            if char != "\n":
                out[index] = " "
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == quote:
                quote = None
        elif char in ("'", '"', "`"):
            quote = char
            out[index] = " "
        index += 1
    return "".join(out)

def scan_javascript_text(rel_path: str, text: str) -> list:
    """Conservatively inventory common JS/Bun/Deno process APIs.
    (Kiểm kê bảo thủ API process JS/Bun/Deno thường gặp.)"""
    source = _mask_js_comments(text)
    code = _mask_js_strings(source)
    findings = []
    aliases = {}
    namespaces = set()
    import_re = re.compile(
        r"import\s*\{([^}]+)\}\s*from\s*(['\"])(?:node:)?child_process\2"
        r"|require\s*\(\s*(['\"])(?:node:)?child_process\3\s*\)"
    )
    for match in import_re.finditer(source):
        imported = match.group(1)
        if imported:
            for item in imported.split(","):
                parts = re.split(r"\s+as\s+", item.strip())
                name = parts[0].strip()
                alias = parts[-1].strip()
                if name in JS_PROCESS_APIS:
                    aliases[alias] = f"child_process.{name}"
        else:
            line_start = code.rfind("\n", 0, match.start()) + 1
            prefix = code[line_start:match.start()]
            assigned = re.search(r"(?:const|let|var)\s+(\w+)\s*=\s*$", prefix)
            if assigned:
                namespaces.add(assigned.group(1))
    commonjs_destructure = re.compile(
        r"(?:const|let|var)\s*\{([^}]+)\}\s*=\s*require\s*\(\s*(['\"])(?:node:)?child_process\2\s*\)"
    )
    for match in commonjs_destructure.finditer(source):
        for item in match.group(1).split(","):
            parts = re.split(r"\s*:\s*", item.strip())
            name = parts[0].strip()
            alias = parts[-1].strip()
            if name in JS_PROCESS_APIS:
                aliases[alias] = f"child_process.{name}"
    namespace_import = re.compile(
        r"import\s+(?:\*\s+as\s+)?([A-Za-z_$][\w$]*)\s+from\s*(['\"])(?:node:)?child_process\2"
    )
    for match in namespace_import.finditer(source):
        namespaces.add(match.group(1))

    dynamic_namespace_import = re.compile(
        r"(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=\s*(?:await\s*)?"
        r"import\s*\(\s*(['\"])(?:node:)?child_process\2\s*\)"
    )
    for match in dynamic_namespace_import.finditer(source):
        namespaces.add(match.group(1))

    dynamic_destructure_import = re.compile(
        r"(?:const|let|var)\s*\{([^}]+)\}\s*=\s*(?:await\s*)?"
        r"import\s*\(\s*(['\"])(?:node:)?child_process\2\s*\)"
    )
    for match in dynamic_destructure_import.finditer(source):
        for item in match.group(1).split(","):
            parts = re.split(r"\s*:\s*", item.strip(), maxsplit=1)
            imported = parts[0].strip()
            alias = parts[-1].strip()
            if imported in JS_PROCESS_APIS:
                aliases[alias] = f"child_process.{imported}"

    # Follow simple literal namespace copies, but do not claim arbitrary data-flow coverage.
    # Theo alias namespace dạng literal; không tuyên bố đã bao phủ data-flow tổng quát.
    namespace_alias = re.compile(
        r"\b(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=\s*([A-Za-z_$][\w$]*)\s*;?"
    )
    while True:
        added = {
            match.group(1)
            for match in namespace_alias.finditer(code)
            if match.group(2) in namespaces and match.group(1) not in namespaces
        }
        if not added:
            break
        namespaces.update(added)

    computed_calls = []
    for namespace in namespaces:
        computed_member = re.compile(
            rf"\b{re.escape(namespace)}\s*(?:\?\.|\.)?\s*"
            r"\[\s*(['\"])(" + "|".join(sorted(JS_PROCESS_APIS)) + r")\1\s*\]"
        )
        for match in computed_member.finditer(source):
            api_name = match.group(2)
            if re.match(r"\s*\(", code[match.end():]):
                computed_calls.append((match.start(), api_name))
            line_start = code.rfind("\n", 0, match.start()) + 1
            prefix = code[line_start:match.start()]
            assigned = re.search(r"(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=\s*$", prefix)
            if assigned:
                aliases[assigned.group(1)] = f"child_process.{api_name}"

    patterns = [(re.compile(rf"\b{re.escape(alias)}\s*\("), api) for alias, api in aliases.items()]
    for namespace in namespaces:
        patterns.extend(
            (
                re.compile(rf"\b{re.escape(namespace)}\s*(?:\?\.|\.)\s*{api}\s*\("),
                f"child_process.{api}",
            )
            for api in JS_PROCESS_APIS
        )
        patterns.extend(
            (
                re.compile(
                    rf"\b{re.escape(namespace)}\s*(?:\?\.|\.)?\s*"
                    rf"\[\s*(['\"]){api}\1\s*\]\s*\("
                ),
                f"child_process.{api}",
            )
            for api in JS_PROCESS_APIS
        )
    patterns.extend([
        (re.compile(r"\bBun\s*\.\s*spawn\s*\("), "Bun.spawn"),
        (re.compile(r"\bDeno\s*\.\s*(?:Command|run)\s*\("), "Deno.Command"),
        (re.compile(r"\beval\s*\("), "eval"),
        (re.compile(r"\bnew\s+Function\s*\("), "Function"),
    ])
    seen = set()
    inline_require = re.compile(
        r"require\s*\(\s*(['\"])(?:node:)?child_process\1\s*\)\s*\.\s*(" + "|".join(sorted(JS_PROCESS_APIS)) + r")\s*\("
    )
    for match in inline_require.finditer(source):
        line = source.count("\n", 0, match.start()) + 1
        api = f"child_process.{match.group(2)}"
        seen.add((line, api))
        findings.append({
            "file": rel_path,
            "line": line,
            "api": api,
            "executable": "<dynamic-or-unreviewed>",
            "review_status": "unreviewed",
        })
    for pattern, api in patterns:
        for match in pattern.finditer(code):
            line = code.count("\n", 0, match.start()) + 1
            key = (line, api)
            if key in seen:
                continue
            seen.add(key)
            findings.append({
                "file": rel_path,
                "line": line,
                "api": api,
                "executable": "<dynamic-or-unreviewed>",
                "review_status": "unreviewed",
            })
    for offset, api_name in computed_calls:
        line = source.count("\n", 0, offset) + 1
        key = (line, f"child_process.{api_name}")
        if key in seen:
            continue
        seen.add(key)
        findings.append({
            "file": rel_path,
            "line": line,
            "api": key[1],
            "executable": "<dynamic-or-unreviewed>",
            "review_status": "unreviewed",
        })
    return sorted(findings, key=lambda item: (item["line"], item["api"]))

__all__ = ['_mask_js_comments', '_mask_js_strings', 'scan_javascript_text']
