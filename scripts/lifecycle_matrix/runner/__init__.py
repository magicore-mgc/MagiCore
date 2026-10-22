"""Lazy public exports for runner owners."""

from __future__ import annotations

from importlib import import_module

_EXPORTS = {'STEP_TIMEOUT_DEFAULT_S': '.context', 'LaneRunContext': '.context', 'RECOVERY_PHASE': '.recovery', 'RECOVERY_READY_TIMEOUT_S': '.recovery', '_recovery_probe': '.recovery', 'run_lane': '.runner', 'toolchain_preflight': '.preflight'}
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
