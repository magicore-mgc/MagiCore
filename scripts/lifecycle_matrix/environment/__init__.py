"""Lazy public exports for environment owners."""

from __future__ import annotations

from importlib import import_module

_EXPORTS = {'PULUMI_LIFECYCLE_STACK': '.pulumi', 'pulumi_local_environment': '.pulumi', 'lifecycle_environment': '.shared', 'lifecycle_step_environment': '.shared', 'swift_toolchain_environment': '.swift', 'flutter_sdk_pub_cache_directory': '.flutter', 'flutter_sdk_home_directory': '.flutter', 'recovery_environment': '.recovery', 'mgc_add_python_test_build_tools': '.python'}
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
