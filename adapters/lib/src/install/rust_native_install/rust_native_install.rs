//! Selects online or offline native Rust installation.
//! Chọn luồng install Rust native online hoặc offline.

use mgc_types::ResolvedGraph;
use mgc_types::adapter::InstallSummary;
use std::time::Instant;

use super::super::{content_store, shared_store};
use super::error::RustNativeInstallError;
use super::offline_install::{install_rust_cached, preflight_rust_artifact_cache};
use super::online_install::install_rust_online;

/// Install Rust packages using MGC-owned online or offline cache flows.
/// Cài package Rust qua luồng cache online hoặc offline do MGC quản lý.
pub(in crate::install) async fn install_rust_native(
    graph: &ResolvedGraph,
    offline: bool,
) -> Result<InstallSummary, RustNativeInstallError> {
    let started = Instant::now();
    let cargo_home = if offline {
        shared_store::SharedStoreRun::cargo_path()?.install_root
    } else {
        shared_store::SharedStoreRun::cargo()?.install_root
    };
    if offline {
        preflight_rust_artifact_cache(graph, &cargo_home)?;
        let store = content_store()?;
        return install_rust_cached(graph, &cargo_home, &store, started);
    }

    install_rust_online(graph, &cargo_home, started).await
}
