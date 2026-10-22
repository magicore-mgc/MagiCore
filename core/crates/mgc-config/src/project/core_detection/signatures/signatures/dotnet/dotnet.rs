//! dotnet manifest recognition.
//! Nhận diện manifest dotnet.
use super::super::shared::files_with_extension;
use crate::project::read_regular_project_text;
use std::path::Path;
pub(in crate::project::project::core_detection) fn detect_dotnet_cores(
    project_root: &Path,
) -> anyhow::Result<Vec<String>> {
    let mut cores = Vec::new();
    for manifest in files_with_extension(project_root, "csproj")? {
        let content = read_regular_project_text(&manifest, ".NET project manifest")?
            .ok_or_else(|| anyhow::anyhow!(".NET manifest disappeared during core detection"))?;
        let lower = content.to_ascii_lowercase();
        if lower.contains("microsoft.net.sdk.maui") || lower.contains("<usemaui>true</usemaui>") {
            cores.push("app".to_string());
        } else if lower.contains("microsoft.net.sdk.web") {
            cores.push("web".to_string());
        } else {
            cores.push("lib".to_string());
        }
    }
    Ok(cores)
}
