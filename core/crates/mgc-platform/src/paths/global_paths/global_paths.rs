//! User-global MagiCore data paths (MGC_DATA_DIR or ~/MagiCore-SystemData).
//! Đường dẫn dữ liệu MagiCore toàn user (MGC_DATA_DIR hoặc ~/MagiCore-SystemData).

use std::path::{Path, PathBuf};

use crate::paths::mgc_data_dir;

/// User-global MagiCore data paths (MGC_DATA_DIR or ~/MagiCore-SystemData).
pub struct GlobalPaths {
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
    pub lock_signatures: PathBuf,
    /// Core-identity attestations by project (`identities/<path-hash>/attestation.json`).
    /// Chứng thực identity core theo project.
    pub identities: PathBuf,
}

impl GlobalPaths {
    /// Resolve user-global paths; `MAGICORE_STORE_ROOT` overrides only `store`.
    /// Resolve global paths from the visible data root; the legacy store override remains supported.
    /// Dùng root dữ liệu tập trung và vẫn hỗ trợ override store cũ.
    pub fn new() -> std::io::Result<Self> {
        let mgc_dir = mgc_data_dir();
        // Keep shared ecosystem stores aligned with the CAS override used by
        // mgc-store, including isolated Windows test and CI sandboxes.
        // Đồng bộ store ecosystem với override CAS của mgc-store, kể cả
        // sandbox test/CI Windows được cô lập.
        let store = std::env::var_os("MAGICORE_STORE_ROOT")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| mgc_dir.join("store"));
        Ok(Self {
            root: mgc_dir.clone(),
            store,
            cache: mgc_dir.join("cache"),
            logs: mgc_dir.join("logs"),
            locks: mgc_dir.join("locks"),
            dlx: mgc_dir.join("dlx"),
            advisories: mgc_dir.join("advisories"),
            patches: mgc_dir.join("patches"),
            quarantine: mgc_dir.join("quarantine"),
            registry: mgc_dir.join("registry"),
            lock_signatures: mgc_dir.join("lock-signatures.json"),
            identities: mgc_dir.join("identities"),
        })
    }

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

    pub fn patches_dir(&self) -> &Path {
        &self.patches
    }
}
