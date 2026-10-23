"""Adapter capability and CLI route verification."""

from __future__ import annotations

from ..catalog.catalog import (
    LANES,
)
from ..paths.paths import (
    REPOSITORY_ROOT,
)
import os
import sys

# ---------------------------------------------------------------------------
# P0-5 adapter-capability ground truth (Tech Lead verdict 2026-09-16):
# hardcoded from the PRODUCTION adapters — what each core's PackageAdapter
# actually returns for the registry lifecycle surface. The lane taxonomy
# must match this table, never the aspiration (fail-closed gate below).
# Evidence per row (dẫn chứng từng dòng):
#   web   → TRUE×3: the native engine IS the registry pipeline
#           (impl PackageAdapter at adapters/web/src/lib.rs:265, install/
#           orchestrator + CAS + mgc.lock).
#   lib   → TRUE×3: TRUE for the TS lane — LibAdapter delegates resolve/
#           fetch to the embedded web engine (adapters/lib/src/adapter).
#           P0-B (2026-09-16): resolve/fetch now FAIL CLOSED for the
#           toolchain-owned languages (rust/python/go/java/dotnet) — the
#           core-level row stays TRUE because the TS lane's mgc-owned ops
#           are real; delegated lib lanes never claim mgc owners.
#   ai    → TRUE×3: implemented fail-closed (requires uv.lock /
#           requirements.lock) — adapters/ai/src/adapter:57/61/65.
#   app   → TRUE×3: implemented (flutter pub orchestration) —
#           adapters/app/src/adapter:52/56/60.
#   game  → adapter protocol lacks generic resolve/fetch, but the Bevy CLI
#           route invokes the Lib/Rust native engine (install/game.rs).
#   iot   → adapter protocol lacks generic resolve/fetch, but esp32-rust
#           CLI route invokes the Lib/Rust native engine (install/iot.rs).
#   clo   → resolve/fetch FALSE outside the embedded web engine —
#           terraform/CDK modules are fetched by `terraform init`
#           (adapters/cloud/src/adapter:78/93); install TRUE
#           (adapter.rs:106).
#   hardware → FALSE×3: MgError::Unsupported for resolve/fetch/install
#           (adapters/hardware/src/adapter:55/67/77), plus
#           write_manifest (:41) and update (:111).
#   cicd  → FALSE×3: MgError::Unsupported for resolve/fetch/install
#           (adapters/cicd/src/adapter:65/75/83), plus write_manifest
#           (:52); add/remove/update → "cicd has no package manager".
# (Bảng năng lực adapter theo core; CLI route override phải được nêu đích
# danh vì một adapter Unsupported không phủ định một route MGC khác.)
# ---------------------------------------------------------------------------
ADAPTER_REGISTRY_CAPABILITIES = {
    "web":      {"resolve": True,  "fetch": True,  "install": True},
    "lib":      {"resolve": True,  "fetch": True,  "install": True},
    "ai":       {"resolve": True,  "fetch": True,  "install": True},
    "app":      {"resolve": True,  "fetch": True,  "install": True},
    "game":     {"resolve": False, "fetch": False, "install": True},
    "iot":      {"resolve": False, "fetch": False, "install": True},
    "clo":      {"resolve": False, "fetch": False, "install": True},
    "hardware": {"resolve": False, "fetch": False, "install": False},
    "cicd":     {"resolve": False, "fetch": False, "install": False},
}

# Some CLI dependency lanes intentionally route through the shared Lib
# resolver instead of their core adapter. Keep these explicit so a core
# adapter's unsupported package trait cannot erase (or invent) the actual
# CLI implementation route. Runtime capabilities cross-check is still
# required before treating this table as evidence.
CLI_NATIVE_DEPENDENCY_ROUTES = {
    ("game", "rust"): "cli/src/commands/core/install/game.rs -> Lib/Rust adapter",
    ("iot", "rust"): "cli/src/commands/core/install/iot.rs -> Lib/Rust adapter",
    # Cloud CDK/Pulumi embed the MGC WebAdapter for JS package lifecycle;
    # this exception is framework-scoped. Terraform/Cloudflare remain
    # unsupported and do not inherit the cloud core's embedded route.
    # (Cloud CDK/Pulumi nhúng WebAdapter của MGC cho JS package lifecycle;
    # ngoại lệ theo framework, không áp dụng cho Terraform/Cloudflare.)
    ("clo", "cdk"): "adapters/cloud/src/adapter/content_store/content_store.rs -> embedded WebAdapter",
    ("clo", "pulumi"): "adapters/cloud/src/adapter/content_store/content_store.rs -> embedded WebAdapter",
}

def validate_adapter_consistency() -> int:
    """Cross-check the lane against its adapter or explicit CLI route.
    Generic adapter capability cannot disprove a separate MGC-native CLI
    route, but such an exception must name a real source file and is still
    checked operation-by-operation against the compiled binary.
    (Đối chiếu lane với adapter hoặc CLI route tường minh. Adapter chung
    không phủ định route MGC riêng, nhưng ngoại lệ phải trỏ tới source thật
    và vẫn bị cross-check từng operation với binary.)"""
    violations = []
    for lane in LANES:
        tag = f"{lane['core']}/{lane['language']}"
        caps = ADAPTER_REGISTRY_CAPABILITIES.get(lane["core"])
        if caps is None:
            # Unknown core row = the taxonomy cannot be verified against
            # the adapter — fail-closed instead of assuming.
            # (Core chưa có dòng bảng = không verify được taxonomy với
            # adapter — fail-closed thay vì giả định.)
            violations.append(
                f"{tag}: core '{lane['core']}' missing from "
                "ADAPTER_REGISTRY_CAPABILITIES — add the adapter's real row"
            )
            continue
        route_source = CLI_NATIVE_DEPENDENCY_ROUTES.get((lane["core"], lane["language"]))
        if route_source:
            # These CLI paths are verified against the binary's DepGate
            # operation table below; the generic adapter itself remains
            # unsupported for the registry protocol surface.
            source_path = route_source.split(" ->", 1)[0]
            source_path = os.path.join(
                REPOSITORY_ROOT,
                source_path,
            )
            if not os.path.isfile(source_path):
                violations.append(
                    f"{tag}: declared native CLI route source is missing: {route_source}"
                )
            continue
        owner = lane.get("install_owner", "")
        ops = lane.get("owner_by_operation", {})
        if not caps["install"] and owner == "native-engine":
            violations.append(
                f"{tag}: install_owner=native-engine but the production "
                "adapter returns Unsupported for install — false-positive "
                "taxonomy (P0-5)"
            )
        for op in ("resolve", "fetch"):
            op_owner = str(ops.get(op, ""))
            if not caps[op] and op_owner.startswith(("mgc", "magicore")):
                violations.append(
                    f"{tag}: owner_by_operation['{op}']='{op_owner}' but the "
                    f"production adapter returns Unsupported for {op} — "
                    "false-positive taxonomy (P0-5)"
                )
    if violations:
        for v in violations:
            print(f"ADAPTER CONSISTENCY GATE VIOLATION: {v}", file=sys.stderr)
        return 1
    print(
        f"adapter/CLI-route consistency gate: {len(LANES)} lanes have "
        "a declared implementation route (P0-5)"
    )
    return 0

__all__ = ['ADAPTER_REGISTRY_CAPABILITIES', 'CLI_NATIVE_DEPENDENCY_ROUTES', 'validate_adapter_consistency']
