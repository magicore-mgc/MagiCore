//! Typed failures owned by the Cloud build lane.
//! Lỗi typed thuộc lane build Cloud.

use std::fmt;
use std::path::PathBuf;

/// Cloud project/provider selection and toolchain failures.
#[derive(Debug)]
pub(super) enum CloudBuildError {
    FrameworkNotDetected { root: PathBuf },
    ToolchainMissing { tool: &'static str },
    CloudflareBuildInCicdCore,
}

impl fmt::Display for CloudBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FrameworkNotDetected { root } => {
                write!(
                    formatter,
                    "No cloud framework detected in {}",
                    root.display()
                )
            }
            Self::ToolchainMissing { tool } => write!(
                formatter,
                "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
            ),
            Self::CloudflareBuildInCicdCore => write!(
                formatter,
                "cloudflare workers build/deploy belongs to the cicd core (Q12) — cloud core does not handle it as P1"
            ),
        }
    }
}

impl std::error::Error for CloudBuildError {}
