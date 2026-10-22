"""Recovery, audit, verification and unimplemented operation statuses."""

from __future__ import annotations
from ..contract.dimensions import (
    ALL_DIMENSIONS,
)
from ..contract.operations import (
    NATIVE_PM_USER_OPERATIONS,
)
from ..contract.status import (
    PASS_STATUSES,
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_UNSUPPORTED,
    STATUS_UNVERIFIED,
)
from .context import (
    LaneRunContext,
)
from .recovery import (
    _recovery_probe,
)
import os

def outcomes_phase(ctx):
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

    install_status = ctx.install_status
    # recovery — THE flagship dimension (Gate 11-C): real crash + real
    # repair through the built-in failpoint handshake. `mgc install` parks
    # at after-generation-begin (READY marker fsync'd by the binary), the
    # harness SIGKILLs it, then `mgc store doctor --repair` must leave the
    # store HEALTHY with 0 stale staging (Rust contract:
    # cli/tests/kill_injection_matrix.rs). Only lanes explicitly marked
    # `recovery_probe: true` (web/js and lib/ts using web.install) HAVE this
    # crash surface; native ownership alone is not recovery evidence.
    # Other lanes are honestly unsupported. POSIX only: no signal support
    # → unverified, never faked.
    # (recovery — dimension QUAN TRỌNG NHẤT (Gate 11-C): crash thật + sửa
    # chữa thật qua handshake failpoint có sẵn. `mgc install` đỗ tại
    # after-generation-begin (marker READY do binary fsync), harness
    # SIGKILL, rồi `mgc store doctor --repair` phải để store HEALTHY với
    # 0 stale staging (hợp đồng Rust: cli/tests/kill_injection_matrix.rs).
    # Chỉ lane khai `recovery_probe: true` (web/js và lib/ts dùng đúng
    # web.install) CÓ bề mặt crash này; native ownership không tự chứng
    # minh recovery. Lane khác unsupported trung thực. Chỉ POSIX: không
    # hỗ trợ signal → unverified, không bao giờ bịa.)
    if not lane.get("recovery_probe", False):
        dims["recovery"] = STATUS_UNSUPPORTED
        detail["recovery_output"] = (
            "no lane-specific crash-recovery probe is implemented; "
            "native dependency ownership does not imply recovery evidence"
        )
    elif os.name == "nt":
        dims["recovery"] = STATUS_UNVERIFIED
        detail["recovery_output"] = (
            "no POSIX signal support on this platform — probe not run, not faked"
        )
    elif dims.get("create") != STATUS_NATIVE or install_status not in PASS_STATUSES:
        dims["recovery"] = STATUS_UNVERIFIED
        detail["recovery_output"] = (
            "lane prerequisites (create+install pass) not met — probe not run"
        )
    elif _recovery_probe(mgc_bin, project_path, sandbox, timeout_s, detail):
        dims["recovery"] = STATUS_NATIVE
    else:
        dims["recovery"] = STATUS_FAILED

    # audit rides the audit matrix — reference only, never inferred.
    # audit thuộc matrix audit riêng — tham chiếu, không suy ra.
    dims["audit"] = STATUS_UNVERIFIED
    detail["audit_output"] = (
        "audit capability is proven by the separate audit capability "
        "matrix; this lifecycle matrix does not verify it"
    )

    # Dimensions with no step/probe for this lane: `unsupported` (honest —
    # `run`/`dev` on non-web lanes; the `store`/`offline` probes are not
    # part of any lane's step sequence today).
    # (Dimension không có bước/probe cho lane này: `unsupported` (trung
    # thực — `run`/`dev` trên lane non-web; probe `store`/`offline` chưa
    # thuộc step sequence của lane nào hiện nay).)
    for dim in ALL_DIMENSIONS:
        dims.setdefault(dim, STATUS_UNSUPPORTED)

    # Integrity verification is an explicit evidence dimension, not inferred
    # from fetching or a source-level implementation. No lane-specific
    # tampered-artifact probe exists in this matrix yet, so record unverified.
    # (Xác minh integrity là dimension evidence riêng, không suy từ fetch hay
    # code tồn tại. Matrix chưa có probe artifact bị sửa theo từng lane nên ghi
    # unverified.)
    dims["verify"] = STATUS_UNVERIFIED
    detail["verify_output"] = (
        "no lane-specific tampered-artifact rejection probe is implemented; "
        "source-level verifier presence is not runtime evidence"
    )

    # CRUD/list/frozen/offline/GC have no lane-specific execution probes yet.
    # Keep each operation explicit and unverified; install evidence cannot
    # stand in for the rest of the package-manager contract.
    # (Chưa có probe riêng CRUD/list/frozen/offline/GC theo lane. Ghi từng
    # operation là unverified; evidence install không thay thế phần còn lại.)
    for operation in NATIVE_PM_USER_OPERATIONS:
        if operation == "install":
            continue
        if operation in dims and dims[operation] != STATUS_UNSUPPORTED:
            continue
        dims[operation] = STATUS_UNVERIFIED
        detail[f"{operation.replace('-', '_')}_output"] = (
            "no lane-specific execution probe exists for this package operation"
        )

__all__ = ['outcomes_phase']
