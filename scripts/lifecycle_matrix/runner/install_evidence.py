"""Install outcome, lockfile, fetch and materialization evidence."""

from __future__ import annotations
from ..contract.ownership import (
    _owner_pass_status,
)
from ..contract.status import (
    PASS_STATUSES,
    STATUS_FAILED,
    STATUS_UNVERIFIED,
)
from ..filesystem.filesystem import (
    _dir_nonempty,
)
from ..filesystem.lockfiles import (
    _lockfile_candidates,
    _lockfile_entry_count,
)
from ..filesystem.markers import (
    _materialize_marker,
)
from .context import (
    LaneRunContext,
)
import os

def install_evidence_phase(ctx):
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

    install_status = dims.get("install")
    ctx.install_status = install_status
    # ------------------------------------------------------------------
    # Gate 11-C fine-grained install evidence: resolve / lock / fetch /
    # materialize — derived from the install OUTCOME plus observable
    # filesystem artifacts, never from intent. No artifact observed →
    # `unverified` (a zero-dep template is an honesty boundary, not a
    # pass); install failure → `failed` for the whole chain.
    # (Bằng chứng install tinh Gate 11-C: resolve/lock/fetch/materialize —
    # suy từ KẾT QUẢ install cộng artifact filesystem quan sát được,
    # không bao giờ suy từ ý định. Không quan sát thấy artifact →
    # `unverified` (template zero-dep là một biên trung thực, không phải
    # pass); install fail → `failed` cho cả chuỗi.)
    install_status = dims.get("install")
    project_path = os.path.join(sandbox, project_dir)
    if install_status == STATUS_FAILED:
        dims["resolve"] = STATUS_FAILED
        dims["lock"] = STATUS_FAILED
        dims["fetch"] = STATUS_FAILED
        dims["materialize"] = STATUS_FAILED
    elif install_status in PASS_STATUSES:
        owner = _owner_pass_status(lane)
        # lock: a known lockfile artifact EXISTS after install.
        # (lock: một artifact lockfile đã biết TỒN TẠI sau install.)
        probes = _lockfile_candidates(project_path, lane["language"])
        found, with_entries = [], []
        for name, kind in probes:
            probe_path = os.path.join(project_path, name)
            if not os.path.isfile(probe_path):
                continue
            found.append(name)
            count = _lockfile_entry_count(probe_path, kind)
            if count:
                with_entries.append(f"{name}({count})")
        if found:
            dims["lock"] = owner
            detail["lock_output"] = f"lockfile artifacts present: {found}"
        else:
            dims["lock"] = STATUS_UNVERIFIED
            detail["lock_output"] = (
                f"no lockfile artifact observed for language "
                f"'{lane['language']}' (zero-dep template or lockless "
                f"ecosystem) — claim unverified"
            )
        # resolve: the lockfile carries ≥1 resolved entry.
        # (resolve: lockfile mang ít nhất 1 entry đã resolve.)
        if with_entries:
            dims["resolve"] = owner
            detail["resolve_output"] = f"resolved entries observed: {with_entries}"
        else:
            dims["resolve"] = STATUS_UNVERIFIED
            detail["resolve_output"] = (
                "no resolved package entries observed — resolve claim unverified"
            )
        # materialize: the dependency tree landed where the ecosystem
        # actually puts it.
        # (materialize: cây dependency đã về đúng nơi ecosystem đặt nó.)
        label, marker = _materialize_marker(
            project_path, lane["language"], environment=lane_env
        )
        if label is None:
            dims["materialize"] = STATUS_UNVERIFIED
            detail["materialize_output"] = (
                "no observable materialization marker for this ecosystem"
            )
        elif _dir_nonempty(marker):
            dims["materialize"] = owner
            detail["materialize_output"] = f"materialized: {label} non-empty"
        else:
            dims["materialize"] = STATUS_UNVERIFIED
            detail["materialize_output"] = f"marker '{label}' absent or empty"
        # fetch: proven ONLY when bytes demonstrably landed — resolved
        # entries AND materialization together; one without the other is
        # unverified (never auto-passed).
        # (fetch: chỉ chứng minh khi byte thật sự về — entry đã resolve
        # VÀ materialize CÙNG lúc; thiếu một là unverified (không bao giờ
        # tự động pass).)
        if dims["resolve"] == owner and dims["materialize"] == owner:
            dims["fetch"] = owner
        else:
            dims["fetch"] = STATUS_UNVERIFIED
            detail["fetch_output"] = (
                f"fetch needs resolve+materialize evidence together "
                f"(resolve={dims['resolve']}, materialize={dims['materialize']})"
            )
    else:
        # install never ran (lane skipped) — the chain stays unverified.
        # (install chưa chạy (lane skip) — chuỗi giữ unverified.)
        for dim in ("resolve", "lock", "fetch", "materialize"):
            dims.setdefault(dim, STATUS_UNVERIFIED)

__all__ = ['install_evidence_phase']
