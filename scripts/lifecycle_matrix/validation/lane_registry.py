"""Lane registry validation."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..contract.dimensions import (
    ALL_DIMENSIONS,
)
from ..contract.ownership import (
    LIFECYCLE_OWNER_VALUES,
    NATIVE_LIFECYCLE_OWNER_EVIDENCE,
    lifecycle_owner_for,
)

def validate_lane_registry(lanes=LANES):
    """Reject ambiguous or internally inconsistent lifecycle lane declarations."""
    lane_keys = set()
    framework_keys = set()
    for lane in lanes:
        core_language = (lane.get("core"), lane.get("language"))
        if not all(core_language):
            raise ValueError(f"lane is missing core/language identity: {lane!r}")
        framework_id = lane.get("framework_id") or ""
        key = (*core_language, framework_id)
        if key in lane_keys:
            raise ValueError(f"duplicate core/language/framework lane: {key}")
        lane_keys.add(key)

        required = lane.get("required_dims")
        if not required or len(required) != len(set(required)):
            raise ValueError(f"lane has empty or duplicate required dimensions: {key}")
        unknown_required = sorted(set(required) - set(ALL_DIMENSIONS))
        if unknown_required:
            raise ValueError(
                f"lane has unknown required dimension(s) {unknown_required}: {key}"
            )
        overrides = lane.get("lifecycle_owner_overrides", {})
        if not isinstance(overrides, dict):
            raise ValueError(f"lane lifecycle_owner_overrides must be an object: {key}")
        invalid_override_dims = sorted(set(overrides) - set(ALL_DIMENSIONS))
        if invalid_override_dims:
            raise ValueError(
                f"lane has unknown lifecycle owner override dimensions "
                f"{invalid_override_dims}: {key}"
            )
        invalid_override_owners = {
            dim: owner for dim, owner in overrides.items()
            if not isinstance(owner, str) or owner not in LIFECYCLE_OWNER_VALUES
        }
        if invalid_override_owners:
            raise ValueError(
                f"lane has invalid lifecycle owner override(s) "
                f"{invalid_override_owners}: {key}"
            )
        unproven_native_overrides = {
            dim: owner
            for dim, owner in overrides.items()
            if owner == "mgc-native"
            and (*key, dim) not in NATIVE_LIFECYCLE_OWNER_EVIDENCE
        }
        if unproven_native_overrides:
            raise ValueError(
                "native lifecycle owner override lacks evidence for exact "
                f"lane/operation {unproven_native_overrides}: {key}"
            )
        invalid_owners = {
            dim: lifecycle_owner_for(lane, dim)
            for dim in required
                if lifecycle_owner_for(lane, dim) not in LIFECYCLE_OWNER_VALUES
        }
        if invalid_owners:
            raise ValueError(f"lane has invalid lifecycle owner(s) {invalid_owners}: {key}")

        step_names = [step[0] for step in lane.get("steps", ())]
        unknown_steps = sorted(set(step_names) - set(ALL_DIMENSIONS))
        if unknown_steps:
            raise ValueError(f"lane has unknown lifecycle step(s) {unknown_steps}: {key}")
        duplicate_steps = sorted(
            name for name in set(step_names) if step_names.count(name) > 1
        )
        if duplicate_steps:
            raise ValueError(f"lane has duplicate lifecycle step(s) {duplicate_steps}: {key}")

        framework_ids = lane.get("framework_ids", ())
        if framework_ids and not framework_id:
            raise ValueError(
                f"lane with framework_ids requires an exact framework_id: {core_language}"
            )
        if framework_id and list(framework_ids) != [framework_id]:
            raise ValueError(
                "framework_id must match the lane's single framework_ids entry: "
                f"{core_language}"
            )
        for framework in framework_ids:
            framework_key = (core_language[0], framework)
            if framework_key in framework_keys:
                raise ValueError(
                    f"duplicate core/framework lifecycle lane: "
                    f"{framework_key[0]}/{framework_key[1]}"
                )
            framework_keys.add(framework_key)
    return True

__all__ = ['validate_lane_registry']
