"""Check lane identity, source scope, and ownership contract."""

from __future__ import annotations

from ...contract.dimensions import required_dimensions_for_lane
from ...validation.native_pm import (
    lifecycle_lane_owner_contract_errors,
    release_scope_out_reason,
)
from .models import GateState, LaneContext


def prepare_lane(state: GateState, lane: dict) -> LaneContext | None:
    key = (lane["core"], lane["language"], lane.get("framework_id", ""))
    label = f"{key[0]}/{key[1]}" + (f"#{key[2]}" if key[2] else "")
    if key not in state.release_scope:
        state.failures.append(f"unexpected lane outside global v1.2 scope: {key}")
        return None
    if key in state.seen:
        state.failures.append(f"duplicate lane in lifecycle matrix: {key}")
        return None
    state.seen.add(key)
    state.failures.extend(lifecycle_lane_owner_contract_errors(lane))

    expected_reason = release_scope_out_reason(*key)
    scope_out = expected_reason is not None
    if not scope_out:
        state.release_blocking_seen.add(key)
    if lane.get("release_scope_out_reason") != expected_reason:
        state.failures.append(
            f"{label} release scope-out reason differs from the source contract"
        )
    expected_role = "scoped-out" if scope_out else "release-blocking"
    if lane.get("gate_role") != expected_role:
        state.failures.append(
            f"{label} gate_role = {lane.get('gate_role')!r} "
            f"(expected {expected_role!r})"
        )
    if lane.get("evidence_only") is not scope_out:
        state.failures.append(
            f"{label} evidence_only differs from the source scope contract"
        )
    if not scope_out and lane.get("toolchain_available") is not True:
        state.failures.append(
            f"{label} required toolchain is unavailable or unverified"
        )

    source_required = required_dimensions_for_lane(*key)
    if source_required is None or lane.get("required_dimensions") != list(source_required):
        state.failures.append(
            f"{label} required_dimensions differ from the source contract"
        )
    return LaneContext(lane, key, label, scope_out, source_required)
