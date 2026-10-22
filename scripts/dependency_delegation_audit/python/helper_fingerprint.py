"""Python helper-review primitives.

AST fingerprints and reviewed module-namespace resolution are separate owners.
"""

from __future__ import annotations

import ast
import hashlib

def _python_binding_root(node):
    while isinstance(node, (ast.Attribute, ast.Subscript)):
        node = node.value
    return node.id if isinstance(node, ast.Name) else None

def _python_ast_fingerprint_dump(value):
    if isinstance(value, ast.AST):
        field_names = list(value._fields)
        if hasattr(value, "type_params") and "type_params" not in field_names:
            field_names.append("type_params")
        fields = []
        for field_name in field_names:
            field_value = getattr(value, field_name, None)
            # Empty type_params appears only in newer Python AST schemas.
            # type_params rỗng chỉ xuất hiện trong schema AST của Python mới hơn.
            if field_name == "type_params" and not field_value:
                continue
            fields.append(
                f"{field_name}={_python_ast_fingerprint_dump(field_value)}"
            )
        return f"{type(value).__name__}({','.join(fields)})"
    if isinstance(value, (list, tuple)):
        return "[" + ",".join(_python_ast_fingerprint_dump(item) for item in value) + "]"
    return repr(value)

def _python_process_helper_fingerprint(node):
    canonical = _python_ast_fingerprint_dump(node)
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()

__all__ = ['_python_binding_root', '_python_ast_fingerprint_dump', '_python_process_helper_fingerprint']
