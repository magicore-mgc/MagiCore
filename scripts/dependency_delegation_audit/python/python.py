"""Python implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
from ..rules.python import PYTHON_PROCESS_FUNCTIONS
from .process_calls import _python_call_api, _python_executable
import ast
import hashlib

def _portable_ast_dump(node):
    """Match the reviewed AST representation across Python versions.
    Giữ fingerprint chính xác khi Python 3.15 khác cách in list rỗng.
    """
    if isinstance(node, ast.AST):
        fields = [
            f"{name}={_portable_ast_dump(value)}"
            for name, value in ast.iter_fields(node)
            if value is not None and not (isinstance(value, list) and not value)
        ]
        return f"{type(node).__name__}({', '.join(fields)})"
    if isinstance(node, list):
        return f"[{', '.join(_portable_ast_dump(value) for value in node)}]"
    return repr(node)

def scan_python_text(rel_path: str, text: str) -> list:
    """Inventory known Python stdlib process calls; findings need review.
    (Kiểm kê lời gọi process stdlib đã biết; mọi finding cần review.)"""
    try:
        tree = ast.parse(text, filename=rel_path)
    except SyntaxError as error:
        return [{
            "file": rel_path,
            "line": error.lineno or 1,
            "api": "<parse-error>",
            "executable": "<unparsed-python>",
            "review_status": "unreviewed",
            "parse_error": error.msg,
        }]

    module_aliases = {}
    function_aliases = {}
    wildcard_imports = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            for alias in node.names:
                root_module = alias.name.split(".", 1)[0]
                bound_name = alias.asname or root_module
                module_aliases[bound_name] = root_module
        elif isinstance(node, ast.ImportFrom) and node.module:
            module = node.module.split(".", 1)[0]
            if module not in PYTHON_PROCESS_FUNCTIONS:
                continue
            for alias in node.names:
                if alias.name == "*":
                    wildcard_imports.append({
                        "file": rel_path,
                        "line": node.lineno,
                        "api": f"<wildcard-import:{module}>",
                        "executable": "<dynamic>",
                        "review_status": "unreviewed",
                        "parse_error": "wildcard import prevents resolving process API calls",
                    })
                    continue
                if alias.name in PYTHON_PROCESS_FUNCTIONS[module] or (
                    module == "os"
                    and alias.name.startswith(("exec", "spawn"))
                ):
                    function_aliases[alias.asname or alias.name] = f"{module}.{alias.name}"

    findings = list(wildcard_imports)
    for node in ast.walk(tree):
        if not isinstance(node, ast.Call):
            continue
        api = _python_call_api(node, module_aliases, function_aliases)
        if api is None:
            continue
        findings.append({
            "file": rel_path,
            "line": node.lineno,
            "api": api,
            "executable": _python_executable(node, api),
            "call_fingerprint": hashlib.sha256(
                _portable_ast_dump(node).encode("utf-8")
            ).hexdigest(),
            "review_status": "unreviewed",
            "review_reason": None,
            "parse_error": None,
        })
    return sorted(findings, key=lambda item: (item["line"], item["api"]))

__all__ = ['_portable_ast_dump', 'scan_python_text']
