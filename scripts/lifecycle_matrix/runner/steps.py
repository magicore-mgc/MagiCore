"""Ordered lifecycle step execution and status assignment."""

from __future__ import annotations
from ..contract.ownership import (
    lifecycle_owner_for,
    lifecycle_pass_status,
)
from ..contract.status import (
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_UNSUPPORTED,
    STATUS_UNVERIFIED,
)
from ..diagnostics.diagnostics import (
    lifecycle_failure_excerpt,
)
from .context import (
    LaneRunContext,
)

def steps_phase(ctx):
    lane = ctx.lane
    mgc_bin = ctx.mgc_bin
    timeout_s = ctx.timeout_s
    sandbox = ctx.sandbox
    dims = ctx.dims
    detail = ctx.detail
    project_dir = ctx.project_dir
    project_path = ctx.project_path
    lane_env = ctx.lane_env
    flutter_sdk_pub_cache = ctx.flutter_sdk_pub_cache
    flutter_sdk_home = ctx.flutter_sdk_home
    run_step = ctx.run_step
    blocked_lifecycle_steps = ctx.blocked_lifecycle_steps

    for step, argv in lane["steps"]:
        if step in blocked_lifecycle_steps:
            dims[step] = STATUS_UNVERIFIED
            detail[f"{step}_output"] = blocked_lifecycle_steps[step]
            continue
        if dims.get("create") != STATUS_NATIVE:
            # Preserve explicit unsupported ownership; other skipped steps stay unverified.
            # Bước đã khai unsupported giữ đúng trạng thái; skip khác không được coi là pass.
            dims[step] = (
                STATUS_UNSUPPORTED
                if lifecycle_owner_for(lane, step) == "unsupported"
                else STATUS_UNVERIFIED
            )
            continue
        rc, out = run_step(argv, subdir=project_dir, lifecycle_step=step)
        if rc != 0:
            dims[step] = STATUS_FAILED
            detail[f"{step}_output"] = lifecycle_failure_excerpt(lane, out)
        else:
            # A successful wrapper command is not evidence of native
            # implementation. Record the declared owner of this operation;
            # external test/build/run toolchains remain delegated even when
            # MGC successfully orchestrates them.
            dims[step] = lifecycle_pass_status(lane, step)

__all__ = ['steps_phase']
