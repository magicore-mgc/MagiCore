"""Lazy public exports for evidence owners."""

from __future__ import annotations

from importlib import import_module

_EXPORTS = {'FRAMEWORK_QUALIFICATION_EVIDENCE': '.records', 'PLATFORM_EVIDENCE_PATH': '.records', 'PLATFORM_EVIDENCE_PROMOTION_THRESHOLD': '.records', 'PLATFORM_EVIDENCE_OS_NAMES': '.records', 'PLATFORM_EVIDENCE_RELEASE_SCOPE': '.records', 'lifecycle_platform_name': '.source', 'lifecycle_working_tree_clean': '.source', 'lifecycle_source_matches': '.source', '_platform_evidence_load': '.platform', '_platform_evidence_write': '.platform', 'record_platform_green': '.platform', 'reset_platform_counter': '.platform'}
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
