"""Validate lane verdicts and native package-manager ownership claims."""

from __future__ import annotations

from ...validation.native_pm import native_pm_lane_errors
from .models import GateState, LaneContext


def validate_lane_claims(state: GateState, context: LaneContext) -> None:
    lane, key, label = context.lane, context.key, context.label
    if (
        not context.scope_out
        and lane["verdict"] != "orchestration-lifecycle-passed"
    ):
        state.failures.append(
            f"{label} lifecycle verdict = {lane['verdict']} "
            f"(expected orchestration-lifecycle-passed) — dimensions: {lane['dimensions']}"
        )
    if lane.get("dependency_owner") != "mgc-native":
        state.failures.extend(native_pm_lane_errors(lane))

    owner = lane.get("install_owner")
    valid_owners = {
        "native-engine", "managed-delegation", "plain-delegation", "unsupported"
    }
    if owner not in valid_owners:
        state.failures.append(
            f"{label} missing install_owner — the matrix must name who owns "
            "the install (taxonomy P0-D)"
        )
    elif key in state.native_pm_scope and owner != "native-engine":
        state.failures.append(
            f"{label} install_owner = {owner} — a native-PM claim lane must be native-engine"
        )

