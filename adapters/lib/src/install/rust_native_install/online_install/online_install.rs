//! Downloads, verifies and materializes Rust registry packages online.
//! Tải, xác minh và materialize package registry Rust online.

use mgc_resolver::protocols::{CratesProtocol, RegistryProtocol};
use mgc_store::ContentStore;
use mgc_types::ResolvedGraph;
use mgc_types::adapter::{InstallCacheMode, InstallSummary};
use std::path::Path;
use std::time::Instant;

use super::super::super::{cargo_compat, content_store, entry_from_package};
use super::super::artifact_cache::{cargo_archive_path, read_cached_rust_artifact};
use super::super::error::RustNativeInstallError;
use super::super::source_readiness::{finalize_source, require_cargo_source_marker, source_root};

pub(in crate::install::rust_native_install) async fn install_rust_online(
    graph: &ResolvedGraph,
    cargo_home: &Path,
    started: Instant,
) -> Result<InstallSummary, RustNativeInstallError> {
    let protocol = CratesProtocol::from_env();
    let store = content_store()?;
    install_rust_online_with_protocol(graph, cargo_home, &protocol, &store, started).await
}

async fn install_rust_online_with_protocol(
    graph: &ResolvedGraph,
    cargo_home: &Path,
    protocol: &CratesProtocol,
    store: &ContentStore,
    started: Instant,
) -> Result<InstallSummary, RustNativeInstallError> {
    let mut added = Vec::with_capacity(graph.packages.len());
    let mut bytes_from_cache = 0_u64;

    for pkg in &graph.packages {
        let entry = entry_from_package(pkg);
        let archive_path = cargo_archive_path(cargo_home, &entry);
        let bytes = match read_cached_rust_artifact(&archive_path, &entry.sha256)? {
            Some(bytes) => {
                bytes_from_cache = bytes_from_cache.saturating_add(bytes.len() as u64);
                bytes
            }
            None => protocol.download(&entry).await?,
        };
        protocol.verify(&entry, &bytes)?;
        store.import_bytes(&bytes)?;
        let source_root = source_root(cargo_home, &entry.name, &entry.version);
        match std::fs::symlink_metadata(&source_root) {
            Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {
                cargo_compat::verify_source_checksum(&source_root, &entry.sha256, &bytes)?;
                // A directory that verifies its checksum record but has no Cargo
                // marker was written by an older MagiCore release; Cargo would
                // re-extract it on the next build. Repairing the marker here is
                // safe because the checksum record has just re-verified the tree.
                // (Thư mục checksum hợp lệ nhưng thiếu marker Cargo là do bản
                // MagiCore cũ ghi; Cargo sẽ giải nén lại ở build sau. Vá marker ở
                // đây an toàn vì checksum record vừa xác minh lại cây.)
                if require_cargo_source_marker(&source_root).is_err() {
                    mgc_resolver::protocols::write_cargo_source_marker(&source_root)?;
                }
            }
            Ok(_) => {
                return Err(RustNativeInstallError::SourceCacheNotRegularDirectory {
                    path: source_root.display().to_string(),
                });
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                protocol.materialize(&entry, &bytes, cargo_home)?;
                finalize_source(&source_root, &entry.sha256, &bytes)?;
            }
            Err(error) => {
                return Err(RustNativeInstallError::SourceCacheInspection {
                    path: source_root.display().to_string(),
                    source: error,
                });
            }
        }
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
