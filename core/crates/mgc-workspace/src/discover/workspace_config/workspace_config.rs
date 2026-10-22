//! Workspace configuration reading and layout parsing.
//! Đọc cấu hình và phân tích layout workspace.

use super::DiscoverOptions;
use std::path::Path;

/// Default application layout directory.
/// Thư mục mặc định cho layout ứng dụng.
const DEFAULT_APPS_DIR: &str = "apps";

/// Default package layout directory.
/// Thư mục mặc định cho layout package.
const DEFAULT_PACKAGES_DIR: &str = "packages";

/// Read workspace config only when it is a regular, non-symlink file.
/// Chỉ đọc cấu hình khi đó là file thường, không phải symlink.
pub fn read_workspace_config(project_root: &Path) -> anyhow::Result<Option<String>> {
    let path = project_root.join("magicore.workspace.toml");
    let metadata = match std::fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    if metadata.file_type().is_symlink() {
        anyhow::bail!(
            "workspace config '{}' must not be a symlink",
            path.display()
        );
    }
    if !metadata.is_file() {
        anyhow::bail!(
            "workspace config '{}' must be a regular file",
            path.display()
        );
    }
    let contents =
        mgc_adapter_base::project_file::read_regular_text(&path, "magicore.workspace.toml")?;
    Ok(Some(contents))
}

/// Parse the workspace layout; absent configuration uses apps/ and packages/.
/// Phân tích layout workspace; thiếu cấu hình thì dùng apps/ và packages/.
pub fn workspace_layout(project_root: &Path) -> anyhow::Result<DiscoverOptions> {
    let Some(contents) = read_workspace_config(project_root)? else {
        return Ok(DiscoverOptions {
            apps_dir: DEFAULT_APPS_DIR.to_string(),
            packages_dir: DEFAULT_PACKAGES_DIR.to_string(),
        });
    };
    #[derive(serde::Deserialize)]
    struct Config {
        #[serde(default)]
        layout: Option<Layout>,
    }
    #[derive(serde::Deserialize)]
    struct Layout {
        #[serde(default)]
        apps_dir: Option<String>,
        #[serde(default)]
        packages_dir: Option<String>,
    }
    let config: Config = toml::from_str(&contents)?;
    Ok(DiscoverOptions {
        apps_dir: config
            .layout
            .as_ref()
            .and_then(|l| l.apps_dir.clone())
            .unwrap_or_else(|| DEFAULT_APPS_DIR.to_string()),
        packages_dir: config
            .layout
            .as_ref()
            .and_then(|l| l.packages_dir.clone())
            .unwrap_or_else(|| DEFAULT_PACKAGES_DIR.to_string()),
    })
}
