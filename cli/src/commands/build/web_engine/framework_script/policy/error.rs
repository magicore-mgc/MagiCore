//! Typed failures owned by Web build-script policy.
//! Lỗi typed thuộc policy script build Web.

use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub(super) enum FrameworkScriptError {
    ForbiddenPackageManager {
        script: String,
        manifest: PathBuf,
        manager: String,
    },
}

impl fmt::Display for FrameworkScriptError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForbiddenPackageManager {
                script,
                manifest,
                manager,
            } => write!(
                formatter,
                "Unsupported script '{script}' in '{}': it delegates to '{manager}'. Core-web must execute natively through MagiCore or framework-local binaries, not through another package manager.",
                manifest.display()
            ),
        }
    }
}

impl std::error::Error for FrameworkScriptError {}
