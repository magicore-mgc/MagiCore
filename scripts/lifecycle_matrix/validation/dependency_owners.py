"""Dependency owner vocabulary checks."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..contract.ownership import (
    DEPENDENCY_OWNER_VOCABULARY,
)
import sys

def validate_dependency_owners() -> int:
    """P0-D dependency-owner gate (2026-09-16): every lane declares
    `dependency_owner` from the closed vocabulary, and an `mgc-native`
    claim REQUIRES the `native-engine` install_owner label — the two
    taxonomies must agree or the verdict gate below would be laundrable.
    Fail-closed: missing/unknown/inconsistent value blocks with exit 1.
    (Cổng P0-D (2026-09-16): mọi lane khai `dependency_owner` thuộc bộ giá
    trị đóng, và claim `mgc-native` BẮT BUỘC nhãn install_owner
    `native-engine` — hai taxonomy phải khớp nếu không cổng verdict bên
    dưới sẽ bị giặt. Fail-closed: thiếu/lạ/mâu thuẫn → exit 1.)"""
    violations = []
    for lane in LANES:
        tag = f"{lane['core']}/{lane['language']}"
        dep = lane.get("dependency_owner")
        if not dep:
            violations.append(f"{tag}: missing dependency_owner (P0-D)")
            continue
        if dep not in DEPENDENCY_OWNER_VOCABULARY:
            violations.append(
                f"{tag}: unknown dependency_owner '{dep}' (vocabulary: "
                f"{', '.join(DEPENDENCY_OWNER_VOCABULARY)})"
            )
        if dep == "mgc-native" and lane.get("install_owner") != "native-engine":
            violations.append(
                f"{tag}: dependency_owner=mgc-native but install_owner="
                f"'{lane.get('install_owner')}' — contradiction (P0-D)"
            )
    if violations:
        for v in violations:
            print(f"DEPENDENCY-OWNER GATE VIOLATION: {v}", file=sys.stderr)
        return 1
    print(
        f"dependency-owner gate: {len(LANES)} lanes clean "
        f"(vocabulary: {', '.join(DEPENDENCY_OWNER_VOCABULARY)})"
    )
    return 0

__all__ = ['validate_dependency_owners']
