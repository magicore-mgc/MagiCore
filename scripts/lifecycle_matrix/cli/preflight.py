"""Argument, provenance, binary and taxonomy preflight."""

from __future__ import annotations

from .context import MatrixRunContext
from ..errors import _fail
from ..evidence.platform import record_platform_green, reset_platform_counter
from ..evidence.source import lifecycle_working_tree_clean
from ..paths.paths import REPOSITORY_ROOT
from ..process.process import run_text_capture
from ..validation.adapter import validate_adapter_consistency
from ..validation.dep_gate import validate_dep_gate_consistency
from ..validation.dependency_owners import validate_dependency_owners
from ..validation.framework import print_dependency_owner_summary
from ..validation.lane_owners import validate_lane_owners
from ..validation.native_pm import evidence_commit_error
import datetime
import os
import sys

def prepare_matrix_run() -> MatrixRunContext | int:
    """Apply the existing CLI early exits and return a verified run context."""
    # P0-3 platform-evidence CLI: --record-green <os> / --reset <os> write
    # the gitignored streak counter consumed by the matrix JSON
    # (platform_evidence) and the CI promotion job. Kept in THIS script so
    # the counter schema has exactly one home.
    # (CLI evidence platform P0-3: --record-green <os> / --reset <os> ghi
    # counter gitignored mà JSON matrix (platform_evidence) và job
    # promotion CI tiêu thụ. Đặt trong CHÍNH script này để schema counter
    # chỉ có một ngôi nhà duy nhất.)
    cli_args = sys.argv[1:]
    is_ci = os.environ.get("GITHUB_ACTIONS", "").lower() == "true"
    commit = run_text_capture(
        ["git", "rev-parse", "HEAD"],
        capture_output=True,
        text=True,
        cwd=REPOSITORY_ROOT,
    ).stdout.strip()
    source_tree_clean_before_run = lifecycle_working_tree_clean()
    if cli_args:
        if cli_args[0] in ("--record-green", "--reset") and len(cli_args) == 2:
            if cli_args[0] == "--record-green":
                provenance_error = evidence_commit_error(
                    commit,
                    os.environ.get("GITHUB_SHA"),
                    is_ci=is_ci,
                    recorded_sha=os.environ.get("MGC_PLATFORM_EVIDENCE_SHA"),
                    require_recorded=True,
                )
                if provenance_error:
                    _fail(provenance_error)
                return record_platform_green(cli_args[1])
            # Reset only removes a positive streak claim. Keep this recovery
            # path available even after a SHA mismatch so failed/misbound CI
            # cannot leave an old green streak looking consecutive.
            return reset_platform_counter(cli_args[1])
        _fail(
            f"unknown arguments: {' '.join(cli_args)} "
            "(expected --record-green <os> / --reset <os>)"
        )
    provenance_error = evidence_commit_error(
        commit,
        os.environ.get("GITHUB_SHA"),
        is_ci=is_ci,
    )
    if provenance_error:
        _fail(provenance_error)
    # Resolve to an ABSOLUTE path: steps run with cwd = sandbox, so a
    # relative binary path would resolve inside the sandbox and vanish.
    # Quy về đường dẫn TUYỆT ĐỐI: bước chạy với cwd = sandbox nên đường
    # dẫn tương đối sẽ trỏ vào sandbox và biến mất.
    mgc_bin = os.environ.get("MGC_LIFECYCLE_BIN", "./target/debug/mgc")
    # Validation dry-run mode (Gate 11-C): check lane OWNER metadata
    # WITHOUT running any lifecycle step — the CI gate that blocks a
    # lane whose taxonomy is missing or self-contradictory (install
    # owner native-engine but fetch owned by cargo, etc.).
    # (Chế độ dry-run validate (Gate 11-C): kiểm tra metadata OWNER của
    # lane KHÔNG chạy bước lifecycle nào — cổng CI chặn lane thiếu
    # taxonomy hoặc tự mâu thuẫn (install owner native-engine mà fetch
    # thuộc cargo, v.v.).)
    if os.environ.get("MGC_LIFECYCLE_VALIDATE_ONLY") == "1":
        # P0-5: BOTH gates run here — owner taxonomy AND adapter
        # consistency — fail-closed on the first violation.
        # (P0-5: CHẠY CẢ HAI cổng ở đây — taxonomy owner VÀ nhất quán
        # adapter — fail-closed ngay vi phạm đầu tiên.)
        owner_rc = validate_lane_owners()
        adapter_rc = validate_adapter_consistency()
        # P0-D: the dependency-owner gate joins the validate-only set;
        # the per-lane summary table (with verdict ceilings + JSON) only
        # prints when every gate is clean — a broken taxonomy never gets
        # a table that looks like a report.
        # (P0-D: cổng dependency-owner gia nhập bộ validate-only; bảng
        # summary theo lane (kèm trần verdict + JSON) chỉ in khi mọi cổng
        # sạch — taxonomy vỡ không bao giờ được có cái bảng trông như
        # báo cáo.)
        dep_rc = validate_dependency_owners()
        # T0.4: the binary↔matrix consistency gate joins the validate-only
        # set (mgc_bin resolved above from MGC_LIFECYCLE_BIN or default).
        # (T0.4: cổng nhất quán binary↔matrix gia nhập bộ validate-only.)
        dep_gate_rc = validate_dep_gate_consistency(mgc_bin)
        if owner_rc or adapter_rc or dep_rc or dep_gate_rc:
            return 1
        print_dependency_owner_summary()
        return 0
    mgc_bin = os.path.abspath(mgc_bin)
    if not os.path.isfile(mgc_bin) and os.name == "nt" and not mgc_bin.endswith(".exe"):
        # Windows binaries carry the .exe suffix — retry the suffixed path
        # before failing (the workflow passes the POSIX spelling).
        # (Binary Windows có hậu tố .exe — thử đường dẫn có hậu tố trước
        # khi fail.)
        candidate = mgc_bin + ".exe"
        if os.path.isfile(candidate):
            mgc_bin = candidate
    if not os.path.isfile(mgc_bin):
        _fail(f"mgc binary not found at {mgc_bin} — build it first")

    # Owner metadata gate (Gate 11-C, vòng-11 verdict): every lane MUST
    # declare per-operation owners BEFORE any collection runs — a lane
    # whose taxonomy is incomplete would emit a matrix that looks
    # machine-readable while laundering "install passed" into
    # "eccosystem managed" (the exact audit finding).
    # (Cổng metadata owner: mọi lane PHẢI khai báo owner theo từng
    # operation TRƯỚC khi collect — lane thiếu taxonomy sẽ sinh matrix
    # trông máy-đọc-được nhưng giặt "install passed" thành "ecosystem
    # được quản lý" (đúng finding của audit).)
    # Fail-closed (P0-5): a violated gate REFUSES collection — the old call
    # ignored the return code, letting mislabeled taxonomy reach the matrix
    # despite the comment above promising otherwise.
    # (Fail-closed (P0-5): cổng vi phạm TỪ CHỐI collect — code cũ bỏ qua
    # return code, cho taxonomy gắn nhãn sai chạm tới matrix dù comment
    # bên trên hứa điều ngược lại.)
    dep_gate_rc = validate_dep_gate_consistency(mgc_bin)
    if (
        validate_lane_owners() != 0
        or validate_adapter_consistency() != 0
        or validate_dependency_owners() != 0
        or dep_gate_rc != 0
    ):
        _fail(
            "lane owner/adapter-consistency/dependency-owner gates failed — "
            "refusing to collect a matrix from mislabeled taxonomy"
        )

    now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    return MatrixRunContext(
        mgc_bin=mgc_bin,
        commit=commit,
        source_tree_clean_before_run=source_tree_clean_before_run,
        generated_at=now,
    )

__all__ = ["prepare_matrix_run"]
