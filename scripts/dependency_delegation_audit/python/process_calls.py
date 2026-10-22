"""Process Calls implementation.

Each function keeps its source behavior and reviewed fail-closed rules.
"""

from __future__ import annotations
from ..rules.python import PYTHON_PROCESS_FUNCTIONS, PYTHON_PROCESS_REVIEWS
import ast

def _python_call_api(node: ast.Call, module_aliases: dict, function_aliases: dict):
    """Resolve known Python stdlib process API calls through import aliases.
    (Phân giải lời gọi API process stdlib qua alias import.)"""
    func = node.func
    if isinstance(func, ast.Attribute) and isinstance(func.value, ast.Name):
        module = module_aliases.get(func.value.id)
        if module in PYTHON_PROCESS_FUNCTIONS:
            name = func.attr
            if name in PYTHON_PROCESS_FUNCTIONS[module]:
                return f"{module}.{name}"
            if module == "os" and name.startswith(("exec", "spawn")):
                return f"os.{name}"
    if isinstance(func, ast.Name):
        return function_aliases.get(func.id)
    return None

def _python_executable(node: ast.Call, api: str) -> str:
    """Extract a static argv head when safe; preserve dynamic calls explicitly.
    (Lấy argv tĩnh khi an toàn; giữ nguyên nhãn cho lệnh động.)"""
    if not node.args:
        return "<dynamic>"
    try:
        argument = ast.literal_eval(node.args[0])
    except (ValueError, TypeError, SyntaxError):
        return "<dynamic>"
    if isinstance(argument, (list, tuple)):
        if not argument:
            return "<dynamic>"
        argument = argument[0]
    if not isinstance(argument, str):
        return "<dynamic>"
    shell_string = api in {
        "os.system", "os.popen", "subprocess.getoutput", "subprocess.getstatusoutput",
        "asyncio.create_subprocess_shell",
    } or any(
        keyword.arg == "shell"
        and isinstance(keyword.value, ast.Constant)
        and keyword.value.value is True
        for keyword in node.keywords
    )
    if shell_string:
        return "<shell-command>"
    return argument

def review_python_process_calls(calls: list, expected_files=None) -> list:
    """Apply exact AST review records and report stale/missing records.
    Áp dụng bản ghi review khớp AST chính xác; báo mục review cũ/bị thiếu.
    """
    expected_paths = (
        set(expected_files)
        if expected_files is not None
        else set(PYTHON_PROCESS_REVIEWS)
    )
    actual_counts = {}
    for item in calls:
        fingerprint = item.get("call_fingerprint")
        if fingerprint:
            key = (item["file"], item["api"], fingerprint)
            actual_counts[key] = actual_counts.get(key, 0) + 1

    seen_counts = {}
    reviewed = []
    for original in calls:
        item = dict(original)
        fingerprint = item.get("call_fingerprint")
        key = (item["file"], item["api"], fingerprint)
        path_reviews = PYTHON_PROCESS_REVIEWS.get(item["file"], {})
        review = path_reviews.get((item["api"], fingerprint))
        seen_counts[key] = seen_counts.get(key, 0) + 1
        if review and seen_counts[key] <= review[0]:
            item["review_status"] = "reviewed"
            item["review_reason"] = review[1]
        else:
            item["review_status"] = "unreviewed"
            item["review_reason"] = None
        reviewed.append(item)

    # A removed or changed reviewed call must not leave a stale approval in
    # the source allowlist; surface it as a blocking inventory record.
    for rel_path in sorted(expected_paths & PYTHON_PROCESS_REVIEWS.keys()):
        for (api, fingerprint), (expected_count, reason) in PYTHON_PROCESS_REVIEWS[rel_path].items():
            actual_count = actual_counts.get((rel_path, api, fingerprint), 0)
            if actual_count < expected_count:
                reviewed.append({
                    "file": rel_path,
                    "line": 0,
                    "api": api,
                    "executable": "<missing-reviewed-call>",
                    "call_fingerprint": fingerprint,
                    "review_status": "stale-review",
                    "review_reason": reason,
                    "parse_error": f"expected {expected_count} exact call(s), found {actual_count}",
                })
    return sorted(reviewed, key=lambda item: (item["file"], item["line"], item["api"]))

__all__ = ['_python_call_api', '_python_executable', 'review_python_process_calls']
