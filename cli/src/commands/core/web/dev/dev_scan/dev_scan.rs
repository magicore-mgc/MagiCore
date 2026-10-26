//! Scan source files as the dev server serves them.
//! Scan file source khi dev server phục vụ chúng.
//!
//! The build scans `dist/`, but a dev server never produces `dist/`: it
//! transforms sources on request. A source file that would fail the build scan
//! should be visible while it is still being edited, not an hour later at
//! release.
//! Bản build scan `dist/`, nhưng dev server không bao giờ tạo `dist/`: nó biến
//! đổi source theo yêu cầu. File source sẽ fail ở scan build nên hiện ra trong
//! lúc còn đang sửa, không phải một giờ sau lúc phát hành.

use std::path::Path;

use anyhow::Result;

/// Outcome of scanning the project sources.
/// Kết quả scan source của project.
#[derive(Debug, Default)]
pub(crate) struct DevScan {
    /// Files that would block a build.
    /// File sẽ chặn build.
    pub blocking: Vec<String>,
}

impl DevScan {
    /// One line per blocking file, ready for the dev banner.
    /// Một dòng mỗi file chặn, sẵn cho banner dev.
    pub(crate) fn summary(&self) -> Option<String> {
        if self.blocking.is_empty() {
            return None;
        }
        Some(format!(
            "dev scan: {} source file(s) would fail a build: {}",
            self.blocking.len(),
            self.blocking.join(", ")
        ))
    }
}

/// Source extensions the dev server transforms.
/// Đuôi source mà dev server biến đổi.
const SOURCE_EXTENSIONS: [&str; 7] = ["ts", "tsx", "js", "jsx", "mjs", "cjs", "vue"];

/// Scan the project sources under `root`.
///
/// Only a blocking rule is reported: a development build note makes no sense
/// for source that has not been built.
/// Scan source project dưới `root`. Chỉ báo luật chặn: ghi chú "bản dev" không
/// có nghĩa với source chưa build.
pub(crate) fn scan_sources(root: &Path) -> Result<DevScan> {
    let mut result = DevScan::default();
    if !root.is_dir() {
        return Ok(result);
    }
    for entry in walk(root)? {
        let relative = entry
            .strip_prefix(root)
            .unwrap_or(&entry)
            .to_string_lossy()
            .replace('\\', "/");
        // Dependencies are not the user's source and their review belongs to
        // install, not to a dev banner.
        // Dependency không phải source của user; việc review chúng thuộc install,
        // không thuộc banner dev.
        if relative.starts_with("node_modules/") || relative.starts_with("dist/") {
            continue;
        }
        if !SOURCE_EXTENSIONS.iter().any(|ext| {
            entry
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|actual| actual.eq_ignore_ascii_case(ext))
        }) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&entry) else {
            continue;
        };
        if has_dynamic_eval(&text) {
            result.blocking.push(relative);
        }
    }
    Ok(result)
}

fn walk(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        if name == "node_modules" || name == "dist" || name.starts_with('.') {
            continue;
        }
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            files.extend(walk(&path)?);
        } else if metadata.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}

/// Whether the text calls `eval` with something that is not a literal.
/// Text có gọi `eval` với đối số không phải literal không.
fn has_dynamic_eval(text: &str) -> bool {
    let mut rest = text;
    while let Some(index) = rest.find("eval(") {
        let after = &rest[index + 5..];
        let literal = after.starts_with('"')
            || after.starts_with('\'')
            || after.starts_with('`')
            || after.starts_with('[')
            || after.starts_with('{');
        if !literal {
            return true;
        }
        rest = after;
    }
    false
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
