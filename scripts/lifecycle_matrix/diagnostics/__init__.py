"""Lazy public exports for diagnostics owners."""

from __future__ import annotations

from importlib import import_module

_EXPORTS = {'COMMAND_OUTPUT_EXCERPT_LIMIT': '.diagnostics', 'github_actions_annotation_escape': '.diagnostics', 'github_actions_error_annotations': '.diagnostics', 'command_output_excerpt': '.diagnostics', 'lifecycle_failure_excerpt': '.diagnostics'}
__all__ = list(_EXPORTS)

def __getattr__(name: str):
    try:
        module_name = _EXPORTS[name]
    except KeyError as error:
        raise AttributeError(name) from error
    module = import_module(module_name, __name__)
    value = getattr(module, name)
    globals()[name] = value
    return value
