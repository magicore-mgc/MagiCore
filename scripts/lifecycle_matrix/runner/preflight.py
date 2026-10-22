"""Required external toolchain probes before lane execution."""

from __future__ import annotations

import shutil

from ..contract.dimensions import ALL_DIMENSIONS
from ..contract.status import STATUS_UNVERIFIED


def toolchain_preflight(ctx) -> bool:
    """Mark every dimension unverified when a declared toolchain is absent.

    Thiếu toolchain được khai báo thì ghi mọi dimension là unverified.
    """
    for probe_cmd in ctx.lane.get("toolchain_probes") or []:
        if shutil.which(probe_cmd) is None:
            ctx.dims = {dimension: STATUS_UNVERIFIED for dimension in ALL_DIMENSIONS}
            ctx.detail["toolchain_probe"] = (
                f"toolchain '{probe_cmd}' not found on PATH — "
                "all dimensions recorded as unverified (P0-6 evidence-only lane)"
            )
            return False
    return True


__all__ = ["toolchain_preflight"]
