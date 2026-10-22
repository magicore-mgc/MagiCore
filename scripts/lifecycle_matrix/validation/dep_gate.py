"""Binary dep-gate route verification."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..contract.operations import (
    ALL_DEPENDENCY_OPERATIONS,
)
from ..evidence.records import (
    FRAMEWORK_QUALIFICATION_EVIDENCE,
)
from ..process.process import (
    run_text_capture,
)
import json
import os
import subprocess
import sys

def check_dep_gate_consistency(binary_ownership: dict, lanes: list) -> list:
    """Cross-check every package operation against the compiled CLI truth.
    A coarse `install` match is insufficient: CRUD/list/frozen/offline/GC
    can differ by ecosystem. Missing binary cells and missing lane owners
    fail closed; an unsupported binary cell cannot be laundered as native.
    (Đối chiếu thuần: `dependency_owner` của lane matrix với bảng của
    binary. Cả hai chiều đều fail.)
    `binary_ownership`: {core: {"operations": {op: owner}, "languages":
    {lang: {op: owner}}, "frameworks": {id: {op: owner}}}. A lane with
    `framework_id` must match that exact qualification record; it cannot
    fall back to a broader language/core owner.
    Matrix lane languages map to gate languages (typescript->ts,
    react-native->rn, others identical).
    Returns the violation list (empty = consistent)."""
    def matches(binary_owner, matrix_owner):
        if binary_owner == "mgc-native":
            return matrix_owner == "mgc" or matrix_owner == "magicore-shared-cas"
        if binary_owner == "scaffold-only":
            return matrix_owner == "scaffold-only"
        return binary_owner == "unsupported" and str(matrix_owner).startswith("unsupported")
    # Matrix-taxonomy → gate-ecosystem ids, PER CORE: the matrix names a
    # lane by its toolchain language while the firewall matches the
    # lane's detected ecosystem id. (lib, typescript) scaffolds a
    # TypeScript project (gate "ts"); (app, react-native) scaffolds an RN
    # project (gate "rn"); (game, rust) scaffolds BEVY (gate "bevy");
    # (iot, rust) scaffolds esp32-rust (gate "esp32-rust"). Every other
    # (core, language) pair already spells the gate id.
    language_alias = {
        ("web", "javascript"): "js",
        ("web", "typescript"): "ts",
        ("lib", "typescript"): "ts",
        ("app", "react-native"): "rn",
        ("game", "rust"): "bevy",
        ("iot", "rust"): "esp32-rust",
    }
    violations = []
    for lane in lanes:
        tag = f"{lane['core']}/{lane['language']}"
        claimed = lane.get("dependency_owner")
        entry = binary_ownership.get(lane["core"])
        if entry is None:
            violations.append(f"{tag}: core missing from binary ownership table")
            continue
        gate_lang = language_alias.get((lane["core"], lane["language"]), lane["language"])
        truth_ops = None
        manifest_variant = lane.get("manifest_variant")
        if manifest_variant:
            truth_ops = entry.get("variants", {}).get(manifest_variant)
        framework_id = lane.get("framework_id")
        if framework_id:
            truth_ops = entry.get("frameworks", {}).get(framework_id)
            if not isinstance(truth_ops, dict):
                violations.append(
                    f"{tag}: framework '{framework_id}' missing from binary qualification table"
                )
                continue
        if truth_ops is None:
            truth_ops = entry.get("languages", {}).get(gate_lang)
        if truth_ops is None:
            truth_ops = entry.get("operations")
        if not isinstance(truth_ops, dict):
            violations.append(f"{tag}: operation ownership missing from binary table")
            continue
        matrix_ops = lane.get("owner_by_operation", {})
        binary_install = truth_ops.get("install")
        expected_claim = {
            "mgc-native": "mgc-native",
            "scaffold-only": "scaffold-only",
            "unsupported": "unsupported",
        }.get(binary_install)
        claim_matches = claimed == expected_claim or (
            binary_install == "unsupported" and claimed == "scaffold-only"
        )
        if not claim_matches:
            violations.append(
                f"{tag}: matrix claims '{claimed}' but binary install owner is "
                f"'{binary_install or 'missing'}'"
            )
        for operation in ALL_DEPENDENCY_OPERATIONS:
            binary_owner = truth_ops.get(operation)
            matrix_owner = matrix_ops.get(operation)
            if binary_owner is None:
                violations.append(f"{tag}: binary operation owner missing '{operation}'")
            elif matrix_owner is None:
                violations.append(f"{tag}: matrix operation owner missing '{operation}'")
            elif not matches(binary_owner, matrix_owner):
                violations.append(
                    f"{tag}: operation '{operation}' matrix owner '{matrix_owner}' "
                    f"does not match binary owner '{binary_owner}'"
                )
    return violations

def validate_dep_gate_consistency(mgc_bin=None) -> int:
    """T0.4 binary↔matrix consistency gate: shell `mgc capabilities` and
    require every lane's `dependency_owner` to agree with the C0 firewall
    table for the install op. It also captures the complete framework
    qualification inventory from this compiled binary; catalogued
    frameworks without native ownership and a lifecycle lane stay visible
    and block promotion. No binary fails closed in release-gating runs.
    (Cổng nhất quán T0.4: gọi `mgc capabilities` và bắt mọi lane khớp
    bảng tường lửa C0 ở op install. Không có binary → ghi UNAVAILABLE
    trung thực.)"""
    import subprocess

    # An explicitly selected binary is part of the evidence identity. Never
    # replace a missing path with a different local binary: that would let a
    # stale/debug binary silently satisfy a CI or release check.
    # (Binary được chỉ định là một phần identity của evidence; không fallback
    # sang binary debug khác khi đường dẫn được chọn bị thiếu.)
    env_bin = os.environ.get("MGC_BIN")
    candidates = (
        [mgc_bin]
        if mgc_bin is not None
        else [env_bin]
        if env_bin
        else ["./target/debug/mgc"]
    )
    binary = next((c for c in candidates if os.path.isfile(c)), None)
    if binary is None:
        if (
            os.environ.get("MGC_ALLOW_MISSING_DEP_GATE_BINARY") == "1"
            and not os.environ.get("CI")
        ):
            print(
                "dep-gate cross-check: STATIC-ONLY (explicit local opt-in; "
                "not release evidence)"
            )
            return 0
        print(
            "DEP-GATE CROSS-CHECK VIOLATION: no compiled mgc binary; "
            "runtime ownership validation is mandatory",
            file=sys.stderr,
        )
        return 1
    try:
        raw = run_text_capture(
            [binary, "capabilities"], capture_output=True, text=True, timeout=120
        )
    except (OSError, subprocess.SubprocessError) as err:
        print(f"DEP-GATE CROSS-CHECK VIOLATION: cannot run binary: {err}", file=sys.stderr)
        return 1
    if raw.returncode != 0:
        print(
            "DEP-GATE CROSS-CHECK VIOLATION: `mgc capabilities` failed: "
            f"{raw.stderr.strip()}",
            file=sys.stderr,
        )
        return 1
    global FRAMEWORK_QUALIFICATION_EVIDENCE
    try:
        payload = json.loads(raw.stdout)
        ownership = {}
        framework_rows = []
        for core in payload["cores"]:
            table = core["dependency_ownership"]
            def owners(cells):
                return {operation: value["owner"] for operation, value in cells.items()}

            entry = {
                "operations": owners(table["operations"]),
                "languages": {
                    language: owners(cells)
                    for language, cells in table.get("languages", {}).items()
                },
                "frameworks": {
                    row["framework"]: owners(row["dependency_ownership"])
                    for row in core.get("framework_qualification", [])
                },
            }
            for framework in core.get("framework_qualification", []):
                framework_rows.append({
                    "core": core["core"],
                    "framework": framework["framework"],
                    "status": framework["status"],
                    "dependency_ownership": framework["dependency_ownership"],
                    "evidence": framework.get("evidence", ""),
                    "required_manifest": framework.get("required_manifest"),
                })
            for variant, cells in table.get("manifest_variants", {}).items():
                language = variant.split("/", 1)[0]
                entry.setdefault("variants", {})[variant] = owners(cells)
            ownership[core["core"]] = entry
        FRAMEWORK_QUALIFICATION_EVIDENCE = sorted(
            framework_rows, key=lambda row: (row["core"], row["framework"])
        )
    except (ValueError, KeyError, TypeError) as err:
        print(
            "DEP-GATE CROSS-CHECK VIOLATION: unparseable capabilities "
            f"payload: {err}",
            file=sys.stderr,
        )
        return 1
    violations = check_dep_gate_consistency(ownership, LANES)
    if violations:
        for v in violations:
            print(f"DEP-GATE CROSS-CHECK VIOLATION: {v}", file=sys.stderr)
        return 1
    print(
        f"dep-gate cross-check: {len(LANES)} lanes agree with the binary; "
        f"captured {len(FRAMEWORK_QUALIFICATION_EVIDENCE)} framework records"
    )
    return 0

__all__ = ['check_dep_gate_consistency', 'validate_dep_gate_consistency']
