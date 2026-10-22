"""Lifecycle dimensions and per-lane required dimensions."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)

# Locked v1.2 applicability contract. The generated matrix may report every
# known dimension, but only dimensions declared here are release-required for
# that core/language lane. Keeping this registry in source prevents a JSON
# producer from shrinking `required_dimensions`; it also avoids treating an
# intentionally non-applicable operation (for example `dev` for a library)
# as a failed implementation. Every declared lane must appear exactly once.
LANE_REQUIRED_DIMENSIONS = {
    (lane["core"], lane["language"], lane.get("framework_id", "")): tuple(
        lane["required_dims"]
    )
    for lane in LANES
}

def required_dimensions_for_lane(core, language, framework_id=""):
    """Return the source-controlled release dimensions for one lane."""
    return LANE_REQUIRED_DIMENSIONS.get((core, language, framework_id or ""))

# Schema v4: 23 lifecycle dimensions. Fetching bytes is not proof that MGC
# verified artifact integrity before storing or materializing.
# (Schema v4: tải bytes không chứng minh MGC đã xác minh integrity trước
# khi lưu hoặc materialize.)
# `cache-reuse`/`offline` legacy names are replaced by explicit operations
# `store` and `offline-reinstall` so the
# `.dimensions` list is the canonical 23-name contract — a consumer of the
# JSON can rely on the order and the names below.
# (Schema v4: 23 dimension lifecycle. Tên cũ `cache-reuse`/`offline`
# đổi thành operation rõ `store`/`offline-reinstall` để `.dimensions` là hợp
# đồng 23 tên chuẩn — consumer của JSON dựa được vào tên và thứ tự dưới.)
ALL_DIMENSIONS = ["create", "install", "add", "remove", "update", "list",
                  "test", "build", "run", "dev", "audit", "store",
                  "offline-reinstall", "detect", "resolve", "lock", "fetch",
                  "verify", "materialize", "frozen-install", "gc",
                  "optimizer", "recovery"]

__all__ = ['ALL_DIMENSIONS', 'LANE_REQUIRED_DIMENSIONS', 'required_dimensions_for_lane']
