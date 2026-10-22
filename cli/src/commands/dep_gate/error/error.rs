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
                } else {
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
