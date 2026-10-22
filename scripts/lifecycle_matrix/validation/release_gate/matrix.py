"""Run per-lane release evidence checks in source order."""

from __future__ import annotations

from .lane_claims import validate_lane_claims
from .lane_dimensions import validate_lane_dimensions
from .lane_scope import prepare_lane
from .models import GateState


def validate_lanes(state: GateState) -> None:
    for lane in state.data["lanes"]:
        context = prepare_lane(state, lane)
        if context is None:
            continue
        validate_lane_dimensions(state, context)
        validate_lane_claims(state, context)

