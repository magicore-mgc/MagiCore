"""Lazy public exports for catalog owners."""

from __future__ import annotations

from importlib import import_module

_EXPORTS = {'LANES': '.catalog'}
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
