//! Library install lifecycle coordinator and language dispatch.
//! Điều phối lifecycle cài thư viện và định tuyến theo ngôn ngữ.

use super::{
    install_empty_resolved_graph, validate_native_install_inputs, validate_offline_install,
};
use mgc_lockfile::EcosystemTag;
use mgc_store::ContentStore;
use mgc_types::ContentStoreProvider;
use mgc_types::adapter::{InstallOptions, InstallSummary};
use mgc_types::{MgError, MgResult, ResolvedGraph};
use std::path::Path;

use crate::install::go_native_install::install_go_native;
use crate::install::maven_native_install::install_maven_native;
use crate::install::nuget_native_install::install_nuget_native;
use crate::install::python_native_install::install_python_native;
use crate::install::rust_native_install::install_rust_native;
use crate::install::{cargo_compat, direct_package_ids, write_canonical_lock_with_roots};
use crate::language::LibLanguage;

/// Install coordinator for the existing Lib adapter lifecycle.
/// Điều phối lifecycle hiện có của adapter Lib.
pub(crate) async fn run_install(
    language: LibLanguage,
    web: Option<&mgc_web_adapter::WebAdapter>,
    graph: &ResolvedGraph,
    project_root: &Path,
    opts: InstallOptions,
    _store: Option<&ContentStore>,
    lock_packages: Vec<mgc_lockfile::Package>,
) -> MgResult<InstallSummary> {
    validate_offline_install(language, opts.offline)?;

    if language != LibLanguage::Ts {
        validate_native_install_inputs(language, graph, project_root, &lock_packages)?;
        if graph.packages.is_empty() {
            return install_empty_resolved_graph(
                language,
                graph,
                project_root,
                opts.offline,
                lock_packages,
            );
        }
    }

    match language {
        LibLanguage::Ts => {
            let web = web.ok_or_else(|| {
                MgError::Other("TypeScript lib requires web adapter delegate".to_string())
            })?;
            web.install(graph, project_root, opts).await
        }
        LibLanguage::Rust => {
            let summary = install_rust_native(graph, opts.offline).await?;
            if !opts.offline {
                write_canonical_lock_with_roots(
                    project_root,
                    EcosystemTag::Rust,
                    lock_packages,
                    direct_package_ids(graph),
                )?;
            }
            cargo_compat::refresh_lock(project_root)?;
            Ok(summary)
        }
        LibLanguage::Python => {
            let summary = install_python_native(graph, &lock_packages, opts.offline).await?;
            if !opts.offline {
                write_canonical_lock_with_roots(
                    project_root,
                    EcosystemTag::Python,
                    lock_packages,
                    direct_package_ids(graph),
                )?;
            }
            Ok(summary)
        }
        // Go: native module proxy engine (Phase 2) — zip download →
        // ziphash/sumdb verify → CAS → download-cache materialization.
        // (Go: engine module proxy native (Phase 2) — tải zip → verify
        // ziphash/sumdb → CAS → materialize download cache.)
        LibLanguage::Go => {
            let summary = install_go_native(graph, &lock_packages).await?;
            write_canonical_lock_with_roots(
                project_root,
                EcosystemTag::Go,
                lock_packages,
                direct_package_ids(graph),
            )?;
            Ok(summary)
        }
        // Java: native Maven engine (Phase 2) — jar+pom download →
        // sha256/sha1 verify → CAS → local-repo materialization
        // (`mvn -o` readable). Gradle-owned installs stay unsupported
        // (build scripts are programs — honest failure, never a silent
        // no-op summary).
        // (Java: engine Maven native (Phase 2) — tải jar+pom → verify
        // sha256/sha1 → materialize local repo (đọc được bởi `mvn -o`).
        // Gradle-owned installs stay unsupported vì build script là chương
        // trình — phải fail trung thực, không trả summary no-op âm thầm.)
        LibLanguage::Java => {
            let summary = install_maven_native(graph, &lock_packages).await?;
            write_canonical_lock_with_roots(
                project_root,
                EcosystemTag::Maven,
                lock_packages,
                direct_package_ids(graph),
            )?;
            Ok(summary)
        }
        // .NET: native NuGet v3 engine (Phase 2) — nupkg download →
        // registration SHA-512 verify → CAS → global-packages materialization
        // (`dotnet restore --source`-readable layout).
        // (.NET: engine NuGet v3 native (Phase 2) — tải nupkg → verify
        // SHA-512 theo registration → CAS → materialize global-packages
        // (layout mà `dotnet restore --source` đọc được).)
        LibLanguage::DotNet => {
            let summary = install_nuget_native(graph, &lock_packages).await?;
            write_canonical_lock_with_roots(
                project_root,
                EcosystemTag::NuGet,
                lock_packages,
                direct_package_ids(graph),
            )?;
            Ok(summary)
        }
    }
}
