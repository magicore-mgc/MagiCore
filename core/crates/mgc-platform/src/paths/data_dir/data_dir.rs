//! Data-root resolution: MGC_DATA_DIR override and managed tool directories.
//! Resolve data-root: override MGC_DATA_DIR và các thư mục tool do MGC quản lý.

use std::path::{Path, PathBuf};

pub const MGC_DATA_DIR_ENV: &str = "MGC_DATA_DIR";

/// Resolve the visible root for MagiCore-managed stores and caches.
/// Tìm root dễ thấy cho store/cache do MagiCore quản lý.
/// `MGC_DATA_DIR` overrides the default `~/MagiCore-SystemData` location.
/// `MGC_DATA_DIR` ghi đè vị trí mặc định `~/MagiCore-SystemData`.
pub fn mgc_data_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    if let Some(raw) = std::env::var_os(MGC_DATA_DIR_ENV).filter(|value| !value.is_empty()) {
        let path = PathBuf::from(raw);
        return if path.is_absolute() {
            path
        } else {
            home.join(path)
        };
    }
    home.join("MagiCore-SystemData")
}

/// Persistent roots for user-installed command-line tools and their shims.
/// Root bền vững cho CLI tool của user và shim nằm trong PATH.
pub fn global_tools_dir() -> PathBuf {
    mgc_data_dir().join("tools")
}

pub fn global_bin_dir() -> PathBuf {
    mgc_data_dir().join("bin")
}

/// Resolve stable per-project data outside the project's source tree.
/// Tách dữ liệu từng project khỏi cây mã nguồn.
pub fn project_data_dir(project_root: &Path) -> PathBuf {
    let canonical = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let identity = canonical.to_string_lossy();
    let digest = blake3::hash(identity.as_bytes()).to_hex();
    let name = canonical
        .file_name()
        .and_then(|value| value.to_str())
        .map(safe_data_component)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "project".to_string());
    mgc_data_dir()
        .join("projects")
        .join(format!("{name}-{}", &digest[..16]))
}

fn safe_data_component(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .take(48)
        .collect()
}
