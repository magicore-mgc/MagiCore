"""Validate lifecycle dimension names, statuses, and declared owners."""

from __future__ import annotations

from ...contract.dimensions import ALL_DIMENSIONS
from ...contract.ownership import LIFECYCLE_OWNER_VALUES
from ...contract.ownership import lifecycle_status_owner_matches
from ...contract.status import ALL_STATUSES, PASS_STATUSES
from .models import GateState, LaneContext


def validate_lane_dimensions(state: GateState, context: LaneContext) -> None:
    lane, label = context.lane, context.label
    lifecycle_owners = lane.get("lifecycle_owners")
    dimensions = lane.get("dimensions", {})
    if not isinstance(dimensions, dict):
        state.failures.append(f"{label} dimensions must be an object")
        dimensions = {}

    if not isinstance(lifecycle_owners, dict):
        state.failures.append(f"{label} is missing lifecycle owner evidence")
    else:
        unknown_owners = sorted(set(lifecycle_owners) - set(ALL_DIMENSIONS))
        if unknown_owners:
            state.failures.append(
                f"{label} has unknown lifecycle owner dimensions: "
                + ", ".join(unknown_owners)
            )
        for dimension in context.source_required or ():
            owner = lifecycle_owners.get(dimension)
            if not isinstance(owner, str) or owner not in LIFECYCLE_OWNER_VALUES:
                state.failures.append(
                    f"{label} {dimension} has missing/invalid lifecycle owner: {owner!r}"
                )
            elif (
                (not context.scope_out or lane.get("toolchain_available") is True)
                and not lifecycle_status_owner_matches(dimensions.get(dimension), owner)
            ):
                state.failures.append(
                    f"{label} {dimension} status/owner mismatch: "
                    f"{dimensions.get(dimension)!r} vs {owner!r}"
                )

    unknown = sorted(set(dimensions) - set(ALL_DIMENSIONS))
    if unknown:
        state.failures.append(f"{label} has unknown dimensions: " + ", ".join(unknown))
    missing = sorted(set(ALL_DIMENSIONS) - set(dimensions))
    if missing:
        state.failures.append(
            f"{label} is missing v1.2 dimensions: " + ", ".join(missing)
        )
    invalid = sorted(name for name, status in dimensions.items() if status not in ALL_STATUSES)
    if invalid:
        state.failures.append(
            f"{label} has invalid dimension statuses: "
            + ", ".join(f"{name}={dimensions.get(name)!r}" for name in invalid)
        )
    required_failures = [
        name for name in (context.source_required or ())
        if dimensions.get(name) not in PASS_STATUSES
    ]
    if required_failures and not context.scope_out:
        state.failures.append(
            f"{label} has non-passing required dimensions: "
            + ", ".join(
                f"{name}={dimensions.get(name, 'missing')}"
                for name in required_failures
            )
        )

