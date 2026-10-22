//! Native Python install
//! Install Python native

use mgc_resolver::protocols::{PypiProtocol, RegistryProtocol};
use mgc_types::adapter::{InstallCacheMode, InstallSummary};
use mgc_types::{MgResult, ResolvedGraph};
use std::path::Path;
use std::time::Instant;

use super::super::{content_store, entry_from_package, markers_for, shared_store};
use super::error::PythonNativeInstallError;

/// Native python install: download each wheel/sdist → verify sha256 → import
/// to the mgc CAS → materialize into `{store}/pypi/wheels/`. No `uv sync`/
/// `pip install` spawn — mgc owns resolve/fetch/install (Phase 2).
/// Install python native: tải từng wheel/sdist → verify sha256 → import vào
/// CAS mgc → materialize vào `{store}/pypi/wheels/`. Không spawn `uv sync`/
/// `pip install` — mgc giữ resolve/fetch/install (Phase 2).
pub(crate) async fn install_python_native(
    graph: &ResolvedGraph,
    lock_packages: &[mgc_lockfile::Package],
    offline: bool,
) -> Result<InstallSummary, PythonNativeInstallError> {
    let started = Instant::now();
    // Do not report success for artifacts this runtime cannot import. Check
    // the whole graph before network or store mutation so a compiled wheel
    // or sdist fails honestly instead of yielding a broken Python project.
    // (Kiểm tra toàn graph trước download/store để không báo cài thành công
    // cho wheel biên dịch hoặc sdist mà runtime hiện tại không import được.)
    validate_python_importable_graph(graph)?;
    let protocol = PypiProtocol::from_env();
    let store_root = if offline {
        shared_store::SharedStoreRun::pypi_path()?.install_root
    } else {
        shared_store::SharedStoreRun::pypi()?.install_root
    };
    let wheels_dir = store_root.join("wheels");

    // Offline replay is all-or-nothing with respect to cache availability:
    // authenticate every pinned artifact before creating the CAS or
    // materializing any package. This avoids turning a late cache miss into
    // a misleading partial successful install.
    // (Offline phải preflight toàn bộ cache trước khi tạo CAS/materialize.)
    if offline {
        for pkg in &graph.packages {
            let mut entry = entry_from_package(pkg);
            entry.extra_markers = markers_for(pkg, lock_packages);
            let relative =
                PypiProtocol::artifact_cache_relpath(&entry.artifact_url, &entry.sha256)?;
            let artifact_path = wheels_dir.join(relative);
            if read_cached_python_artifact(&artifact_path, &entry.sha256)?.is_none() {
                return Err(PythonNativeInstallError::OfflineCacheMiss {
                    name: entry.name,
                    version: entry.version,
                });
            }
        }
    }

    let store = content_store()?;
    let mut added = Vec::with_capacity(graph.packages.len());
    let mut bytes_from_cache = 0_u64;

    for pkg in &graph.packages {
        let mut entry = entry_from_package(pkg);
        // Reattach lock markers (wheel tags for the ABI warning, sdist
        // flags) — the graph carries integrity only.
        // (Gắn lại marker từ lock.)
        entry.extra_markers = markers_for(pkg, lock_packages);
        let artifact_relpath =
            PypiProtocol::artifact_cache_relpath(&entry.artifact_url, &entry.sha256)?;
        let artifact_path = wheels_dir.join(artifact_relpath);
        let (bytes, cache_hit) = match read_cached_python_artifact(&artifact_path, &entry.sha256)? {
            Some(bytes) => (bytes, true),
            None => {
                if offline {
                    return Err(PythonNativeInstallError::OfflineCacheDisappeared {
                        name: entry.name,
                        version: entry.version,
                    });
                }
                let bytes = protocol.download(&entry).await?;
                // The materializer verifies the registry digest before
                // publishing a newly fetched artifact.
                // (Materializer xác minh digest trước khi lưu artifact mới.)
                protocol.materialize(&entry, &bytes, &wheels_dir)?;
                (bytes, false)
            }
        };
        if cache_hit {
            bytes_from_cache = bytes_from_cache.saturating_add(bytes.len() as u64);
        }
        // Pure-python wheels additionally unpack into an importable site
        // dir (compiled wheels honestly skip — the ABI warning above).
        // Preflight above guarantees this is importable. Keep a runtime
        // assertion at the materializer boundary in case resolver output
        // changes between validation and installation.
        // (Preflight bảo đảm import được; kiểm lại tại materializer.)
        if protocol
            .materialize_importable(&entry, &bytes, &wheels_dir)?
            .is_none()
        {
            return Err(PythonNativeInstallError::ArtifactBecameNonImportable {
                name: entry.name,
                version: entry.version,
            });
        }
        store.import_bytes(&bytes)?;
        added.push(pkg.id.clone());
    }

    Ok(InstallSummary {
        added,
        bytes_from_cache,
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: InstallCacheMode::MgCStore,
    })
}

/// Read a digest-addressed wheel cache entry without following a final
/// symlink, then re-verify its SHA-256 before allowing it to satisfy an
/// install. A missing entry is a cache miss; malformed or modified entries
/// fail closed rather than falling through to unverified bytes.
/// Đọc cache wheel theo digest, không theo symlink và xác minh SHA-256 lại.
// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: install_python_native (offline preflight + cached replay; body
//   stays in lib), lib install tests.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(crate) fn read_cached_python_artifact(
    path: &Path,
    expected_sha256: &str,
) -> MgResult<Option<Vec<u8>>> {
    mgc_python_service::install::read_cached_python_artifact(path, expected_sha256)
}

// COMPAT facade (S09a/P03) — delegates to mgc_python_service::install.
// Facade tương thích (S09a/P03) — ủy quyền cho mgc_python_service.
// Callers: install_python_native (importability preflight; body stays in
//   lib), lib install tests.
// TODO(S09b-cleanup): remove once ai retargets onto mgc-python-service directly.
pub(crate) fn validate_python_importable_graph(graph: &ResolvedGraph) -> MgResult<()> {
    mgc_python_service::install::validate_python_importable_graph(graph)
}
