//! Read project references that pin shared Web package roots.
//! Đọc project reference đang ghim package root Web dùng chung.

use crate::commands::cache::filesystem::canonical_or_original;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct WebSharedCacheProjectRef {
    schema_version: u32,
    project_root: String,
    package_roots: Vec<String>,
}

pub(in crate::commands::cache::prune::web_shared) fn read_web_shared_pinned_package_roots(
    root: &Path,
) -> HashSet<PathBuf> {
    let refs_root = root.join("refs").join("projects");
    let mut pinned = HashSet::new();
    let Ok(entries) = std::fs::read_dir(&refs_root) else {
        return pinned;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(reference) = serde_json::from_str::<WebSharedCacheProjectRef>(&contents) else {
            let _ = std::fs::remove_file(&path);
            continue;
        };
        if reference.schema_version != 1 {
            continue;
        }
        let project_root = PathBuf::from(&reference.project_root);
        if !project_root.exists() {
            let _ = std::fs::remove_file(&path);
            continue;
        }
        for package_root in reference.package_roots {
            pinned.insert(canonical_or_original(Path::new(&package_root)));
        }
    }
    pinned
}

pub(in crate::commands::cache::prune::web_shared) fn count_web_shared_live_project_refs(
    root: &Path,
) -> usize {
    let refs_root = root.join("refs").join("projects");
    let Ok(entries) = std::fs::read_dir(refs_root) else {
        return 0;
    };
    entries
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                return false;
            }
            let Ok(contents) = std::fs::read_to_string(path) else {
                return false;
            };
            let Ok(reference) = serde_json::from_str::<WebSharedCacheProjectRef>(&contents) else {
                return false;
            };
            reference.schema_version == 1 && PathBuf::from(reference.project_root).exists()
        })
        .count()
}
