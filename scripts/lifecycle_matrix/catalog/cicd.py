"""Lifecycle lane profiles for the cicd core."""

from __future__ import annotations

LANES = [
    {
        "core": "cicd",
        "language": "github-actions",
        "evidence_only": True,
        # github-actions lane: the scaffold writes .github/workflows/ci.yml
        # (mgc-native file generation, marker below); lifecycle install/test/
        # build steps record what actually happens today — evidence, not a
        # gate.
        # (lane github-actions: scaffold ghi .github/workflows/ci.yml (sinh
        # file bởi mgc — marker dưới); các bước install/test/build ghi đúng
        # cái xảy ra hôm nay — bằng chứng, không phải gate.)
        "toolchain_probes": [],
        "scaffold": ["create-cicd", "github-actions", "test-cicd"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        # P0-5 (Tech Lead verdict 2026-09-16): the previous `native-engine`
        # + mgc-owned resolve/fetch claim was a taxonomy FALSE-POSITIVE —
        # the production adapter returns MgError::Unsupported for the
        # registry surface: CicdAdapter::resolve (adapter.rs:65), ::fetch
        # (adapter.rs:75), ::install (adapter.rs:83), ::write_manifest
        # (adapter.rs:52), and ::add/::remove/::update all fail with
        # "cicd has no package manager". CI/CD is a GENERATOR lane: the
        # scaffold writes .github/workflows/ci.yml once, pipeline files are
        # HAND-OWNED afterwards (adapter guidance), and execution lives on
        # the CI provider — mgc manages no cache or toolchain here. Of the
        # three owner labels this is the most honest: everything past the
        # scaffold belongs to an unmanaged external system (the provider +
        # humans), which is exactly plain-delegation. Per-operation owners
        # name the truth: registry ops are `unsupported-…`, store/
        # materialize belong to the CI provider.
        # (P0-5: claim cũ `native-engine` + resolve/fetch của mgc là
        # false-positive taxonomy — adapter production trả
        # MgError::Unsupported cho mặt registry: resolve/fetch/install/
        # write_manifest (adapter.rs:65/75/83/52), add/remove/update đều
        # fail "cicd has no package manager". cicd là lane GENERATOR:
        # scaffold ghi .github/workflows/ci.yml một lần, file pipeline do
        # NGƯỜI quản sau đó (guidance của adapter), thực thi nằm ở CI
        # provider — mgc không quản cache/toolchain nào ở đây. Trong 3
        # nhãn đây là nhãn trung thực nhất: mọi thứ sau scaffold thuộc hệ
        # thống ngoài không quản lý (provider + con người) — đúng nghĩa
        # plain-delegation. Owner per-operation nói thẳng: op registry là
        # `unsupported-…`, store/materialize thuộc CI provider.)
        "install_owner": "unsupported",
        # P0-D: cicd is a generator lane (P0-5 — the adapter returns
        # Unsupported for the registry surface; pipelines are hand-owned
        # afterwards): scaffold-only.
        # (P0-D: cicd là lane generator (P0-5 — adapter trả Unsupported
        # cho mặt registry; pipeline do người quản sau đó): scaffold-only.)
        "dependency_owner": "scaffold-only",
        "owner_by_operation": {},
    },
]

__all__ = ["LANES"]
