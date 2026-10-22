//! Standard project-local `.magicore/` directories.
//! Các thư mục chuẩn `.magicore/` cục bộ project.

use std::path::{Path, PathBuf};

/// Standard subdirectories under project root (.magicore/)
pub struct ProjectPaths {
    pub root: PathBuf,
    pub store: PathBuf,
    pub cache: PathBuf,
    pub logs: PathBuf,
    pub locks: PathBuf,
    pub dlx: PathBuf,
    pub advisories: PathBuf,
    pub patches: PathBuf,
    pub quarantine: PathBuf,
    pub registry: PathBuf,
    pub lock_signatures: PathBuf, // sidecar .magicore/lock-signatures.json (ref: 16 §4, phản biện v2 #5)
}

impl ProjectPaths {
    /// Compute all standard paths from project root.
    pub fn from_root(root: &Path) -> Self {
        let mgc_dir = root.join(".magicore");
        Self {
            root: root.to_path_buf(),
            store: mgc_dir.join("store"),
            cache: mgc_dir.join("cache"),
            logs: mgc_dir.join("logs"),
            locks: mgc_dir.join("locks"),
            dlx: mgc_dir.join("dlx"),
            advisories: mgc_dir.join("advisories"),
            patches: mgc_dir.join("patches"),
            quarantine: mgc_dir.join("quarantine"),
            registry: mgc_dir.join("registry"),
            lock_signatures: mgc_dir.join("lock-signatures.json"),
        }
    }

    /// Ensure all directories exist.
    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.store)?;
        std::fs::create_dir_all(&self.cache)?;
        std::fs::create_dir_all(&self.logs)?;
        std::fs::create_dir_all(&self.locks)?;
        std::fs::create_dir_all(&self.dlx)?;
        std::fs::create_dir_all(&self.advisories)?;
        std::fs::create_dir_all(&self.patches)?;
        std::fs::create_dir_all(&self.quarantine)?;
        std::fs::create_dir_all(&self.registry)?;
        Ok(())
    }

    /// Path to patches directory (~/.magicore/patches/ or <project>/.magicore/patches/)
    pub fn patches_dir(&self) -> &Path {
        &self.patches
    }
}
