use crate::commands::build::implementation::project_manifest_present;
use anyhow::Result;
use std::path::Path;

/// First `*.csproj` directly inside the project root (SDK-style layout).
/// (File `*.csproj` đầu tiên ngay trong root project.)
#[cfg(feature = "lib")]
pub(in crate::commands::build::implementation) fn find_local_csproj(
    root: &std::path::Path,
) -> Result<Option<std::path::PathBuf>> {
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("csproj"))
            && mgc_config::project::read_regular_project_text(&path, "C# project manifest")?
                .is_some()
        {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

/// Detect Java build descriptors before the TypeScript fallback.
/// Nhận diện build descriptor Java trước nhánh fallback TypeScript.
#[cfg(feature = "lib")]
pub(in crate::commands::build::implementation) fn has_java_build_manifest(
    root: &Path,
) -> Result<bool> {
    [
        "pom.xml",
        "build.gradle",
        "build.gradle.kts",
        "settings.gradle",
        "settings.gradle.kts",
    ]
    .iter()
    .try_fold(false, |found, name| {
        if found {
            Ok(true)
        } else {
            project_manifest_present(root, name)
        }
    })
}
