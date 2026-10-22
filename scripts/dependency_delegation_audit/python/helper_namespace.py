"""Python reviewed-namespace tracing helpers."""

from __future__ import annotations

import ast

from .helper_fingerprint import _python_binding_root

def _python_aliases_reviewed_namespace(node, alias_roots):
    if isinstance(node, ast.Name):
        return node.id in alias_roots
    if isinstance(node, ast.Attribute):
        if node.attr == "modules" and _python_binding_root(node) in alias_roots:
            return True
        return (
            node.attr in {
                "__getitem__", "copy", "get", "items", "pop", "setdefault",
                "values",
            }
            and _python_aliases_reviewed_namespace(node.value, alias_roots)
        )
    if isinstance(node, ast.Subscript):
        return _python_aliases_reviewed_namespace(node.value, alias_roots)
    if isinstance(
        node,
        (ast.Dict, ast.DictComp, ast.GeneratorExp, ast.List, ast.ListComp, ast.Set,
         ast.SetComp, ast.Tuple),
    ):
        # Follow module references through shallow copies and containers.
        # Theo dấu tham chiếu module qua bản sao nông và biểu thức chứa mapping.
        if isinstance(node, ast.Dict):
            values = node.values
        elif isinstance(
            node, (ast.DictComp, ast.GeneratorExp, ast.ListComp, ast.SetComp)
        ):
            values = (
                [node.key, node.value]
                if isinstance(node, ast.DictComp)
                else [node.elt]
            )
            for generator in node.generators:
                values.append(generator.iter)
                values.extend(generator.ifs)
        else:
            values = node.elts
        return any(
            value is not None
            and _python_aliases_reviewed_namespace(value, alias_roots)
            for value in values
        )
    if isinstance(node, ast.BinOp) and isinstance(node.op, ast.BitOr):
        return (
            _python_aliases_reviewed_namespace(node.left, alias_roots)
            or _python_aliases_reviewed_namespace(node.right, alias_roots)
        )
    if isinstance(node, ast.Call):
        if isinstance(node.func, ast.Attribute):
            if node.func.attr == "copy" and any(
                _python_aliases_reviewed_namespace(argument, alias_roots)
                for argument in node.args
            ):
                return True
            if (
                node.func.attr in {
                    "__getitem__", "copy", "get", "items", "pop",
                    "setdefault", "values",
                }
                and _python_aliases_reviewed_namespace(
                    node.func.value, alias_roots
                )
            ):
                return True
        if isinstance(node.func, ast.Name) and node.func.id in {
            "dict", "frozenset", "list", "set", "tuple",
        }:
            return any(
                _python_aliases_reviewed_namespace(argument, alias_roots)
                for argument in (*node.args, *(kw.value for kw in node.keywords))
            )
    return False

def _python_references_module_registry(node, sys_aliases, registry_aliases):
    """Find expressions derived from sys.modules aliases.
    (Tìm biểu thức xuất phát từ alias của sys.modules.)
    """
    if isinstance(node, ast.Name) and node.id in registry_aliases:
        return True
    if (
        isinstance(node, ast.Attribute)
        and node.attr == "modules"
        and _python_binding_root(node) in sys_aliases
    ):
        return True
    return any(
        _python_references_module_registry(child, sys_aliases, registry_aliases)
        for child in ast.iter_child_nodes(node)
    )

__all__ = ['_python_aliases_reviewed_namespace', '_python_references_module_registry']
