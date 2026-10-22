//! File-selection coordinator — điều phối chọn file package.
use super::include_rules::should_include;
use super::walk_config::configure;
use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Select publishable files under the project root.
/// Chọn file được phép đóng gói bên dưới thư mục gốc dự án.
pub fn select_files(root: &Path) -> Result<Vec<PathBuf>> {
    let npmignore = root.join(".npmignore");
    let has_npmignore = npmignore.exists();

    let mut builder = ::ignore::WalkBuilder::new(root);
    builder.hidden(false); // Include hidden files unless a rule excludes them — nhận file ẩn trừ khi có luật loại trừ.
    builder.require_git(false); // Read ignore files without a .git directory — đọc luật ignore dù không có thư mục .git.
    builder.parents(false); // Keep ignore rules scoped to this project — chỉ áp dụng luật trong dự án này.
    configure(&mut builder, has_npmignore);

    let mut files: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();

    for entry in builder.build() {
        let entry = entry?;
        if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
            continue;
        }
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(path);
        if should_include(rel) && seen.insert(rel.to_path_buf()) {
            files.push(rel.to_path_buf());
        }
    }

    // Keep required publish files even when ignore rules match them — giữ lại file publish bắt buộc dù trùng luật ignore.
    for name in super::include_rules::ALWAYS_INCLUDE {
        let rel = PathBuf::from(name);
        if root.join(&rel).is_file() && seen.insert(rel.clone()) {
            files.push(rel);
        }
    }

    files.sort();
    Ok(files)
}
