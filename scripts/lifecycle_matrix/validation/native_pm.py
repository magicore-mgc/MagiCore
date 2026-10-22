"""Native package-manager claim and evidence validation."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..contract.operations import (
    ALL_DEPENDENCY_OPERATIONS,
    NATIVE_PM_REQUIRED_DIMENSIONS,
    NATIVE_PM_USER_OPERATIONS,
)
from ..contract.status import (
    LIFECYCLE_VERDICTS,
    STATUS_NATIVE,
)

def native_pm_supported(dependency_owner, install_owner, delegated, dimensions, operation_owners):
    """Require native evidence for each package-engine boundary.
    (Chỉ đạt khi từng ranh giới package-engine có evidence native.)"""
    expected_owners = {
        **{operation: "mgc" for operation in NATIVE_PM_USER_OPERATIONS},
        "resolve": "mgc",
        "lock": "mgc",
        "fetch": "mgc",
        "verify": "mgc",
        "store": "magicore-shared-cas",
        "materialize": "mgc",
    }
    return (
        dependency_owner == "mgc-native"
        and install_owner == "native-engine"
        and not delegated
        and all(dimensions.get(name) == STATUS_NATIVE for name in NATIVE_PM_REQUIRED_DIMENSIONS)
        and all(operation_owners.get(name) == owner for name, owner in expected_owners.items())
    )

def native_pm_unavailable_verdict(dependency_owner):
    """Preserve explicit unsupported ownership when runtime tools are absent.
    (Giữ verdict unsupported đã khai rõ khi thiếu toolchain runtime.)"""
    if dependency_owner in {"scaffold-only", "unsupported"}:
        return "unsupported"
    return "not-native-pm"

def native_pm_claim_scope(lanes):
    """Return every lane that declares MGC-native dependency ownership.
    (Trả mọi lane tự khai MGC sở hữu dependency native.)"""
    return {
        (lane["core"], lane["language"], lane.get("framework_id", ""))
        for lane in lanes
        if lane.get("dependency_owner") == "mgc-native"
    }

def native_pm_claim_errors(lanes):
    """Reject each native ownership claim without complete runtime evidence.
    (Từ chối claim native nếu thiếu bằng chứng runtime đầy đủ.)"""
    failures = []
    for lane in lanes:
        if lane.get("dependency_owner") != "mgc-native":
            continue
        failures.extend(native_pm_lane_errors(lane))
    return failures

def native_pm_lane_errors(lane):
    """Validate native-PM evidence only for native claims; keep other owners explicit.
    (Chỉ đòi native-PM evidence với claim native; owner khác phải khai rõ.)"""
    label = f"{lane.get('core', '?')}/{lane.get('language', '?')}"
    owner = lane.get("dependency_owner")
    install_owner = lane.get("install_owner")
    verdict = lane.get("native_pm_verdict")
    valid_install_owners = {
        "native-engine",
        "managed-delegation",
        "plain-delegation",
        "unsupported",
    }
    if owner not in {"mgc-native", "delegated", "scaffold-only", "unsupported"}:
        return [f"{label} has invalid dependency_owner {owner!r}"]
    if install_owner not in valid_install_owners:
        return [f"{label} has invalid install_owner {install_owner!r}"]
    failures = []
    lifecycle_verdict = lane.get("verdict")
    if (
        not isinstance(lifecycle_verdict, str)
        or lifecycle_verdict not in LIFECYCLE_VERDICTS
    ):
        failures.append(f"{label} has invalid lifecycle verdict {lifecycle_verdict!r}")
    if owner == "mgc-native":
        if install_owner != "native-engine":
            failures.append(
                f"{label} declares mgc-native with install_owner {install_owner!r}"
            )
        dimensions = lane.get("dimensions")
        if not isinstance(dimensions, dict):
            failures.append(f"{label} has malformed native-PM dimensions")
            dimensions = {}
        delegated = lane.get("native_pm_delegated")
        if not isinstance(delegated, list):
            failures.append(f"{label} has malformed native-PM delegation evidence")
            delegated = ["invalid"]
        operation_owners = lane.get("owner_by_operation")
        if not isinstance(operation_owners, dict):
            failures.append(f"{label} has malformed native-PM operation ownership")
            operation_owners = {}
        if verdict == "native-pm-supported" and not native_pm_supported(
            owner,
            install_owner,
            delegated,
            dimensions,
            operation_owners,
        ):
            failures.append(
                f"{label} declares native-pm-supported without complete package-manager evidence"
            )
        elif verdict not in {"native-pm-supported", "not-native-pm"}:
            failures.append(
                f"{label} mgc-native owner requires native_pm_verdict 'native-pm-supported' or 'not-native-pm', got {verdict!r}"
            )
        return failures
    if owner == "delegated":
        if install_owner not in {"managed-delegation", "plain-delegation"}:
            failures.append(
                f"{label} delegated owner has incompatible install_owner {install_owner!r}"
            )
        expected = (
            "compatibility-passed"
            if lifecycle_verdict == "orchestration-lifecycle-passed"
            else "not-native-pm"
        )
        if verdict != expected:
            failures.append(
                f"{label} delegated owner requires native_pm_verdict {expected!r}, got {verdict!r}"
            )
        return failures
    release_reason = lane.get("release_scope_out_reason")
    if not isinstance(release_reason, str) or not release_reason.strip():
        failures.append(
            f"{label} {owner} owner is missing an explicit release scope-out reason"
        )
    if install_owner != "unsupported" or verdict != "unsupported":
        failures.append(
            f"{label} {owner} owner requires unsupported install and native-PM verdicts"
        )
    return failures

def lifecycle_lane_owner_contract_errors(lane):
    """Require matrix package owners to match the source lane registry.
    (Bắt owner package trong matrix khớp registry lane trong source.)"""
    core = lane.get("core", "?")
    language = lane.get("language", "?")
    framework_id = lane.get("framework_id", "") or ""
    label = f"{core}/{language}" + (f"#{framework_id}" if framework_id else "")
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
        return [f"{label} has no source owner contract"]
    errors = []
    for field, description in (
        ("dependency_owner", "dependency owner"),
        ("install_owner", "install owner"),
    ):
        actual = lane.get(field)
        expected = source_lane.get(field)
        if actual != expected:
            errors.append(
                f"{label} {description} differs from source contract: "
                f"matrix={actual!r}, source={expected!r}"
            )
    matrix_operations = lane.get("owner_by_operation")
    source_operations = source_lane.get("owner_by_operation")
    if not isinstance(matrix_operations, dict):
        errors.append(f"{label} is missing per-operation ownership")
    elif not isinstance(source_operations, dict):
        errors.append(f"{label} source contract is missing per-operation ownership")
    else:
        for operation in ALL_DEPENDENCY_OPERATIONS:
            actual = matrix_operations.get(operation)
            expected = source_operations.get(operation)
            if actual != expected:
                errors.append(
                    f"{label} operation owner differs from source contract for {operation}: "
                    f"matrix={actual!r}, source={expected!r}"
                )
        extra_operations = sorted(set(matrix_operations) - set(ALL_DEPENDENCY_OPERATIONS))
        if extra_operations:
            errors.append(
                f"{label} has unknown package operation owners: "
                + ", ".join(extra_operations)
            )
    return errors

def evidence_commit_error(
    checkout_sha,
    expected_sha,
    *,
    is_ci=False,
    recorded_sha=None,
    require_recorded=False,
):
    """Reject CI evidence whose checked-out source differs from the run SHA."""
    if not checkout_sha:
        return "matrix checkout did not resolve a commit SHA"
    if is_ci and not expected_sha:
        return "GITHUB_SHA is required when generating lifecycle evidence in CI"
    if expected_sha and checkout_sha.lower() != expected_sha.lower():
        return f"matrix checkout SHA {checkout_sha} does not match workflow SHA {expected_sha}"
    if is_ci and require_recorded:
        if not recorded_sha:
            return "MGC_PLATFORM_EVIDENCE_SHA is required when recording CI green evidence"
        if recorded_sha.lower() != expected_sha.lower():
            return "platform evidence SHA does not match workflow GITHUB_SHA"
    return None

# Keep lifecycle evidence-only lanes in the inventory with an exact reason.
# A scope-out never removes the lane or its package claim; native claims still
# pass the separate completeness gate for every required dimension/operation.
# (Giữ lifecycle evidence-only lanes trong inventory cùng lý do chính xác.
# Scope-out không xóa lane hay claim package; claim native vẫn phải qua gate
# riêng kiểm đủ dimension và operation.)
LANE_RELEASE_SCOPE_OUT_REASONS = {
    ("lib", "java", "java"): (
        "Java dependency resolution exists, but native Java test/build execution is not implemented or release-qualified."
    ),
    ("lib", "dotnet", "dotnet"): (
        "Native NuGet install exists, but MGC does not produce the MSBuild restore state required by `dotnet test/build --no-restore`; .NET test/build is outside the approved 00-index §5.1 tool allowlist for this release."
    ),
    ("app", "swift", ""): (
        "Swift lifecycle evidence remains non-blocking until release E2E qualifies app lifecycle parity."
    ),
    ("app", "objc", ""): (
        "Objective-C dependency lifecycle is unsupported and not qualified for v1.2 release."
    ),
    ("app", "react-native", ""): (
        "React Native has scaffold evidence only; native dependency lifecycle is not qualified for v1.2."
    ),
    ("clo", "terraform", ""): (
        "Terraform dependency operations are unsupported; the declared cloud release lanes are CDK and Pulumi."
    ),
    ("game", "rust", ""): (
        "Game/Rust lifecycle evidence remains non-blocking until the game lane is qualified for release."
    ),
    ("hardware", "benchmark", ""): (
        "Hardware is generator and benchmark tooling only; it has no package lifecycle."
    ),
    ("iot", "rust", ""): (
        "IoT/Rust lifecycle evidence remains non-blocking until board toolchains and the lane are release-qualified."
    ),
    ("cicd", "github-actions", ""): (
        "GitHub Actions is a workflow scaffold; install, test, and build run on the external CI provider."
    ),
}

def release_scope_out_reason(core, language, framework_id=""):
    """Return the source-controlled reason a lane is visible but out of release scope.
    (Trả lý do từ source cho lane vẫn hiện nhưng nằm ngoài phạm vi release.)"""
    return LANE_RELEASE_SCOPE_OUT_REASONS.get((core, language, framework_id or ""))

__all__ = ['LANE_RELEASE_SCOPE_OUT_REASONS', 'evidence_commit_error', 'lifecycle_lane_owner_contract_errors', 'native_pm_claim_errors', 'native_pm_claim_scope', 'native_pm_lane_errors', 'native_pm_supported', 'native_pm_unavailable_verdict', 'release_scope_out_reason']
