"""Run each declared lane and collect its lifecycle evidence."""

from __future__ import annotations

from .context import MatrixRunContext
from ..catalog.catalog import LANES
from ..contract.ownership import lifecycle_owner_for
from ..runner.runner import run_lane
from ..validation.native_pm import release_scope_out_reason
import sys

def collect_lane_results(context: MatrixRunContext) -> list[dict]:
    """Collect all lane results in catalog order and preserve evidence logs."""
    mgc_bin = context.mgc_bin
    commit = context.commit
    now = context.generated_at
    results = []
    for lane in LANES:
        r = run_lane(mgc_bin, lane)
        # Failed-dimension output is PRINTED (CI log must show WHY a
        # lane failed) and stored in the artifact (the JSON keeps the
        # evidence next to the verdict).
        # Output của dimension fail được IN ra (log CI phải cho thấy vì
        # sao lane fail) và lưu vào artifact (JSON giữ bằng chứng cạnh
        # verdict).
        for key, out in r["detail"].items():
            if key.endswith("_output"):
                print(f"--- {lane['core']}/{lane['language']} {key} ---", file=sys.stderr)
                print(out, file=sys.stderr)
        results.append({
            "core": lane["core"],
            "language": lane["language"],
            "framework_id": lane.get("framework_id", ""),
            "dimensions": r["dims"],
            "required_dimensions": lane["required_dims"],
            "lifecycle_owners": {
                dimension: lifecycle_owner_for(lane, dimension)
                for dimension in lane["required_dims"]
            },
            # P0-6 (Tech Lead 2026-09-15): lanes are SPLIT by gate role —
            # Lifecycle evidence-only lanes stay visible with a source reason;
            # their native-PM claims, when declared, remain strictly gated.
            # (Lane lifecycle evidence-only vẫn hiện cùng lý do từ source;
            # claim native-PM nếu có vẫn bị gate nghiêm ngặt.)
            "evidence_only": bool(lane.get("evidence_only")),
            "release_scope_out_reason": release_scope_out_reason(
                lane["core"], lane["language"], lane.get("framework_id", "")
            ),
            "gate_role": (
                "scoped-out"
                if release_scope_out_reason(
                    lane["core"], lane["language"], lane.get("framework_id", "")
                )
                else "release-blocking"
            ),
            "toolchain_available": r.get("toolchain_available"),
            # Honest dimension rename (P0-mới-2): lanes whose install is
            # proven against a lane-injected fixture (not the template's
            # own dependency set) record that boundary in the dimension
            # NAME — the JSON can never be read as "template ships real
            # deps".
            # Đổi tên dimension trung thực (P0-mới-2): lane có install
            # được chứng minh trên fixture lane tự bơm (không phải
            # dependency set của template) ghi rõ biên đó trong TÊN
            # dimension — JSON không thể bị đọc là "template ship deps thật".
            "install_dim_name": lane.get("install_dim_name"),
            "native_pm_delegated": lane.get("native_pm_delegated", []),
            # Install OWNER (P0-D taxonomy, vòng-9): native-engine |
            # managed-delegation | plain-delegation — the JSON names who
            # owns resolve+lock+fetch+materialize so no verdict or doc
            # can launder delegation into a native claim.
            # (Chủ sở hữu install (taxonomy P0-D): native-engine |
            # managed-delegation | plain-delegation — JSON nêu rõ ai giữ
            # resolve+lock+fetch+materialize để không verdict hay tài
            # liệu nào giặt ủy quyền thành claim native.)
            "install_owner": lane.get(
                "install_owner",
                "plain-delegation" if "install" in lane.get("delegated", []) else "native-engine",
            ),
            # P0-D: who owns the dependency lifecycle for this lane — the
            # verdict gate below reads this field (native-pm-supported
            # REQUIRES mgc-native; delegated caps at compatibility-passed).
            # Emitted only after validate_dependency_owners() passed.
            # (P0-D: ai giữ lifecycle dependency của lane — cổng verdict
            # bên dưới đọc trường này (native-pm-supported BẮT BUỘC
            # mgc-native; delegated trần compatibility-passed). Chỉ emit
            # sau khi validate_dependency_owners() pass.)
            "dependency_owner": lane.get("dependency_owner"),
            # Per-OPERATION owners (Gate 11-C, vòng-11 verdict): resolve/
            # lock/fetch/store/materialize each name their owner — the
            # machine-readable split so "install passed" can never be
            # read as "resolve/lock/store native". Emitted only after
            # validate_lane_owners() passed (the gate ran at main() head).
            # (Owner THEO-TỪNG-OPERATION: resolve/lock/fetch/store/
            # materialize mỗi operation nêu owner — tách máy-đọc-được để
            # "install passed" không bao giờ bị đọc là "resolve/lock/
            # store native". Chỉ emit sau validate_lane_owners() pass
            # (cổng chạy ở đầu main()).)
            "owner_by_operation": lane.get("owner_by_operation", {}),
            "commit": commit,
            "verified_at": now,
            "detail": {
                k: v for k, v in r["detail"].items() if k.endswith("_output")
            },
        })
    return results

__all__ = ["collect_lane_results"]
