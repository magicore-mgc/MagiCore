"""Lifecycle lane profiles for the hardware core."""

from __future__ import annotations

LANES = [
    {
        "core": "hardware",
        "language": "benchmark",
        "evidence_only": True,
        # hardware lane: the core's frameworks are `optimizer`/`bench` —
        # mgc-native benchmark tooling, no FPGA vendor toolchain exists in
        # this core yet. No external probe (mgc owns the harness); the
        # `fpga` toolchain question stays open and honestly unclaimed.
        # (lane hardware: framework của core là `optimizer`/`bench` — công
        # cụ benchmark của mgc, CHƯA có toolchain FPGA vendor nào ở core
        # này. Không probe ngoài (mgc giữ harness); câu hỏi toolchain `fpga`
        # còn mở và trung thực không claim.)
        "toolchain_probes": [],
        "scaffold": ["create-hardware", "bench", "test-hw"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        # P0-5 (Tech Lead verdict 2026-09-16): the previous `native-engine`
        # + mgc-owned resolve/fetch claim was a taxonomy FALSE-POSITIVE —
        # the production adapter returns MgError::Unsupported for the whole
        # registry surface: HardwareAdapter::resolve (adapter.rs:55),
        # ::fetch (adapter.rs:67), ::install (adapter.rs:77),
        # ::update (adapter.rs:111), ::write_manifest (adapter.rs:41).
        # Hardware is a SCAFFOLD/GENERATOR-ONLY core: `mgc add-hardware
        # <pkg>` materializes bundled templates into optimizer/ + bench/,
        # nothing ever resolves or fetches. Of the three owner labels this
        # is the most honest: the ONLY machinery that materializes is mgc's
        # own in-tree, mgc-managed template generator — no unmanaged
        # external toolchain exists (toolchain_probes below is empty), so
        # plain-delegation would be literally false; managed-delegation
        # captures "mgc manages the machinery" without claiming the
        # registry pipeline. Per-operation owners name the truth: registry
        # ops are `unsupported-…`, store/materialize name the generator.
        # (P0-5: claim cũ `native-engine` + resolve/fetch của mgc là
        # false-positive taxonomy — adapter production trả
        # MgError::Unsupported cho toàn bộ mặt registry: resolve/fetch/
        # install/update/write_manifest (adapter.rs:55/67/77/111/41).
        # hardware là core CHỈ-SCAFFOLD/GENERATOR: `mgc add-hardware <pkg>`
        # materialize template bundled vào optimizer/ + bench/, không gì
        # resolve hay fetch cả. Trong 3 nhãn hiện có đây là nhãn trung thực
        # nhất: cơ khí DUY NHẤT materialize là template generator trong-
        # tree do chính mgc quản — không tồn tại toolchain ngoài nào
        # (toolchain_probes bên dưới rỗng), nên plain-delegation sẽ sai
        # đen; managed-delegation bắt được nghĩa "mgc quản cơ khí" mà
        # không claim pipeline registry. Owner per-operation nói thẳng:
        # op registry là `unsupported-…`, store/materialize nêu generator.)
        "install_owner": "unsupported",
        # P0-D: hardware is a scaffold/generator core (P0-5 — the adapter
        # returns Unsupported for the whole registry surface): the only
        # materialization is mgc's own bundled-template generator, so the
        # honest owner is scaffold-only, NOT delegated (no external
        # toolchain) and NOT mgc-native (no registry lifecycle).
        # (P0-D: hardware là core scaffold/generator (P0-5 — adapter trả
        # Unsupported cho toàn bộ mặt registry): cơ khí duy nhất là
        # generator template bundled của mgc, nên owner trung thực là
        # scaffold-only, KHÔNG phải delegated (không toolchain ngoài) và
        # KHÔNG phải mgc-native (không lifecycle registry).)
        "dependency_owner": "scaffold-only",
        "owner_by_operation": {},
    },
]

__all__ = ["LANES"]
