//! Store project references and read package roots pinned against pruning.
//! Lưu tham chiếu project và đọc package root được ghim khỏi prune.

use crate::cache::current_unix_secs;
use crate::cache::{SharedCacheProjectRef, SharedWebCache};
use crate::cache_prune::path_to_cache_ref_string;
use crate::manifest::atomic_write;
use mgc_types::MgResult;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

impl SharedWebCache {
    pub fn project_ref_path(&self, project_root: &Path) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(path_to_cache_ref_string(project_root).as_bytes());
        let key = hex::encode(hasher.finalize());
        self.root
            .join("refs")
            .join("projects")
            .join(format!("{key}.json"))
    }

    pub fn write_project_ref(
        &self,
        project_root: &Path,
        package_roots: impl IntoIterator<Item = PathBuf>,
    ) -> MgResult<()> {
        let path = self.project_ref_path(project_root);
        let mut roots = package_roots
            .into_iter()
            .map(|path| path_to_cache_ref_string(&path))
            .collect::<Vec<_>>();
        roots.sort();
        roots.dedup();
        let payload = serde_json::to_vec_pretty(&SharedCacheProjectRef {
            schema_version: 1,
            project_root: path_to_cache_ref_string(project_root),
            updated_at: current_unix_secs(),
            package_roots: roots,
        })?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        atomic_write(&path, &payload)
    }
}

pub fn read_shared_cache_pinned_package_roots(root: &Path) -> std::collections::HashSet<PathBuf> {
    let refs_root = root.join("refs").join("projects");
    let mut pinned = std::collections::HashSet::new();
    if !refs_root.exists() {
        return pinned;
    }

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
        let Ok(reference) = serde_json::from_str::<SharedCacheProjectRef>(&contents) else {
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
            pinned.insert(PathBuf::from(package_root));
        }
    }
    pinned
}
