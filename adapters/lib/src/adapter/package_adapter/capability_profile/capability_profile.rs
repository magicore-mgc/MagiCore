//! Capability claims and per-language profile selection for LibAdapter.
//! Claim capability và chọn profile theo ngôn ngữ cho LibAdapter.

use crate::adapter::types::{JavaManifestKind, LibAdapter};
use crate::language::LibLanguage;
use mgc_types::capabilities::Capability;

impl LibAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `detect_language`/`manifest_is_lib` — real.
    /// - ScaffoldProvider: `mgc create-lib <lang>` CLI lane — real.
    /// - DependencyResolver: native CLI lanes resolve through the protocol
    ///   engine; all manifest mutations commit through the journaled CLI
    ///   gateway. Direct adapter add/remove/update methods fail closed.
    ///   TS rides the embedded web engine.
    /// - LockfileProvider: real manifest writers (cargo/pyproject/web).
    /// - ArtifactFetcher: standalone `fetch(graph)` is supported only by
    ///   TypeScript's web engine. Other native lanes fetch inside install
    ///   and do not claim the independent capability.
    /// - ContentStoreProvider: crate::install::run_install — native
    ///   download → verify → CAS import → materialize the toolchain layout
    ///   (cargo/pypi/go/maven/nuget), shared store for TS — real.
    /// - AuditProvider: per-language scanner dispatch — real.
    ///
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật.
    /// Base: native resolve/manifest/store operations; fetch remains an
    /// install-internal step outside the standalone ArtifactFetcher trait.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::DependencyResolver,
        Capability::LockfileProvider,
        Capability::ContentStoreProvider,
        Capability::AuditProvider,
    ];

    /// Java Gradle is a build program, not an MGC-owned dependency
    /// manifest. It may be detected and audited, but registry mutation
    /// capabilities stay unclaimed. (Gradle là chương trình build, không
    /// phải manifest MGC sở hữu.)
    pub const CAPABILITIES_JAVA_DELEGATED: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
    ];

    /// Python projects owned by another manager retain detection/scanning,
    /// but do not claim native dependency mutation or installation.
    /// (Project Python do manager khác sở hữu chỉ claim detect/audit.)
    pub const CAPABILITIES_PYTHON_UNOWNED: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
    ];

    /// TS lane đi trên web engine nhúng — materializer (node_modules
    /// layout) + lifecycle (scripts) THẬT qua engine đó, nên instance
    /// TS claim thêm 2 capability này; ngôn ngữ toolchain-owned
    /// (rust/python/go/...) KHÔNG claim (materialize/toolchain thuộc
    /// toolchain của chúng — trung thực).
    /// (The TS lane rides the embedded web engine — materializer +
    /// lifecycle are real THROUGH that engine, so a TS instance claims
    /// them; toolchain-owned languages do not.)
    pub const CAPABILITIES_TS: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::DependencyResolver,
        Capability::LockfileProvider,
        Capability::ArtifactFetcher,
        Capability::ContentStoreProvider,
        Capability::Materializer,
        Capability::LifecycleRunner,
        Capability::AuditProvider,
    ];
}

pub(in crate::adapter::package_adapter) fn select_capabilities(
    adapter: &LibAdapter,
) -> &'static [Capability] {
    match adapter.language {
        LibLanguage::Ts => LibAdapter::CAPABILITIES_TS,
        LibLanguage::Python if !adapter.python_native_owned() => {
            LibAdapter::CAPABILITIES_PYTHON_UNOWNED
        }
        LibLanguage::Java if adapter.java_kind != JavaManifestKind::Pom => {
            LibAdapter::CAPABILITIES_JAVA_DELEGATED
        }
        _ => LibAdapter::CAPABILITIES,
    }
}
