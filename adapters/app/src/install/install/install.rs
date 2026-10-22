//! App install entry point and lifecycle dispatch.
//! Điểm vào install App và điều phối lifecycle.
use crate::language::AppLanguage;
use mgc_resolver::protocols::SwiftRegistryProtocol;
use mgc_store::ContentStore;
use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::Path;

use super::language_gate::{ensure_dispatchable_language, ensure_offline_supported};

/// Install orchestrator for natively supported Flutter and Swift app projects.
/// Điều phối install cho project Flutter và Swift được hỗ trợ native.
pub async fn run_install(
    language: AppLanguage,
    graph: &ResolvedGraph,
    project_root: &Path,
    opts: InstallOptions,
    _store: Option<&ContentStore>,
    lock_packages: Vec<mgc_lockfile::Package>,
) -> MgResult<InstallSummary> {
    run_install_with_swift_protocol(
        language,
        graph,
        project_root,
        opts,
        _store,
        lock_packages,
        None,
    )
    .await
}

pub(crate) async fn run_install_with_swift_protocol(
    language: AppLanguage,
    graph: &ResolvedGraph,
    project_root: &Path,
    opts: InstallOptions,
    _store: Option<&ContentStore>,
    lock_packages: Vec<mgc_lockfile::Package>,
    swift_protocol: Option<SwiftRegistryProtocol>,
) -> MgResult<InstallSummary> {
    ensure_dispatchable_language(language)?;

    // Only Flutter has verified cache-only archive replay; other lanes stay closed.
    // Chỉ Flutter có replay archive không mạng; lane khác vẫn từ chối offline.
    ensure_offline_supported(language, opts.offline)?;

    match language {
        // Flutter: native pub.dev engine (Phase 2) — no `flutter pub get`
        // spawn for resolve/fetch/install.
        AppLanguage::Flutter => super::flutter::run_flutter_install(
            graph,
            project_root,
            opts.offline,
            opts.replay_from_lock,
            lock_packages,
        )
        .await,

        AppLanguage::Kotlin => Err(MgError::Unsupported {
            core: "app",
            capability: "native Kotlin dependency install",
            guidance: "MagiCore does not yet implement native Kotlin dependency resolution and materialization".to_string(),
        }),

        // Swift registry archives are verified and materialized by MGC;
        // Git-source dependencies are rejected by the resolver.
        // (MGC xác minh và materialize archive Swift registry; dependency Git
        // bị resolver từ chối.)
        AppLanguage::Swift => super::swift::run_swift_install(
            graph,
            project_root,
            opts.replay_from_lock,
            lock_packages,
            swift_protocol,
        )
        .await,

        // RN spans JS, Android, and iOS package graphs. Until all three
        // graphs are owned and committed atomically, fail instead of
        // reporting a partially installed project as successful.
        AppLanguage::ReactNative => Err(MgError::Unsupported {
            core: "app",
            capability: "complete native React Native dependency install",
            guidance: "MagiCore requires native JS, Maven, and CocoaPods engines plus one atomic cross-ecosystem transaction".to_string(),
        }),

        AppLanguage::ObjC
        | AppLanguage::Multi
        | AppLanguage::Maui
        | AppLanguage::Tauri => Err(MgError::Unsupported {
            core: "app",
            capability: "native dependency install",
            guidance: "MagiCore does not yet implement a native dependency engine for this app ecosystem".to_string(),
        }),
    }
}
