"""Lazy public exports for validation owners."""

from __future__ import annotations

from importlib import import_module

_EXPORTS = {'LANE_RELEASE_SCOPE_OUT_REASONS': '.native_pm', 'native_pm_supported': '.native_pm', 'native_pm_unavailable_verdict': '.native_pm', 'native_pm_claim_scope': '.native_pm', 'native_pm_claim_errors': '.native_pm', 'native_pm_lane_errors': '.native_pm', 'lifecycle_lane_owner_contract_errors': '.native_pm', 'evidence_commit_error': '.native_pm', 'release_scope_out_reason': '.native_pm', 'SCAFFOLD_MARKERS': '.scaffold', 'scaffold_language_matches': '.scaffold', 'ADAPTER_REGISTRY_CAPABILITIES': '.adapter', 'CLI_NATIVE_DEPENDENCY_ROUTES': '.adapter', 'validate_adapter_consistency': '.adapter', 'validate_lane_registry': '.lane_registry', 'validate_lane_owners': '.lane_owners', 'validate_dependency_owners': '.dependency_owners', 'check_dep_gate_consistency': '.dep_gate', 'validate_dep_gate_consistency': '.dep_gate', 'framework_catalog_errors': '.framework', 'print_dependency_owner_summary': '.framework', 'platform_evidence_errors': '.platform'}
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
