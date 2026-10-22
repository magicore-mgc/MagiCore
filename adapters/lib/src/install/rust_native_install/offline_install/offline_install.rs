//! Preflights and replays verified Cargo archive cache entries offline.
//! Preflight và replay archive Cargo đã xác minh ở chế độ offline.

use mgc_resolver::protocols::CratesProtocol;
use mgc_resolver::protocols::RegistryProtocol;
use mgc_store::ContentStore;
use mgc_types::ResolvedGraph;
use mgc_types::adapter::{InstallCacheMode, InstallSummary};
use std::path::Path;
use std::time::Instant;

use super::super::super::{cargo_compat, entry_from_package};
use super::super::artifact_cache::{cargo_archive_path, read_cached_rust_artifact};
use super::super::error::RustNativeInstallError;

/// Validate the full locked archive set without writing or opening CAS.
/// Xác minh toàn bộ archive theo lock trước khi ghi hoặc mở CAS.
pub(crate) fn preflight_rust_artifact_cache(
    graph: &ResolvedGraph,
    cargo_home: &Path,
) -> Result<(), RustNativeInstallError> {
    for pkg in &graph.packages {
        let entry = entry_from_package(pkg);
        let path = cargo_archive_path(cargo_home, &entry);
        if read_cached_rust_artifact(&path, &entry.sha256)?.is_none() {
            return Err(RustNativeInstallError::OfflineCacheMiss {
                name: entry.name,
                version: entry.version,
            });
        }
    }
    Ok(())
}

/// Replay verified offline archives into CAS and Cargo-compatible layout.
/// Nạp archive offline đã xác minh vào CAS và layout tương thích Cargo.
pub(crate) fn install_rust_cached(
    graph: &ResolvedGraph,
    cargo_home: &Path,
    store: &ContentStore,
    started: Instant,
) -> Result<InstallSummary, RustNativeInstallError> {
    let protocol = CratesProtocol::from_env();
    let mut added = Vec::with_capacity(graph.packages.len());
    let mut bytes_from_cache = 0_u64;

    for pkg in &graph.packages {
        let entry = entry_from_package(pkg);
        let path = cargo_archive_path(cargo_home, &entry);
        let bytes = read_cached_rust_artifact(&path, &entry.sha256)?.ok_or_else(|| {
            RustNativeInstallError::OfflineCacheDisappeared {
                name: entry.name.clone(),
                version: entry.version.clone(),
            }
        })?;
        protocol.verify(&entry, &bytes)?;
        store.import_bytes(&bytes)?;
        protocol.materialize(&entry, &bytes, cargo_home)?;
        cargo_compat::write_source_checksum(
            &cargo_home
                .join("registry")
                .join("src")
                .join(format!("{}-{}", entry.name, entry.version)),
            &entry.sha256,
            &bytes,
        )?;
        bytes_from_cache = bytes_from_cache.saturating_add(bytes.len() as u64);
        added.push(pkg.id.clone());
    }

    Ok(InstallSummary {
        added,
        bytes_from_cache,
        duration_ms: started.elapsed().as_millis() as u64,
        cache_mode: InstallCacheMode::MgCStore,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
