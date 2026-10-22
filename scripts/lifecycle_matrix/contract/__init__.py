"""Lazy public exports for contract owners."""

from __future__ import annotations

from importlib import import_module

_EXPORTS = {'ALL_DEPENDENCY_OPERATIONS': '.operations', 'NATIVE_INSTALL_PIPELINE_OPERATIONS': '.operations', 'NATIVE_OPERATIONS_EXCEPT_GC': '.operations', 'NATIVE_OPERATIONS_EXCEPT_GC_AND_OFFLINE': '.operations', 'NATIVE_USER_OPERATION_LANES': '.operations', 'NATIVE_PM_USER_OPERATIONS': '.operations', 'NATIVE_PM_REQUIRED_DIMENSIONS': '.operations', 'REQUIRED_OWNER_OPERATIONS': '.operations', 'LANE_REQUIRED_DIMENSIONS': '.dimensions', 'ALL_DIMENSIONS': '.dimensions', 'required_dimensions_for_lane': '.dimensions', 'LIFECYCLE_OWNER_VALUES': '.ownership', 'LIFECYCLE_STEP_OWNER_DEFAULTS': '.ownership', 'NATIVE_LIFECYCLE_OWNER_EVIDENCE': '.ownership', 'DEPENDENCY_OWNER_VOCABULARY': '.ownership', 'lifecycle_owner_for': '.ownership', 'lifecycle_pass_status': '.ownership', 'lifecycle_status_for_owner': '.ownership', 'lifecycle_status_owner_matches': '.ownership', '_owner_pass_status': '.ownership', 'STATUS_NATIVE': '.status', 'STATUS_MANAGED': '.status', 'STATUS_PLAIN': '.status', 'STATUS_UNVERIFIED': '.status', 'STATUS_UNSUPPORTED': '.status', 'STATUS_FAILED': '.status', 'PASS_STATUSES': '.status', 'ALL_STATUSES': '.status', 'LIFECYCLE_VERDICTS': '.status', 'SCHEMA_VERSION': '.status'}
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
