"""Scaffold creation, language detection and early toolchain probe."""

from __future__ import annotations
from ..contract.status import (
    STATUS_FAILED,
    STATUS_NATIVE,
    STATUS_UNVERIFIED,
)
from ..validation.scaffold import (
    scaffold_language_matches,
)
import os

def scaffold_phase(ctx):
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

    # create: the scaffold step proves the create dimension — BUT only
    # when the produced project matches the requested language. A
    # fallback template of another language (e.g. a Rust Cargo.toml
    # scaffolded for `create-lib java`) is a FAILED create, never a
    # passed one (P0-2 anti-fake-scaffold guard).
    # create: bước scaffold chứng minh dimension create — nhưng chỉ khi
    # project sinh ra khớp ngôn ngữ yêu cầu. Template fallback của ngôn
    # ngữ khác (vd Cargo.toml Rust cho `create-lib java`) là create
    # FAILED, không bao giờ passed (chống scaffold giả P0-2).
    rc, out = run_step(lane["scaffold"])

    # The scaffolded project directory (scaffold NAME arg) is where
    # every lifecycle step runs.
    # Thư mục project được scaffold (đối số NAME) là nơi chạy mọi bước.
    if rc != 0:
        dims["create"] = STATUS_FAILED
        # The scaffold never produced a project — detect was never
        # exercised: unverified, never a pass (fail-closed).
        # (Scaffold chưa sinh được project — detect chưa từng chạy:
        # unverified, không bao giờ pass (fail-closed).)
        dims["detect"] = STATUS_UNVERIFIED
        detail["create_output"] = out[-2000:]
    elif not scaffold_language_matches(
        sandbox, project_dir, lane["language"], lane.get("required_markers")
    ):
        dims["create"] = STATUS_FAILED
        # detect FAILED: the scaffold voice picked the WRONG toolchain —
        # this mismatch IS the detect dimension's failure evidence.
        # (detect FAILED: giọng scaffold chọn SAI toolchain — sự lệch này
        # chính là bằng chứng fail của dimension detect.)
        dims["detect"] = STATUS_FAILED
        found = sorted(
            f for f in os.listdir(os.path.join(sandbox, project_dir))
            if not f.startswith(".")
        )
        detail["create_output"] = (
            f"scaffold fell back to another language (requested "
            f"{lane['language']}); project files: {found}"
        )
    else:
        dims["create"] = STATUS_NATIVE
        # detect PASSED: the project carries a manifest of the REQUESTED
        # language — mgc's own scaffold/detect voice chose the right
        # toolchain (evidence: SCAFFOLD_MARKERS).
        # (detect PASSED: project có manifest của ĐÚNG ngôn ngữ yêu cầu —
        # giọng scaffold/detect của mgc chọn đúng toolchain (bằng chứng:
        # SCAFFOLD_MARKERS).)
        dims["detect"] = STATUS_NATIVE

__all__ = ['scaffold_phase']
