"""Initialize source-controlled release-gate scope and schema checks."""

from __future__ import annotations

from ...contract.status import SCHEMA_VERSION
from ...evidence.records import PLATFORM_EVIDENCE_RELEASE_SCOPE
from ...validation.framework import framework_catalog_errors
from ...validation.native_pm import native_pm_claim_scope
from .models import GateState


def prepare_gate(data: dict) -> GateState:
    state = GateState(data=data)
    schema = data.get("schema_version")
    if schema != SCHEMA_VERSION:
        state.failures.append(
            f"matrix schema {schema!r} does not match {SCHEMA_VERSION}"
        )
    state.release_scope = set(PLATFORM_EVIDENCE_RELEASE_SCOPE)
    state.native_pm_scope = native_pm_claim_scope(data["lanes"])
    state.failures.extend(
        framework_catalog_errors(data.get("framework_catalog"), data["lanes"])
    )
    return state

