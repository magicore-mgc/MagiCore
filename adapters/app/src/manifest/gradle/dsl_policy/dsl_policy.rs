//! Fail-closed policy for executable Gradle build scripts.
//! Policy fail-closed cho build script Gradle có thể thực thi.

use std::path::Path;

use super::super::error::GradleManifestError;

const PARSE_CAPABILITY: &str = "parse-gradle-manifest";
const WRITE_CAPABILITY: &str = "write-gradle-manifest";

/// Return the existing parse error selected by the legacy `.kts`-first policy.
/// Trả lỗi parse hiện tại theo policy cũ ưu tiên `.kts`.
pub(in crate::manifest::gradle) fn parse_error(project_root: &Path) -> GradleManifestError {
    let kotlin_path = project_root.join("build.gradle.kts");
    let path = if kotlin_path.exists() {
        kotlin_path
    } else {
        project_root.join("build.gradle")
    };

    if !path.exists() {
        return GradleManifestError::Other("build.gradle not found".to_string());
    }

    GradleManifestError::Unsupported {
        core: "app",
        capability: PARSE_CAPABILITY,
        guidance: format!(
            concat!(
                "{} is executable Gradle DSL; MagiCore does not parse it as a dependency manifest. ",
                "Use only the version-catalog operations MagiCore explicitly supports; ",
                "other dependency operations are unsupported"
            ),
            path.display()
        ),
    }
}

/// Return the existing refusal for generic writes to executable Gradle DSL.
/// Trả lỗi từ chối hiện tại đối với thao tác ghi chung lên DSL Gradle.
pub(in crate::manifest::gradle) fn write_error() -> GradleManifestError {
    GradleManifestError::Unsupported {
        core: "app",
        capability: WRITE_CAPABILITY,
        guidance: concat!(
            "MagiCore does not rewrite executable Gradle DSL; only explicitly supported ",
            "version-catalog operations are available, and other dependency mutations are unsupported"
        )
        .to_string(),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
