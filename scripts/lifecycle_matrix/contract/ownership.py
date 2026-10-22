"""Dependency owners and lifecycle status projections."""

from __future__ import annotations

from .status import (
    STATUS_FAILED,
    STATUS_MANAGED,
    STATUS_NATIVE,
    STATUS_PLAIN,
    STATUS_UNSUPPORTED,
    STATUS_UNVERIFIED,
)

LIFECYCLE_OWNER_VALUES = frozenset(
    {"mgc-native", "managed-delegation", "plain-delegation", "unsupported", "unverified"}
)

LIFECYCLE_STEP_OWNER_DEFAULTS = {
    # The CLI routes these to external project test/build toolchains today.
    # Marking them plain-delegated prevents the wrapper command itself from
    # laundering successful subprocess execution into native evidence.
    "test": "plain-delegation",
    "build": "plain-delegation",
    # `mgc run` starts a project runtime; the runtime remains an external
    # execution dependency even though MGC owns the process boundary.
    "run": "plain-delegation",
}

# A source declaration cannot promote an arbitrary lifecycle subprocess to
# MGC-native. Each exception must be bound to an exact lane and operation
# with a dedicated runtime probe; today only the Web JavaScript dev server
# has that evidence (HTTP/HMR probe). Build/test/run remain delegated until
# an implementation and operation-specific evidence are added together.
# Không được nâng lifecycle subprocess tùy ý thành MGC-native bằng khai báo.
# Mỗi ngoại lệ phải gắn với lane + operation chính xác và probe runtime riêng;
# hiện chỉ Web JavaScript dev server có evidence HTTP/HMR.
NATIVE_LIFECYCLE_OWNER_EVIDENCE = frozenset({
    ("web", "javascript", "", "dev"),
})

def lifecycle_owner_for(lane, dimension):
    """Return the declared owner of a lifecycle dimension, fail-closed."""
    override = lane.get("lifecycle_owner_overrides", {}).get(dimension)
    if override:
        return override
    if dimension == "create":
        return "mgc-native"
    if dimension == "install":
        return {
            "native-engine": "mgc-native",
            "managed-delegation": "managed-delegation",
            "plain-delegation": "plain-delegation",
            "unsupported": "unsupported",
        }.get(lane.get("install_owner", ""), "unverified")
    return LIFECYCLE_STEP_OWNER_DEFAULTS.get(dimension, "unverified")

def lifecycle_pass_status(lane, dimension):
    """Map lifecycle owner to evidence status; unknown ownership never passes."""
    owner = lifecycle_owner_for(lane, dimension)
    return lifecycle_status_for_owner(owner)

def lifecycle_status_for_owner(owner):
    """Map an explicit lifecycle owner to its only valid passing status."""
    if owner == "mgc-native":
        return STATUS_NATIVE
    if owner == "managed-delegation":
        return STATUS_MANAGED
    if owner == "plain-delegation":
        return STATUS_PLAIN
    if owner == "unsupported":
        return STATUS_UNSUPPORTED
    return STATUS_UNVERIFIED

def lifecycle_status_owner_matches(status, owner):
    """Keep a failed probe visible without rewriting it as an owner mismatch.
    (Giữ trạng thái probe thất bại trung thực; không biến nó thành lỗi owner.)"""
    if owner not in LIFECYCLE_OWNER_VALUES:
        return False
    return status == STATUS_FAILED or status == lifecycle_status_for_owner(owner)

# P0-D (2026-09-16): per-lane DEPENDENCY OWNER — judged from the ADAPTER
# CODE, never from aspiration. One of exactly four values per lane:
#   mgc-native      mgc itself resolves+fetches+materializes (web engine)
#   delegated       an external toolchain owns the dependency lifecycle
#                   (cargo/uv/go/gradle/dotnet/flutter/pio/west/terraform)
#   scaffold-only   mgc generates the project once; no dependency lifecycle
#                   exists afterwards
#   unsupported     no dependency surface at all
# Verdict gate: `native-pm-supported` ONLY when dependency_owner is
# mgc-native; delegated lanes cap at the NEW `compatibility-passed`
# native-pm verdict; scaffold-only/unsupported read `unsupported`.
# (P0-D: CHỦ SỞ HỮU DEPENDENCY theo lane — phán theo CODE ADAPTER, không
# theo khát vọng. Đúng bốn giá trị mỗi lane: mgc-native / delegated /
# scaffold-only / unsupported. Cổng verdict: `native-pm-supported` CHỈ khi
# mgc-native; lane delegated trần ở verdict native-pm MỚI
# `compatibility-passed`; scaffold-only/unsupported đọc `unsupported`.)
DEPENDENCY_OWNER_VOCABULARY = ("mgc-native", "delegated", "scaffold-only", "unsupported")

def _owner_pass_status(lane: dict) -> str:
    """Map the lane's DECLARED install_owner to the pass status recorded on
    owner-covered dimensions (install + delegated steps + the resolve/lock/
    fetch/materialize evidence). An mgc-orchestrated success in a managed-
    delegation lane stays managed — a pass can never read as more native
    than the taxonomy declares.
    (Map install_owner ĐÃ KHAI của lane sang trạng thái pass cho các
    dimension thuộc quyền owner (install + bước delegated + bằng chứng
    resolve/lock/fetch/materialize). Thành công do mgc điều phối trong lane
    managed-delegation vẫn là managed — pass không bao giờ được đọc là
    native hơn taxonomy khai.)"""
    return {
        "native-engine": STATUS_NATIVE,
        "managed-delegation": STATUS_MANAGED,
        "plain-delegation": STATUS_PLAIN,
        "unsupported": STATUS_UNSUPPORTED,
    }.get(lane.get("install_owner", ""), STATUS_PLAIN)

__all__ = ['DEPENDENCY_OWNER_VOCABULARY', 'LIFECYCLE_OWNER_VALUES', 'LIFECYCLE_STEP_OWNER_DEFAULTS', 'NATIVE_LIFECYCLE_OWNER_EVIDENCE', '_owner_pass_status', 'lifecycle_owner_for', 'lifecycle_pass_status', 'lifecycle_status_for_owner', 'lifecycle_status_owner_matches']
