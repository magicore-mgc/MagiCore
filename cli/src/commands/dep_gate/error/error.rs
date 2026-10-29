//! Typed failures owned by the dependency gate — lỗi typed do dependency gate sở hữu.
//! Displays preserve established English diagnostics and remain downcastable through anyhow.
//! Giữ nguyên diagnostic tiếng Anh và có thể downcast qua anyhow.

use std::fmt;

#[derive(Debug)]
pub(crate) enum DepGateError {
    CompatibilityDisabled {
        value: String,
    },
    UnsupportedOperation {
        core: String,
        operation: String,
        ecosystem: Option<String>,
    },
    NativeAdapterIdentityMismatch {
        lane: String,
        operation: String,
        expected_core: String,
        expected_language: String,
        actual_core: String,
        actual_language: String,
    },
}

impl DepGateError {
    pub(crate) fn compat_disabled(value: &str) -> Self {
        Self::CompatibilityDisabled {
            value: value.to_owned(),
        }
    }

    pub(crate) fn unsupported(core: &str, operation: &str, ecosystem: Option<&str>) -> Self {
        Self::UnsupportedOperation {
            core: core.to_owned(),
            operation: operation.to_owned(),
            ecosystem: ecosystem.map(str::to_owned),
        }
    }

    pub(crate) fn identity_mismatch(
        lane: &str,
        operation: &str,
        expected_core: &str,
        expected_language: &str,
        actual_core: &str,
        actual_language: &str,
    ) -> Self {
        Self::NativeAdapterIdentityMismatch {
            lane: lane.to_owned(),
            operation: operation.to_owned(),
            expected_core: expected_core.to_owned(),
            expected_language: expected_language.to_owned(),
            actual_core: actual_core.to_owned(),
            actual_language: actual_language.to_owned(),
        }
    }
}

impl fmt::Display for DepGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CompatibilityDisabled { value } => write!(
                f,
                "--compat-runtime '{value}' is disabled for dependency operations: MagiCore will not invoke an external package manager; use a fully supported native lane"
            ),
            Self::UnsupportedOperation {
                core,
                operation,
                ecosystem,
            } => {
                let lane = match ecosystem {
                    Some(ecosystem) => format!("`{core}` {operation} (ecosystem '{ecosystem}')"),
                    None => format!("`{core}` {operation}"),
                };
                if matches!(core.as_str(), "cicd" | "hardware") {
                    write!(
                        f,
                        "{lane} has no package lifecycle — this is a scaffold-only lane (templates/pipelines/generators), never a native package install"
                    )
                } else if operation == "list" {
                    // `list` is refused for a reason the old text got wrong. The
                    // manifest is NOT toolchain-owned — `mgc add` writes it. What
                    // is missing is a verified installed-artifact inventory: a
                    // declared dependency that `MagiCore` resolved but never
                    // materialised is not installed, and reporting it as installed
                    // would be worse than refusing.
                    // (`list` bị từ chối vì lý do text cũ nói SAI: manifest KHÔNG
                    // thuộc toolchain — `mgc add` ghi nó. Cái thiếu là inventory
                    // artefact đã cài: dependency khai báo nhưng chưa materialize
                    // không phải đã cài, báo là đã cài còn tệ hơn từ chối.)
                    write!(
                        f,
                        "{lane} has no verified installed-artifact inventory yet — MagiCore owns the manifest and lock, but a declared or locked package is not proof it was materialised; use `mgc install --dry-run` to inspect what would be installed"
                    )
                } else if !matches!(core.as_str(), "iot" | "game" | "clo" | "cloud") {
                    // Only iot/game/clo have a manifest their toolchain owns
                    // (platformio.ini, west.yml, CDK config). Every other core
                    // here — web, lib, ai, app — has a MagiCore-owned manifest, so
                    // "toolchain-owned" would be false and would push the user
                    // toward calling the package manager directly.
                    // (Chỉ iot/game/clo có manifest toolchain sở hữu. Mọi core
                    // khác ở đây có manifest do MagiCore sở hữu, nên
                    // "toolchain-owned" là sai và đẩy người dùng gọi PM trực tiếp.)
                    write!(
                        f,
                        "{lane} is unsupported for dependency lifecycle — MagiCore owns the manifest, but no approved runner implements this operation yet"
                    )
                } else {
                    // Game/iot/clo manifests ARE toolchain-owned (platformio.ini,
                    // west.yml, CDK config): the owning toolchain must perform the
                    // operation, and MagiCore refuses rather than delegate blindly.
                    // (Manifest game/iot/clo THUỘC toolchain: chính toolchain đó
                    // phải thực hiện, MagiCore từ chối thay vì delegate mù.)
                    write!(
                        f,
                        "{lane} is unsupported for dependency lifecycle — the manifest is toolchain-owned, but no approved compatibility runner is configured for this operation"
                    )
                }
            }
            Self::NativeAdapterIdentityMismatch {
                lane,
                operation,
                expected_core,
                expected_language,
                actual_core,
                actual_language,
            } => write!(
                f,
                "native adapter identity mismatch for {lane} {operation}: expected {expected_core}/{expected_language}, got {actual_core}/{actual_language}"
            ),
        }
    }
}

impl std::error::Error for DepGateError {}

#[cfg(test)]
#[path = "test/mod.rs"]
mod local_tests;
