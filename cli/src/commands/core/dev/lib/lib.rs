//! `mgc dev` for the Lib core: rebuild-on-change, not a server.
//! `mgc dev` cho core Lib: rebuild khi thay đổi, không phải server.
//!
//! A library has no process to serve. The honest dev loop is therefore
//! "watch the sources, rebuild", which is exactly what `dev_port.rs` already
//! documents for this core (`1354 — build-watch`). Starting an HTTP server to
//! satisfy a port table would be worse than having no `dev` at all.
//! (Thư viện không có process để serve. Vòng dev trung thực là "watch source,
//! rebuild" — đúng như `dev_port.rs` ghi cho core này. Bật HTTP server chỉ để
//! khớp bảng port sẽ tệ hơn là không có `dev`.)

use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use mgc_ui::{info, success, warning};

use crate::commands::core::shared;

/// Extensions whose change triggers a rebuild for every lib language.
///
/// Grouped per language folder so a language's watch rules live beside that
/// language's build rules instead of in one shared list.
/// (Nhóm theo ngôn ngữ để rule watch của ngôn ngữ nằm cạnh rule build của nó.)
const RUST_SOURCES: &[&str] = &["rs", "toml"];
const PYTHON_SOURCES: &[&str] = &["py", "pyi", "toml", "cfg"];
const TYPESCRIPT_SOURCES: &[&str] = &["ts", "tsx", "mts", "cts", "json"];
const GO_SOURCES: &[&str] = &["go", "mod", "sum"];
const JAVA_SOURCES: &[&str] = &["java", "kt", "gradle", "xml", "properties"];
const DOTNET_SOURCES: &[&str] = &["cs", "csproj", "sln", "props", "targets"];

/// Directories that never participate in a library rebuild.
/// Thư mục không bao giờ tham gia rebuild thư viện.
const IGNORED_DIRECTORIES: &[&str] = &[
    ".git",
    ".magicore",
    "node_modules",
    "target",
    "dist",
    "build",
    "bin",
    "obj",
    "__pycache__",
    ".venv",
    "venv",
];

/// Minimum gap between two rebuilds triggered by a burst of writes.
/// Khoảng nghỉ tối thiểu giữa hai rebuild khi một loạt ghi diễn ra.
const REBUILD_DEBOUNCE: Duration = Duration::from_millis(400);

/// Source extensions that matter for the project's detected language.
///
/// Returns an empty slice for a language the dev loop cannot watch, so the
/// caller fails closed instead of pretending to build.
/// (Trả rỗng cho ngôn ngữ không watch được để caller fail-closed thay vì giả vờ
/// build.)
pub(in crate::commands::core::dev) fn watch_extensions(root: &Path) -> &'static [&'static str] {
    let has = |name: &str| root.join(name).is_file();
    if has("Cargo.toml") {
        RUST_SOURCES
    } else if has("pyproject.toml") || has("setup.py") {
        PYTHON_SOURCES
    } else if has("go.mod") {
        GO_SOURCES
    } else if has("pom.xml") || has("build.gradle") || has("build.gradle.kts") {
        JAVA_SOURCES
    } else if root
        .read_dir()
        .map(|entries| {
            entries.flatten().any(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("csproj"))
            })
        })
        .unwrap_or(false)
    {
        DOTNET_SOURCES
    } else if has("tsconfig.json") || has("package.json") {
        TYPESCRIPT_SOURCES
    } else {
        &[]
    }
}

/// Count source files the watch rules cover, so `--once` can prove it looked.
/// Đếm file nguồn khớp rule watch để `--once` chứng minh đã quét.
pub(in crate::commands::core::dev) fn count_watched_sources(
    root: &Path,
    extensions: &[&str],
) -> Result<usize> {
    let mut count = 0usize;
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if file_type.is_dir() {
                if !IGNORED_DIRECTORIES.iter().any(|skip| *skip == name) {
                    stack.push(path);
                }
                continue;
            }
            if !file_type.is_file() {
                continue;
            }
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    extensions
                        .iter()
                        .any(|wanted| wanted.eq_ignore_ascii_case(ext))
                })
            {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// Run the library dev loop.
///
/// `once` performs a single rebuild, which is what CI and tests use: watching
/// forever cannot be asserted and would hang a test run.
/// (`once` build một lần — CI/test dùng vì watch vô hạn không assert được và sẽ
/// treo lượt chạy test.)
pub async fn dev(once: bool) -> Result<()> {
    let root = shared::core_project_root("lib")?;
    dev_at(&root, once).await
}

/// Dev loop at an explicit root; tests call this to assert fail-closed rules.
/// Vòng dev tại root tường minh; test gọi để kiểm rule fail-closed.
pub(in crate::commands::core::dev) async fn dev_at(root: &Path, once: bool) -> Result<()> {
    let extensions = watch_extensions(root);
    if extensions.is_empty() {
        return Err(super::error::language_unknown(root));
    }
    let watched = count_watched_sources(root, extensions)?;
    if watched == 0 {
        return Err(super::error::no_sources(root, extensions));
    }

    info(&format!(
        "Lib dev (build-watch): {} source file(s) under {}",
        watched,
        root.display()
    ));
    crate::commands::build::build_lib_for_dev(root).await?;
    success("Lib dev: rebuild complete");

    if once {
        return Ok(());
    }

    // Watching is opt-in per iteration: a rebuild failure is reported and the
    // loop continues so a developer sees every error, not just the first.
    // Watch lặp lại: lỗi rebuild được báo rồi tiếp tục để dev thấy mọi lỗi.
    let fingerprint = source_fingerprint(root, extensions)?;
    let mut last = fingerprint;
    loop {
        tokio::time::sleep(REBUILD_DEBOUNCE).await;
        let current = source_fingerprint(root, extensions)?;
        if current == last {
            continue;
        }
        last = current;
        match crate::commands::build::build_lib_for_dev(root).await {
            Ok(()) => success("Lib dev: rebuild complete"),
            Err(error) => warning(&format!("Lib dev: rebuild failed — {error}")),
        }
    }
}

/// Cheap change detector: newest modification time plus the file count.
///
/// A stat-only fingerprint cannot see an edit that preserves mtime, which is
/// why the count is part of the value: adding or deleting a source always
/// changes it.
/// (Fingerprint chỉ stat không thấy sửa giữ nguyên mtime, nên kèm số file: thêm
/// hoặc xoá source luôn đổi giá trị.)
fn source_fingerprint(root: &Path, extensions: &[&str]) -> Result<(usize, u128)> {
    let mut count = 0usize;
    let mut newest = 0u128;
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if file_type.is_dir() {
                if !IGNORED_DIRECTORIES.iter().any(|skip| *skip == name) {
                    stack.push(path);
                }
                continue;
            }
            let matches = path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    extensions
                        .iter()
                        .any(|wanted| wanted.eq_ignore_ascii_case(ext))
                });
            if !matches {
                continue;
            }
            count += 1;
            if let Ok(metadata) = entry.metadata()
                && let Ok(modified) = metadata.modified()
                && let Ok(since) = modified.duration_since(std::time::UNIX_EPOCH)
            {
                newest = newest.max(since.as_nanos());
            }
        }
    }
    Ok((count, newest))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
