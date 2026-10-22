"""Platform lifecycle evidence verdict checks."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..contract.dimensions import (
    ALL_DIMENSIONS,
    required_dimensions_for_lane,
)
from ..contract.ownership import (
    LIFECYCLE_OWNER_VALUES,
    lifecycle_owner_for,
    lifecycle_status_owner_matches,
)
from ..contract.status import (
    ALL_STATUSES,
    PASS_STATUSES,
    SCHEMA_VERSION,
)
from ..evidence.records import (
    PLATFORM_EVIDENCE_RELEASE_SCOPE,
)
from .framework import (
    framework_catalog_errors,
)
from .native_pm import (
    lifecycle_lane_owner_contract_errors,
    native_pm_claim_errors,
    native_pm_lane_errors,
    release_scope_out_reason,
)

def platform_evidence_errors(
    matrix, checkout_sha, *, expected_platform=None, current_tree_clean=None
):
    """Return failures that must block a platform-green streak increment."""
    errors = []
    if not isinstance(matrix, dict) or matrix.get("matrix_kind") != "lifecycle":
        return ["matrix is missing or is not lifecycle evidence"]
    if matrix.get("schema_version") != SCHEMA_VERSION:
        errors.append(
            f"matrix schema_version {matrix.get('schema_version')!r} "
            f"does not match required schema {SCHEMA_VERSION}"
        )
    if matrix.get("working_tree_clean") is not True:
        errors.append(
            "matrix was generated from a dirty or unknown working tree; "
            "platform promotion requires a clean source checkout"
        )
    if current_tree_clean is not True:
        errors.append(
            "current checkout is dirty or Git could not verify cleanliness; "
            "platform promotion requires a clean source checkout"
        )
    matrix_sha = matrix.get("commit")
    if not checkout_sha or matrix_sha != checkout_sha:
        errors.append(
            f"matrix commit {matrix_sha!r} does not match checkout {checkout_sha!r}"
        )
    if expected_platform and matrix.get("platform") != expected_platform:
        errors.append(
            f"matrix platform {matrix.get('platform')!r} does not match "
            f"runner platform {expected_platform!r}"
        )
    lanes = matrix.get("lanes")
    if not isinstance(lanes, list):
        return errors + ["matrix lanes are missing or malformed"]
    indexed = {}
    for lane in lanes:
        if not isinstance(lane, dict):
            errors.append("matrix contains a malformed lane record")
            continue
        core_language = (lane.get("core"), lane.get("language"))
        framework_id = lane.get("framework_id", "") or ""
        if (
            not all(isinstance(part, str) and part for part in core_language)
            or not isinstance(framework_id, str)
        ):
            errors.append("matrix contains a lane with invalid core/language identity")
            continue
        key = (*core_language, framework_id)
        if key in indexed:
            errors.append(f"matrix contains duplicate lane {key}")
        indexed[key] = lane
    for core, language, framework_id in sorted(
        set(indexed) - PLATFORM_EVIDENCE_RELEASE_SCOPE
    ):
        tag = f"{core}/{language}" + (f"#{framework_id}" if framework_id else "")
        errors.append(f"matrix contains lane outside global v1.2 scope {tag}")
    for core, language, framework_id in sorted(PLATFORM_EVIDENCE_RELEASE_SCOPE):
        tag = f"{core}/{language}" + (f"#{framework_id}" if framework_id else "")
        lane = indexed.get((core, language, framework_id))
        if lane is None:
            errors.append(f"platform release-scope lane {tag} is missing")
            continue
        expected_scope_out_reason = release_scope_out_reason(core, language, framework_id)
        actual_scope_out_reason = lane.get("release_scope_out_reason")
        if actual_scope_out_reason != expected_scope_out_reason:
            errors.append(
                f"platform lane {tag} release scope-out reason differs from source contract"
            )
        scope_out = expected_scope_out_reason is not None
        if not scope_out and lane.get("toolchain_available") is not True:
            errors.append(f"platform lane {tag} toolchain is unavailable or unverified")
        expected_gate_role = "scoped-out" if scope_out else "release-blocking"
        if lane.get("gate_role") != expected_gate_role:
            errors.append(
                f"platform lane {tag} gate_role is {lane.get('gate_role')!r}, "
                f"expected {expected_gate_role!r}"
            )
        required = lane.get("required_dimensions")
        dimensions = lane.get("dimensions")
        if (
            not isinstance(required, list)
            or not required
            or not all(isinstance(name, str) and name for name in required)
            or len(set(required)) != len(required)
        ):
            errors.append(f"platform release-scope lane {tag} has no required dimensions")
            continue
        source_required = required_dimensions_for_lane(core, language, framework_id)
        if source_required is None:
            errors.append(f"platform release-scope lane {tag} has no source applicability contract")
            continue
        source_lane = next(
            (
                candidate for candidate in LANES
                if candidate["core"] == core
                and candidate["language"] == language
                and (candidate.get("framework_id", "") or "") == framework_id
            ),
            None,
        )
        if source_lane is None:
            errors.append(f"platform release-scope lane {tag} has no source owner contract")
            continue
        errors.extend(lifecycle_lane_owner_contract_errors(lane))
        if lane.get("evidence_only") is not bool(source_lane.get("evidence_only")):
            errors.append(
                f"platform lane {tag} evidence_only differs from source contract"
            )
        if tuple(required) != source_required:
            errors.append(
                f"platform release-scope lane {tag} required dimensions differ from source contract: "
                f"matrix={required!r}, source={list(source_required)!r}"
            )
        if not isinstance(dimensions, dict):
            errors.append(f"platform release-scope lane {tag} has malformed dimensions")
            continue
        unknown_dimensions = sorted(set(dimensions) - set(ALL_DIMENSIONS))
        if unknown_dimensions:
            errors.append(
                f"platform release-scope lane {tag} has unknown dimensions: "
                + ", ".join(unknown_dimensions)
            )
        missing_dimensions = sorted(set(ALL_DIMENSIONS) - set(dimensions))
        if missing_dimensions:
            errors.append(
                f"platform release-scope lane {tag} is missing v1.2 dimensions: "
                + ", ".join(missing_dimensions)
            )
        invalid_dimensions = sorted(
            name for name, status in dimensions.items() if status not in ALL_STATUSES
        )
        if invalid_dimensions:
            errors.append(
                f"platform release-scope lane {tag} has invalid dimension statuses: "
                + ", ".join(
                    f"{name}={dimensions.get(name)!r}" for name in invalid_dimensions
                )
            )
        enforced_required = source_required
        failed = [name for name in enforced_required if dimensions.get(name) not in PASS_STATUSES]
        if failed and not scope_out:
            errors.append(
                f"platform release-scope lane {tag} has non-passing required dimensions: "
                + ", ".join(f"{name}={dimensions.get(name, 'missing')}" for name in failed)
            )
        if not scope_out and lane.get("verdict") != "orchestration-lifecycle-passed":
            errors.append(
                f"platform release-scope lane {tag} verdict is "
                f"{lane.get('verdict')!r}, not orchestration-lifecycle-passed"
            )
        lifecycle_owners = lane.get("lifecycle_owners")
        if not isinstance(lifecycle_owners, dict):
            errors.append(f"platform lane {tag} is missing lifecycle owner evidence")
        else:
            unknown_owner_dimensions = sorted(set(lifecycle_owners) - set(ALL_DIMENSIONS))
            if unknown_owner_dimensions:
                errors.append(
                    f"platform lane {tag} has unknown lifecycle owner dimensions: "
                    + ", ".join(unknown_owner_dimensions)
                )
            invalid_owners = {
                dimension: owner for dimension, owner in lifecycle_owners.items()
                if not isinstance(owner, str) or owner not in LIFECYCLE_OWNER_VALUES
            }
            if invalid_owners:
                errors.append(
                    f"platform lane {tag} has invalid lifecycle owners: {invalid_owners!r}"
                )
            for dimension in source_required:
                owner = lifecycle_owners.get(dimension)
                if not isinstance(owner, str) or owner not in LIFECYCLE_OWNER_VALUES:
                    errors.append(
                        f"platform lane {tag} has missing or invalid lifecycle owner "
                        f"for {dimension}: {owner!r}"
                    )
                else:
                    source_owner = lifecycle_owner_for(source_lane, dimension)
                    if owner != source_owner:
                        errors.append(
                            f"platform lane {tag} lifecycle owner differs from source "
                            f"contract for {dimension}: matrix={owner!r}, "
                            f"source={source_owner!r}"
                        )
                if (
                    (not scope_out or lane.get("toolchain_available") is True)
                    and isinstance(owner, str)
                    and owner in LIFECYCLE_OWNER_VALUES
                    and not lifecycle_status_owner_matches(
                        dimensions.get(dimension), owner
                    )
                ):
                    errors.append(
                        f"platform lane {tag} status/owner mismatch for {dimension}: "
                        f"status={dimensions.get(dimension)!r}, owner={owner!r}"
                    )
        if lane.get("dependency_owner") != "mgc-native":
            errors.extend(native_pm_lane_errors(lane))
    errors.extend(native_pm_claim_errors(lanes))
    errors.extend(framework_catalog_errors(matrix.get("framework_catalog"), lanes))
    return errors

__all__ = ['platform_evidence_errors']
