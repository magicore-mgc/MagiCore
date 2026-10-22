//! Find project roots from supported filesystem anchors.
//! Tìm root dự án từ các dấu mốc filesystem được hỗ trợ.
use super::super::ProjectConfig;
use std::path::{Path, PathBuf};

impl ProjectConfig {
    /// Find the nearest project root from a marker/config or language manifest.
    ///
    /// - `mgc.toml` + `.mgc.core` checked in CWD and ALL parent directories
    ///   (monorepo support — T9a signature leo parent).
    /// - Known language/framework manifests are checked from CWD to the nearest parent.
    pub fn find_project_root(from: &Path) -> Option<PathBuf> {
        if has_project_anchor(from) {
            return Some(from.to_path_buf());
        }

        let mut current = from.parent();
        while let Some(dir) = current {
            if has_project_anchor(dir) {
                return Some(dir.to_path_buf());
            }
            current = dir.parent();
        }

        None
    }
}

/// Detect project anchors without following directory-entry symlinks.
/// Tìm dấu mốc project ở thư mục gần nhất, không đi theo symlink thư mục.
pub(in crate::project::project) fn has_project_anchor(root: &Path) -> bool {
    const DIRECT: &[&str] = &[
        ProjectConfig::CORE_MARKER_FILE,
        "mgc.toml",
        "package.json",
        "Cargo.toml",
        "pyproject.toml",
        "pubspec.yaml",
        "Package.swift",
        "go.mod",
        "pom.xml",
        "build.gradle",
        "build.gradle.kts",
        "project.godot",
        "platformio.ini",
        "west.yml",
        "Pulumi.yaml",
        "Pulumi.yml",
        "cdk.json",
        "wrangler.toml",
        "ProjectSettings/ProjectVersion.txt",
    ];
    DIRECT.iter().any(|name| root.join(name).exists())
        || directory_has_extension_anchor(root, "tf")
        || directory_has_extension_anchor(root, "csproj")
        || directory_has_extension_anchor(root, "uproject")
}

/// Check a filename extension at the project root for root discovery.
/// Kiểm tra đuôi file ở root để tìm project mà không đọc nội dung.
fn directory_has_extension_anchor(root: &Path, extension: &str) -> bool {
    std::fs::read_dir(root).is_ok_and(|entries| {
        entries.flatten().any(|entry| {
            let path = entry.path();
            path.extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value.eq_ignore_ascii_case(extension))
                && entry
                    .file_type()
                    .is_ok_and(|kind| kind.is_file() || kind.is_symlink())
        })
    })
}
