"""Status vocabulary and lifecycle verdict schema."""

from __future__ import annotations


# The SIX-VALUE status vocabulary (Gate 11-C). Only the three *-pass
# statuses satisfy a required dimension; `unverified` (skipped/env-missing)
# NEVER auto-promotes to a pass, `unsupported` means the dimension does not
# exist for the lane, `failed` means a crash/step failure.
# (Bộ 6 trạng thái (Gate 11-C). Chỉ ba trạng thái *-pass thỏa dimension
# bắt buộc; `unverified` (skip/thiếu env) KHÔNG BAO GIỜ tự động thành pass,
# `unsupported` = dimension không tồn tại cho lane, `failed` = crash/bước
# fail.)
STATUS_NATIVE = "native-pass"

STATUS_MANAGED = "managed-delegation-pass"

STATUS_PLAIN = "plain-delegation-pass"

STATUS_UNVERIFIED = "unverified"

STATUS_UNSUPPORTED = "unsupported"

STATUS_FAILED = "failed"

PASS_STATUSES = (STATUS_NATIVE, STATUS_MANAGED, STATUS_PLAIN)

ALL_STATUSES = frozenset(
    (*PASS_STATUSES, STATUS_UNVERIFIED, STATUS_UNSUPPORTED, STATUS_FAILED)
)

LIFECYCLE_VERDICTS = frozenset(
    (
        "orchestration-lifecycle-passed",
        "partial",
        "unsupported",
        "evidence-unverified",
    )
)

# JSON schema version v7: framework-qualified lanes, per-dimension lifecycle
# ownership, and the full compiled framework-qualification inventory.
# (Schema JSON v7: lane có framework identity, owner lifecycle từng chiều,
# và inventory qualification framework đầy đủ lấy từ binary đã biên dịch.)
SCHEMA_VERSION = 7

__all__ = ['ALL_STATUSES', 'LIFECYCLE_VERDICTS', 'PASS_STATUSES', 'SCHEMA_VERSION', 'STATUS_FAILED', 'STATUS_MANAGED', 'STATUS_NATIVE', 'STATUS_PLAIN', 'STATUS_UNSUPPORTED', 'STATUS_UNVERIFIED']
