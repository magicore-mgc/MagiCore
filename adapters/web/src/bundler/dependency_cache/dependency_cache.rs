//! Session cache and dependency-bundling coordinator.
//! Cache theo session và điều phối bundle dependency.

use std::{collections::HashMap, path::PathBuf, sync::Arc};

use tokio::sync::RwLock;
use tracing::{debug, warn};

use super::{
    build::bundle_dependency,
    name_validation::dep_name_is_valid,
    package_entry::{allowed_dependency_roots, resolve_package_entry},
    types::CachedDep,
};

/// Bundled dependency cache scoped to one development-server instance.
/// Cache dependency đã bundle, giới hạn trong một instance dev server.
#[derive(Clone, Default)]
pub struct DepsCache {
    /// Map: package name to bundled JavaScript and optional CSS.
    /// Map: tên package tới JavaScript đã bundle và CSS tùy chọn.
    inner: Arc<RwLock<HashMap<String, CachedDep>>>,
    pub node_modules: PathBuf,
}

impl DepsCache {
    /// Create a cache rooted at the project's `node_modules` directory.
    /// Tạo cache trỏ tới thư mục `node_modules` của project.
    pub fn new(node_modules: PathBuf) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            node_modules,
        }
    }

    /// Return cached output or bundle a package on first use.
    /// Trả kết quả cache hoặc bundle package ở lần dùng đầu.
    pub async fn get_or_bundle(&self, pkg_name: &str) -> Option<CachedDep> {
        if !dep_name_is_valid(pkg_name) {
            return None;
        }

        {
            let cache = self.inner.read().await;
            if let Some(dep) = cache.get(pkg_name) {
                debug!("deps-cache hit: {}", pkg_name);
                return Some(dep.clone());
            }
        }

        let entry = match resolve_package_entry(&self.node_modules, pkg_name) {
            Ok(Some(path)) => path,
            Ok(None) => return None,
            Err(error) => {
                warn!("rejected dependency entry '{}': {}", pkg_name, error);
                return None;
            }
        };
        debug!("bundling dep: {} → {}", pkg_name, entry.display());

        let working_dir = self
            .node_modules
            .parent()
            .unwrap_or(&self.node_modules)
            .to_string_lossy()
            .to_string();

        let allowed_roots = match allowed_dependency_roots(&self.node_modules) {
            Ok(roots) => roots,
            Err(error) => {
                warn!("rejected dependency roots '{}': {}", pkg_name, error);
                return None;
            }
        };

        let dep = match bundle_dependency(&entry, &working_dir, &allowed_roots).await {
            Ok(dep) => dep,
            Err(error) => {
                warn!("failed to pre-bundle dep '{}': {}", pkg_name, error);
                return None;
            }
        };

        {
            let mut cache = self.inner.write().await;
            cache.insert(pkg_name.to_string(), dep.clone());
        }

        Some(dep)
    }
}

#[cfg(test)]
#[path = "test/dependency_cache.rs"]
mod tests;
