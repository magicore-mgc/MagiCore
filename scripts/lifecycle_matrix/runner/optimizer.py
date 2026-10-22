"""Runtime optimizer evidence."""

from __future__ import annotations
from ..contract.status import (
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_UNSUPPORTED,
)
from .context import (
    LaneRunContext,
)
import re

def optimizer_phase(ctx):
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

    # optimizer: run the REAL `mgc optimizer` in the sandbox. An exit-0
    # alone proves NOTHING (the command exits 0 with "Optimizer skipped"
    # when no runtime is detected) — the pass requires an actually-applied
    # configuration count > 0; no runtime / no optimizations → honestly
    # `unsupported` for this lane; command failure → `failed`.
    # (optimizer: chạy `mgc optimizer` THẬT trong sandbox. exit-0 trần
    # không chứng minh gì (lệnh exit-0 với "Optimizer skipped" khi không
    # thấy runtime) — pass đòi số cấu hình áp dụng THẬT > 0; không thấy
    # runtime / không có tối ưu → `unsupported` trung thực cho lane; lệnh
    # fail → `failed`.)
    if dims.get("create") == STATUS_NATIVE:
        rc, out = run_step(["optimizer"], subdir=project_dir)
        detail["optimizer_output"] = out[-2000:]
        applied = re.search(r"applied (\d+)/", out)
        if rc == 0 and applied and int(applied.group(1)) > 0:
            dims["optimizer"] = STATUS_NATIVE
        elif rc == 0:
            dims["optimizer"] = STATUS_UNSUPPORTED
        else:
            dims["optimizer"] = STATUS_FAILED

__all__ = ['optimizer_phase']
