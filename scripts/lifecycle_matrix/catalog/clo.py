"""Lifecycle lane profiles for the clo core."""

from __future__ import annotations

LANES = [
    {
        "core": "clo",
        "language": "terraform",
        "evidence_only": True,
        # cloud/terraform lane: terraform CLI is external and usually absent
        # on runners — the probe records unverified instead of faking a run.
        # (lane cloud/terraform: CLI terraform là bên ngoài, thường vắng
        # trên runner — probe ghi unverified thay vì giả một lần chạy.)
        "toolchain_probes": ["terraform"],
        "scaffold": ["create-clo", "terraform", "test-cloud"],
        "steps": [
            ("install", ["install"]),
            ("test", ["test"]),
            ("build", ["build"]),
        ],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "unsupported",
        # The C0 gate rejects Terraform dependency operations; do not label
        # the lane as delegated merely because Terraform exists upstream.
        # (C0 từ chối dependency Terraform; không gắn delegated chỉ vì có
        # tool Terraform ở upstream.)
        "dependency_owner": "unsupported",
        "owner_by_operation": {},
    },
    {
        "core": "clo",
        "language": "cdk",
        "framework_ids": ["cdk"],
        "framework_id": "cdk",
        "scaffold": ["create-clo", "cdk", "test-cloud-cdk"],
        "steps": [("install", ["install"]), ("test", ["test"]), ("build", ["build"])],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {},
    },
    {
        "core": "clo",
        "language": "pulumi",
        "framework_ids": ["pulumi"],
        "framework_id": "pulumi",
        "scaffold": ["create-clo", "pulumi", "test-cloud-pulumi"],
        "toolchain_probes": ["pulumi"],
        "pre_steps": ["initialize_local_pulumi_stack", "mgc_add_real_dependency"],
        # Pulumi's Node.js SDK must be in the package manifest before its
        # CLI can preview the program; use MGC's native Cloud/Web resolver
        # only when the generated template did not already declare it.
        # (SDK Node.js của Pulumi phải có trong manifest trước khi CLI
        # preview; chỉ dùng resolver Cloud/Web native của MGC khi template
        # chưa khai báo sẵn.)
        "dependency_fixture": {
            "command": "add-clo",
            "package": "@pulumi/pulumi@^3.0.0",
            "manifest_path": "package.json",
            "manifest_name": "@pulumi/pulumi",
            "skip_if_declared": True,
        },
        "steps": [("install", ["install"]), ("test", ["test"]), ("build", ["build"])],
        "delegated": [],
        "required_dims": ["create", "install", "test", "build"],
        "install_owner": "native-engine",
        "dependency_owner": "mgc-native",
        "owner_by_operation": {},
    },
]

__all__ = ["LANES"]
