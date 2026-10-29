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
    /// real for Bevy.
    ///
    /// LockfileProvider is NOT claimed, for a narrower reason than "the trait
    /// surface": `LockfileProvider` has exactly one method (`write_manifest`),
    /// and this adapter does implement it. It is withheld because the claim is
    /// per CORE while the implementation is per ENGINE — only Bevy writes a
    /// manifest, and Godot/Unity/Unreal fail closed. Claiming it would advertise
    /// `game` support to a Godot user whose write then fails.
    /// (Không claim LockfileProvider vì lý do hẹp hơn "chưa đủ trait": trait chỉ
    /// có một method và adapter CÓ impl. Claim là theo CORE trong khi impl theo
    /// ENGINE — chỉ Bevy ghi manifest, engine khác fail-closed.)
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
        // Shared optimizer dispatch writes profile/gpu for this core.
        Capability::OptimizerProvider,
    ];
}
