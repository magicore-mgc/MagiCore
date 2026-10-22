"""Lifecycle operations and required operation sets."""

from __future__ import annotations


ALL_DEPENDENCY_OPERATIONS = (
    "install", "add", "remove", "update", "list", "resolve", "lock",
    "fetch", "verify", "store", "materialize", "frozen-install",
    "offline-reinstall", "gc",
)

NATIVE_INSTALL_PIPELINE_OPERATIONS = frozenset(
    {"resolve", "lock", "fetch", "verify", "store", "materialize", "frozen-install"}
)

NATIVE_OPERATIONS_EXCEPT_GC = frozenset(ALL_DEPENDENCY_OPERATIONS) - {"gc"}

NATIVE_OPERATIONS_EXCEPT_GC_AND_OFFLINE = frozenset(ALL_DEPENDENCY_OPERATIONS) - {
    "gc", "offline-reinstall"
}

# Per-lane user-operation owners are declarations, not runtime evidence.
# They mirror `DepOp::owner_for` and are compared with `mgc capabilities`
# during the real-binary matrix run. Unprobed operation dimensions remain
# `unverified` below, so this metadata can never promote a lane by itself.
NATIVE_USER_OPERATION_LANES = {
    ("web", "javascript"): set(ALL_DEPENDENCY_OPERATIONS),
    ("web", "vanilla"): set(ALL_DEPENDENCY_OPERATIONS),
    ("web", "ts"): set(ALL_DEPENDENCY_OPERATIONS),
    ("web", "node"): set(ALL_DEPENDENCY_OPERATIONS),
    ("ai", "python"): set(NATIVE_OPERATIONS_EXCEPT_GC),
    ("app", "flutter"): {"install", "add", "remove", "update"},
    ("app", "swift"): {"install"},
    ("lib", "rust"): {"install", "add", "remove", "update"},
    ("lib", "python"): {"install", "add", "remove", "update", "list"},
    ("lib", "typescript"): set(NATIVE_OPERATIONS_EXCEPT_GC),
    ("lib", "go"): {"install", "add", "remove", "update"},
    ("lib", "java"): {"install", "add", "remove", "update"},
    ("lib", "dotnet"): {"install", "add", "remove", "update"},
    ("game", "rust"): {"install", "add", "remove", "update"},
    ("iot", "rust"): {"install", "add", "remove", "update"},
    ("clo", "terraform"): set(),
    ("clo", "cdk"): set(NATIVE_OPERATIONS_EXCEPT_GC),
    ("clo", "pulumi"): set(NATIVE_OPERATIONS_EXCEPT_GC),
    ("hardware", "benchmark"): set(),
    ("cicd", "github-actions"): set(),
    ("app", "objc"): set(),
    ("app", "react-native"): set(),
}

# A package-manager ownership claim needs evidence for the dependency engine,
# not just the surrounding create/test/build workflow.
# (Claim sở hữu package manager cần evidence cho dependency engine, không
# chỉ cho workflow create/test/build bao quanh.)
NATIVE_PM_USER_OPERATIONS = (
    "install",
    "add",
    "remove",
    "update",
    "list",
    "frozen-install",
    "offline-reinstall",
    "gc",
)

NATIVE_PM_REQUIRED_DIMENSIONS = (
    *NATIVE_PM_USER_OPERATIONS,
    "resolve",
    "lock",
    "fetch",
    "verify",
    "store",
    "materialize",
)

# Operations whose per-operation owner every lane MUST declare (Gate
# 11-C, vòng-11): one install_owner label per lane is NOT enough to
# describe an ecosystem — the audit demands the capability split.
# (Các operation mà mỗi lane PHẢI khai báo owner riêng (Gate 11-C): một
# nhãn install_owner mỗi lane KHÔNG đủ để mô tả ecosystem — audit đòi
# tách năng lực.)
REQUIRED_OWNER_OPERATIONS = list(ALL_DEPENDENCY_OPERATIONS)

__all__ = ['ALL_DEPENDENCY_OPERATIONS', 'NATIVE_INSTALL_PIPELINE_OPERATIONS', 'NATIVE_OPERATIONS_EXCEPT_GC', 'NATIVE_OPERATIONS_EXCEPT_GC_AND_OFFLINE', 'NATIVE_PM_REQUIRED_DIMENSIONS', 'NATIVE_PM_USER_OPERATIONS', 'NATIVE_USER_OPERATION_LANES', 'REQUIRED_OWNER_OPERATIONS']
