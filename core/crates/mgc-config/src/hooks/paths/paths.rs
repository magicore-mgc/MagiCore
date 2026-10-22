//! Project and user hook configuration paths.
//! Đường dẫn cấu hình hook của dự án và người dùng.

use std::path::{Path, PathBuf};

/// Default hooks paths: project-local first, then the user's global file.
/// Đường dẫn hook mặc định: file dự án trước, sau đó tới file toàn user.
pub fn hooks_paths(project_root: &Path) -> Vec<PathBuf> {
    let mut v = vec![project_root.join("mgc.hooks.toml")];
    if let Ok(home) = std::env::var("HOME") {
        v.push(Path::new(&home).join(".config/magicore/hooks.toml"));
    }
    v
}
