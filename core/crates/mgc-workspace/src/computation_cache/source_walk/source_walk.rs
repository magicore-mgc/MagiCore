// Safe source-tree traversal — duyệt cây source an toàn

use super::super::{file_hash::hash_file, path_key::source_path_key};
use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

pub(in crate::computation_cache) fn walk_and_collect_hashes(
    root: &Path,
    current: &Path,
    out: &mut BTreeMap<String, String>,
    visited_directories: &mut HashSet<std::path::PathBuf>,
) -> Result<()> {
    // Keep traversal iterative and reject duplicate physical directories to avoid cycles/aliases.
    // Duyệt lặp và từ chối thư mục vật lý trùng nhau để tránh vòng lặp hoặc alias.
    let mut pending = vec![current.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let metadata = fs::symlink_metadata(&directory).with_context(|| {
            format!("cannot inspect source directory '{}'", directory.display())
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            bail!(
                "source directory '{}' must be a real directory",
                directory.display()
            );
        }
        let canonical = fs::canonicalize(&directory).with_context(|| {
            format!("cannot resolve source directory '{}'", directory.display())
        })?;
        if !visited_directories.insert(canonical) {
            bail!(
                "source directory '{}' was reached more than once",
                directory.display()
            );
        }

        let entries = fs::read_dir(&directory)
            .with_context(|| format!("cannot read source directory '{}'", directory.display()))?;
        for entry in entries {
            let entry = entry.with_context(|| {
                format!(
                    "cannot read an entry in source directory '{}'",
                    directory.display()
                )
            })?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            let file_type = entry
                .file_type()
                .with_context(|| format!("cannot inspect source entry '{}'", path.display()))?;

            if file_type.is_symlink() {
                bail!("source entry '{}' must not be a symlink", path.display());
            }

            if file_type.is_dir() {
                // Generated folders are intentionally excluded from package source hashes.
                // Cố ý loại các thư mục sinh tự động khỏi hash mã nguồn package.
                if matches!(
                    name_str.as_ref(),
                    "node_modules"
                        | "target"
                        | "dist"
                        | "build"
                        | ".git"
                        | ".magicore"
                        | ".cache"
                        | ".turbo"
                        | ".next"
                        | "coverage"
                        | ".venv"
                ) {
                    continue;
                }
                pending.push(path);
            } else if file_type.is_file() {
                // Ignore known volatile files; all other read/hash errors must propagate.
                // Bỏ qua file biến động đã biết; mọi lỗi đọc/hash khác phải được trả về.
                if name_str.ends_with(".log")
                    || name_str.ends_with(".tmp")
                    || name_str.starts_with(".DS_Store")
                {
                    continue;
                }

                let rel = path.strip_prefix(root).with_context(|| {
                    format!("source entry '{}' escaped package root", path.display())
                })?;
                let rel_str = source_path_key(rel);
                let content_hash = hash_file(&path)
                    .with_context(|| format!("cannot hash source file '{}'", path.display()))?;
                if out.insert(rel_str, content_hash).is_some() {
                    bail!(
                        "source path key collision while hashing '{}'",
                        path.display()
                    );
                }
            } else {
                bail!(
                    "source entry '{}' is not a regular file or directory",
                    path.display()
                );
            }
        }
    }
    Ok(())
}
