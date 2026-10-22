"""Python process-helper review and stale-approval checks."""

from __future__ import annotations

import ast

from ..rules.python import PYTHON_PROCESS_HELPER_DEPENDENCIES, PYTHON_PROCESS_HELPER_REVIEWS
from .helper_fingerprint import (
    _python_binding_root, _python_ast_fingerprint_dump,
    _python_process_helper_fingerprint,
)
from .helper_namespace import (
    _python_aliases_reviewed_namespace, _python_references_module_registry,
)

def review_python_process_helpers(sources: dict, expected_files=None) -> list:
    """Bind executable-selection helpers to exact reviewed function ASTs.
    (Ràng buộc hàm chọn executable với AST hàm đã được review chính xác.)
    """
    expected_paths = (
        set(expected_files)
        if expected_files is not None
        else set(PYTHON_PROCESS_HELPER_REVIEWS)
    )
    expected_paths.update(PYTHON_PROCESS_HELPER_REVIEWS)
    reviewed = []
    for rel_path in sorted(expected_paths & PYTHON_PROCESS_HELPER_REVIEWS.keys()):
        source = sources.get(rel_path)
        tree = None
        parse_error = None
        if source is not None:
            try:
                tree = ast.parse(source, filename=rel_path)
            except SyntaxError as error:
                parse_error = error.msg

        for function_name, (expected_fingerprint, reason) in (
            PYTHON_PROCESS_HELPER_REVIEWS[rel_path].items()
        ):
            dependencies = PYTHON_PROCESS_HELPER_DEPENDENCIES.get(
                rel_path, {}
            ).get(function_name, {})
            dependency_names = set(dependencies)
            dependency_alias_roots = set(dependency_names)
            rebindings = []
            dependency_mutations = []
            dependency_binding_errors = []
            namespace_mutations = []
            candidates = []
            if tree is not None:
                nodes = list(ast.walk(tree))
                namespace_mutators = {
                    "exec", "eval", "globals", "locals", "vars", "getattr",
                    "setattr", "delattr", "__import__", "import_module",
                    "resolve_name", "locate", "getmodule", "__getattribute__",
                    "attrgetter", "methodcaller", "getattr_static", "getmembers",
                    "getmembers_static", "get_objects", "get_referrers",
                    "get_referents", "getargvalues", "find_spec", "load_module",
                    "getclosurevars", "module_from_spec", "reload",
                }
                namespace_sensitive_imports = namespace_mutators | {"dict"}
                namespace_callable_alias_roots = set()
                candidates = [
                    node for node in nodes
                    if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
                    and node.name == function_name
                ]
                imports = []
                for node in nodes:
                    if isinstance(node, ast.Import):
                        imports.extend(
                            (
                                alias.asname or alias.name.split(".", 1)[0],
                                alias.name,
                                node.lineno,
                            )
                            for alias in node.names
                        )
                    elif isinstance(node, ast.ImportFrom) and node.module:
                        imports.extend(
                            (
                                alias.asname or alias.name,
                                f"{node.module}.{alias.name}",
                                node.lineno,
                            )
                            for alias in node.names
                            if alias.name != "*"
                        )
                sys_module_roots = {
                    bound_name
                    for bound_name, imported_name, _line in imports
                    if imported_name == "sys"
                }
                module_registry_alias_roots = {
                    bound_name
                    for bound_name, imported_name, _line in imports
                    if imported_name == "sys.modules"
                }
                for dependency, expected_import in dependencies.items():
                    matching_imports = [
                        item for item in imports if item[0] == dependency
                    ]
                    if (
                        len(matching_imports) != 1
                        or matching_imports[0][1] != expected_import
                    ):
                        dependency_binding_errors.append(
                            f"{dependency} must bind once to {expected_import}; "
                            f"found {matching_imports!r}"
                        )
                for bound_name, imported_name, _line in imports:
                    for expected_import in dependencies.values():
                        if (
                            imported_name == expected_import
                            or expected_import.startswith(imported_name + ".")
                            or imported_name.startswith(expected_import + ".")
                        ):
                            dependency_alias_roots.add(bound_name)
                    if imported_name in {"sys", "sys.modules"}:
                        # sys.modules can mutate dependency objects behind stable imports.
                        # sys.modules có thể sửa dependency dù binding import không đổi.
                        dependency_alias_roots.add(bound_name)
                # Propagate simple aliases so `other = shutil; other.which = ...`
                # cannot mutate a reviewed module through a new local name.
                # Truyền alias đơn giản để `other = shutil; other.which = ...`
                # không thể sửa module đã duyệt qua tên cục bộ mới.
                aliases_changed = True
                while aliases_changed:
                    aliases_changed = False
                    for node in nodes:
                        if isinstance(node, ast.Assign):
                            target_names = {
                                child.id for target in node.targets
                                for child in ast.walk(target)
                                if isinstance(child, ast.Name)
                            }
                            namespace_callable_alias = (
                                isinstance(node.value, ast.Attribute)
                                and node.value.attr in namespace_mutators
                            ) or (
                                isinstance(node.value, ast.Name)
                                and node.value.id in (
                                    namespace_mutators
                                    | namespace_callable_alias_roots
                                )
                            )
                            if (
                                namespace_callable_alias
                                and not target_names <= namespace_callable_alias_roots
                            ):
                                # Track resolver aliases through local assignments.
                                # Theo dõi alias resolver qua các phép gán cục bộ.
                                namespace_callable_alias_roots.update(target_names)
                                aliases_changed = True
                            if _python_aliases_reviewed_namespace(
                                node.value, dependency_alias_roots
                            ):
                                if not target_names <= dependency_alias_roots:
                                    dependency_alias_roots.update(target_names)
                                    aliases_changed = True
                            if _python_references_module_registry(
                                node.value,
                                sys_module_roots,
                                module_registry_alias_roots,
                            ):
                                if not target_names <= module_registry_alias_roots:
                                    module_registry_alias_roots.update(target_names)
                                    aliases_changed = True
                            if (
                                isinstance(node.value, ast.Name)
                                and node.value.id in sys_module_roots
                                and not target_names <= sys_module_roots
                            ):
                                sys_module_roots.update(target_names)
                                aliases_changed = True
                        elif isinstance(node, (ast.AnnAssign, ast.NamedExpr)):
                            value = node.value
                            target_names = {
                                child.id for child in ast.walk(node.target)
                                if isinstance(child, ast.Name)
                            }
                            namespace_callable_alias = (
                                isinstance(value, ast.Attribute)
                                and value.attr in namespace_mutators
                            ) or (
                                isinstance(value, ast.Name)
                                and value.id in (
                                    namespace_mutators
                                    | namespace_callable_alias_roots
                                )
                            )
                            if (
                                namespace_callable_alias
                                and not target_names <= namespace_callable_alias_roots
                            ):
                                namespace_callable_alias_roots.update(target_names)
                                aliases_changed = True
                            if _python_aliases_reviewed_namespace(
                                value, dependency_alias_roots
                            ):
                                if not target_names <= dependency_alias_roots:
                                    dependency_alias_roots.update(target_names)
                                    aliases_changed = True
                            if _python_references_module_registry(
                                value,
                                sys_module_roots,
                                module_registry_alias_roots,
                            ):
                                if not target_names <= module_registry_alias_roots:
                                    module_registry_alias_roots.update(target_names)
                                    aliases_changed = True
                            if (
                                isinstance(value, ast.Name)
                                and value.id in sys_module_roots
                                and not target_names <= sys_module_roots
                            ):
                                sys_module_roots.update(target_names)
                                aliases_changed = True
                for node in nodes:
                    if (
                        isinstance(node, ast.Name)
                        and node.id == function_name
                        and isinstance(node.ctx, (ast.Store, ast.Del))
                    ):
                        rebindings.append(node.lineno)
                    elif (
                        isinstance(node, ast.Attribute)
                        and node.attr == function_name
                        and isinstance(node.ctx, (ast.Store, ast.Del))
                    ):
                        rebindings.append(node.lineno)
                    if (
                        isinstance(node, (ast.Name, ast.Attribute, ast.Subscript))
                        and isinstance(node.ctx, (ast.Store, ast.Del))
                        and (
                            _python_binding_root(node) in dependency_alias_roots
                            or (
                                isinstance(node, (ast.Attribute, ast.Subscript))
                                and _python_aliases_reviewed_namespace(
                                    node.value, dependency_alias_roots
                                )
                            )
                        )
                    ):
                        dependency_mutations.append(node.lineno)
                    elif isinstance(node, ast.Import):
                        rebindings.extend(
                            alias.lineno if hasattr(alias, "lineno") else node.lineno
                            for alias in node.names
                            if (alias.asname or alias.name.split(".", 1)[0])
                            == function_name
                        )
                    elif isinstance(node, ast.ImportFrom):
                        if any(alias.name == "*" for alias in node.names) or (
                            node.module in {
                                "builtins", "importlib", "importlib.util",
                                "inspect", "pkgutil", "operator", "pydoc", "gc",
                            }
                            and any(
                                alias.name in namespace_sensitive_imports
                                for alias in node.names
                            )
                        ):
                            # Explicit builtin aliases can hide namespace reads and mutations.
                            # Alias builtin tường minh có thể che thao tác trên namespace.
                            namespace_mutations.append(node.lineno)
                        rebindings.extend(
                            alias.lineno if hasattr(alias, "lineno") else node.lineno
                            for alias in node.names
                            if (alias.asname or alias.name) == function_name
                        )
                    elif isinstance(node, ast.Call):
                        call_arguments = (
                            *node.args,
                            *(keyword.value for keyword in node.keywords),
                        )
                        if any(
                            _python_references_module_registry(
                                argument,
                                sys_module_roots,
                                module_registry_alias_roots,
                            )
                            for argument in call_arguments
                        ):
                            namespace_mutations.append(node.lineno)
                        if isinstance(node.func, ast.Name):
                            if (
                                node.func.id in namespace_mutators
                                or node.func.id in namespace_callable_alias_roots
                            ):
                                namespace_mutations.append(node.lineno)
                        elif isinstance(node.func, ast.Attribute):
                            if (
                                node.func.attr in namespace_mutators
                                or node.func.attr in {
                                    "__dict__", "__setattr__", "__delattr__",
                                }
                            ):
                                namespace_mutations.append(node.lineno)
                    elif (
                        isinstance(node, ast.Attribute)
                        and node.attr in {
                            "__dict__", "__globals__", "f_globals", "f_locals",
                            "f_builtins",
                        }
                    ):
                        # Helper namespace reflection can replace reviewed globals; fail closed.
                        # Phản chiếu globals của helper có thể thay nội dung đã duyệt; chặn mặc định.
                        namespace_mutations.append(node.lineno)
                    elif isinstance(node, ast.ExceptHandler):
                        if node.name == function_name:
                            rebindings.append(node.lineno)
                    elif isinstance(node, ast.arg):
                        if node.arg == function_name:
                            rebindings.append(node.lineno)
                    elif type(node).__name__ in {
                        "MatchAs", "MatchStar", "MatchMapping",
                    }:
                        if (
                            getattr(node, "name", None) == function_name
                            or getattr(node, "rest", None) == function_name
                        ):
                            rebindings.append(getattr(node, "lineno", 0))
                    elif isinstance(node, ast.ClassDef) and node.name == function_name:
                        rebindings.append(node.lineno)
            fingerprints = [
                _python_process_helper_fingerprint(node) for node in candidates
            ]
            exact_match = (
                len(candidates) == 1
                and fingerprints == [expected_fingerprint]
                and not rebindings
                and not dependency_mutations
                and not dependency_binding_errors
                and not namespace_mutations
            )
            if exact_match:
                status = "reviewed"
                error = None
            else:
                status = "unreviewed" if parse_error else "stale-review"
                if source is None:
                    error = "reviewed source file is missing or unreadable"
                elif parse_error:
                    error = f"source parse failed: {parse_error}"
                elif rebindings:
                    error = (
                        "reviewed helper name has additional binding(s) at line(s) "
                        f"{sorted(set(rebindings))}"
                    )
                elif dependency_mutations:
                    error = (
                        "reviewed helper dependency mutation at line(s) "
                        f"{sorted(set(dependency_mutations))}"
                    )
                elif dependency_binding_errors:
                    error = "; ".join(dependency_binding_errors)
                elif namespace_mutations:
                    error = (
                        "dynamic namespace mutation or wildcard import at line(s) "
                        f"{sorted(set(namespace_mutations))}"
                    )
                elif not candidates:
                    error = f"expected function {function_name!r} was not found"
                else:
                    error = (
                        f"expected one exact function fingerprint; found "
                        f"{len(candidates)} candidate(s) with {fingerprints!r}"
                    )
            reviewed.append({
                "file": rel_path,
                "line": candidates[0].lineno if len(candidates) == 1 else 0,
                "function": function_name,
                "function_fingerprint": (
                    fingerprints[0] if len(fingerprints) == 1 else None
                ),
                "binding_locations": sorted(set(rebindings)),
                "dependency_alias_roots": sorted(dependency_alias_roots),
                "dependency_mutation_locations": sorted(set(dependency_mutations)),
                "dependency_binding_errors": dependency_binding_errors,
                "namespace_mutation_locations": sorted(set(namespace_mutations)),
                "review_status": status,
                "review_reason": reason,
                "parse_error": error,
            })
    return reviewed

__all__ = ['_python_binding_root', '_python_ast_fingerprint_dump', '_python_process_helper_fingerprint', '_python_aliases_reviewed_namespace', '_python_references_module_registry', 'review_python_process_helpers']
