"""Restore source-declared per-operation ownership for lifecycle profiles.

Khôi phục ownership theo từng operation, không tạo bằng chứng runtime.
"""

from ..contract.operations import (
    ALL_DEPENDENCY_OPERATIONS,
    NATIVE_INSTALL_PIPELINE_OPERATIONS,
    NATIVE_USER_OPERATION_LANES,
)


def apply_operation_owners(lanes):
    """Apply the existing lane contract before consumers inspect the catalog.

    Áp dụng contract baseline trước khi consumer đọc danh mục lane.
    """
    for _lane in lanes:
        _key = (_lane["core"], _lane["language"])
        if _key not in NATIVE_USER_OPERATION_LANES:
            raise ValueError(f"dependency operation ownership missing for lane {_key}")
        _native_ops = NATIVE_USER_OPERATION_LANES[_key]
        # Offline reinstall is its own capability: Web/TS, Lib/Python wheel
        # cache, AI/Python wheels, and Lib/Rust crate cache enforce cache-only installation. Other
        # native online installers still download from registries and must not
        # inherit an offline claim.
        _offline_native = _key in {
            ("web", "javascript"), ("web", "typescript"), ("web", "vanilla"),
            ("web", "ts"), ("web", "node"), ("lib", "typescript"),
            ("lib", "python"), ("ai", "python"), ("lib", "rust"), ("clo", "cdk"), ("clo", "pulumi"),
            ("app", "flutter"),
        }
        _lane["dependency_owner"] = (
            "mgc-native" if "install" in _native_ops
            else "scaffold-only" if _key == ("hardware", "benchmark")
            else "unsupported"
        )
        _lane["install_owner"] = (
            "native-engine" if "install" in _native_ops
            else "unsupported"
        )
        _owners = {}
        for _op in ALL_DEPENDENCY_OPERATIONS:
            if _key == ("hardware", "benchmark") and _op == "list":
                _owners[_op] = "scaffold-only"
            elif _op == "offline-reinstall" and _offline_native:
                _owners[_op] = "mgc"
            elif _op in _native_ops or (
                _op in NATIVE_INSTALL_PIPELINE_OPERATIONS and "install" in _native_ops
            ):
                _owners[_op] = "magicore-shared-cas" if _op == "store" else "mgc"
            else:
                _owners[_op] = "unsupported-no-runner"
        _lane["owner_by_operation"] = _owners
        _manifest_variants = {
            ("lib", "rust"): "rust/cargo-toml",
            ("lib", "python"): "python/pep621-native",
            ("lib", "typescript"): "ts/package-json",
            ("lib", "go"): "go/go-mod",
            ("lib", "java"): "java/maven-pom",
            ("lib", "dotnet"): "dotnet/csproj",
            ("ai", "python"): "python/mgc-pyproject",
        }
        if _key in _manifest_variants:
            _lane["manifest_variant"] = _manifest_variants[_key]
