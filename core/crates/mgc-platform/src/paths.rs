//! mgc-platform/paths.rs — Standard paths (RULE §12: no hardcoded paths)
//! (Đường dẫn chuẩn: store/cache/logs/locks/dlx/advisories/patches/quarantine/registry + .magicore/lock-signatures.json)

use std::path::{Path, PathBuf};

pub const MGC_DATA_DIR_ENV: &str = "MGC_DATA_DIR";

/// Resolve the visible root for MagiCore-managed stores and caches.
/// Tìm root dễ thấy cho store/cache do MagiCore quản lý.
/// `MGC_DATA_DIR` overrides the default `~/MagiCore-SystemData` location.
/// `MGC_DATA_DIR` ghi đè vị trí mặc định `~/MagiCore-SystemData`.
pub fn mgc_data_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    if let Some(raw) = std::env::var_os(MGC_DATA_DIR_ENV).filter(|value| !value.is_empty()) {
        let path = PathBuf::from(raw);
        return if path.is_absolute() {
            path
        } else {
            home.join(path)
        };
    }
    home.join("MagiCore-SystemData")
}

/// Resolve stable per-project data outside the project's source tree.
/// Tách dữ liệu từng project khỏi cây mã nguồn.
pub fn project_data_dir(project_root: &Path) -> PathBuf {
    let canonical = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let identity = canonical.to_string_lossy();
    let digest = blake3::hash(identity.as_bytes()).to_hex();
    let name = canonical
        .file_name()
        .and_then(|value| value.to_str())
        .map(safe_data_component)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "project".to_string());
    mgc_data_dir()
        .join("projects")
        .join(format!("{name}-{}", &digest[..16]))
}

fn safe_data_component(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .take(48)
        .collect()
}

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

/// Detect project root (walk up for markers) — shared with mgc-config/paths.rs
pub fn find_project_root(from: &Path) -> Option<PathBuf> {
    let mut current = from.to_path_buf();
    loop {
        if current.join("mgc.toml").exists()
            || current.join("mgc.lock").exists()
            || current.join("package.json").exists()
            || current.join("Cargo.toml").exists()
            || current.join("pyproject.toml").exists()
            || current.join(".git").exists()
        {
            return Some(current);
        }
        if !current.pop() {
            break;
        }
    }
    None
}
