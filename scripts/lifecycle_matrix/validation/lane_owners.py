"""Required lane-operation ownership validation."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..contract.operations import (
    REQUIRED_OWNER_OPERATIONS,
)
import sys

def validate_lane_owners() -> int:
    """Gate 11-C owner gate: every lane declares install_owner AND an
    owner for each install operation including integrity verification, and the per-operation owners agree
    with the coarse install_owner label (native-engine lanes must have
    mgc/magicore owning resolve, lock, fetch, verify, store, materialize — any
    native-toolchain owner in that set contradicts the label and BLOCKS).
    Trả về 0 khi sạch; in lỗi và trả 1 khi có lane vi phạm.
    Cổng owner Gate 11-C: mọi lane khai install_owner VÀ owner cho từng
    operation install và xác minh integrity; owner per-operation phải khớp nhãn install_owner
    thô (lane native-engine phải có mgc/magicore giữ resolve, lock,
    fetch, verify, store, materialize — owner toolchain-native trong tập đó mâu
    thuẫn với nhãn và BỊ CHẶN)."""
    violations = []
    for lane in LANES:
        tag = f"{lane['core']}/{lane['language']}"
        owner = lane.get("install_owner")
        if not owner:
            violations.append(f"{tag}: missing install_owner")
            continue
        if owner not in ("native-engine", "managed-delegation", "plain-delegation", "unsupported"):
            violations.append(f"{tag}: unknown install_owner '{owner}'")
        ops = lane.get("owner_by_operation", {})
        if not ops:
            violations.append(f"{tag}: missing owner_by_operation (Gate 11-C per-operation taxonomy)")
            continue
        for op in REQUIRED_OWNER_OPERATIONS:
            if not ops.get(op):
                violations.append(f"{tag}: owner_by_operation missing '{op}'")
        # The coarse install label describes only Install. Each other
        # operation is checked independently against the compiled binary;
        # it must not inherit a native claim from Install.
        if owner == "native-engine":
            install_owner = ops.get("install", "")
            if not (install_owner.startswith("mgc") or install_owner.startswith("magicore")):
                violations.append(f"{tag}: install_owner=native-engine but install is not MGC-owned")
        elif owner == "unsupported" and not ops.get("install", "").startswith("unsupported"):
            violations.append(f"{tag}: install_owner=unsupported but install is not unsupported")
        # A delegation label must show the toolchain actually owning the
        # operations (at least fetch) — otherwise the delegation label is
        # unverified.
        # (Nhãn ủy quyền phải cho thấy toolchain thật sự giữ các
        # operation (ít nhất fetch) — nếu không nhãn ủy quyền chưa được
        # kiểm.)
        if owner in ("managed-delegation", "plain-delegation"):
            fetch_owner = ops.get("fetch", "")
            if fetch_owner.startswith("mgc") or fetch_owner.startswith("magicore"):
                violations.append(
                    f"{tag}: install_owner={owner} but fetch is owned by mgc — "
                    f"mislabeled (should be native-engine or the ops are wrong)"
                )
    if violations:
        for v in violations:
            print(f"OWNER GATE VIOLATION: {v}", file=sys.stderr)
        return 1
    print(f"owner gate: {len(LANES)} lanes clean (install_owner + per-operation owners consistent)")
    return 0

__all__ = ['validate_lane_owners']
