//! Typed failures owned by the IoT build lane.
//! Lỗi typed thuộc lane build IoT.

use std::fmt;
use std::path::PathBuf;

/// Fail-closed provider refusal, toolchain and project selection diagnostics.
#[derive(Debug)]
pub(super) enum IotBuildError {
    Unsupported {
        target: &'static str,
        guidance: &'static str,
    },
    ToolchainMissing {
        tool: &'static str,
    },
    FrameworkNotDetected {
        root: PathBuf,
    },
}

impl fmt::Display for IotBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported { target, guidance } => write!(
                formatter,
                "`mgc build` is not supported for core '{target}': {guidance}"
            ),
            Self::ToolchainMissing { tool } => write!(
                formatter,
                "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
            ),
            Self::FrameworkNotDetected { root } => {
                write!(formatter, "No iot framework detected in {}", root.display())
            }
        }
    }
}

impl std::error::Error for IotBuildError {}
