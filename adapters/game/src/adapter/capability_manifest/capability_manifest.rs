use super::super::GameAdapter;
use mgc_types::capabilities::Capability;

impl GameAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `detect_engine`/`manifest_is_game` — real.
    /// - ScaffoldProvider: `mgc create-game <engine>` + src/scaffold — real.
    /// - ContentStoreProvider is not claimed: the CLI Bevy lane uses the
    ///   native Lib/Rust engine; this adapter's legacy helper delegates.
    /// - AuditProvider: shared-engine polyglot dispatch — real.
    ///
    /// DependencyResolver is NOT claimed (resolve is fail-closed — the
    /// engine toolchain owns the graph), though add/remove/update stay
    /// real for Bevy; LockfileProvider is NOT claimed (Bevy-only
    /// write_manifest is not the full trait surface).
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
    ];
}
