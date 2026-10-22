//! Declare only implemented CI/CD adapter capabilities.
//! Chỉ khai báo capability adapter CICD đã triển khai.

use super::super::adapter::CicdAdapter;
use mgc_types::capabilities::Capability;

impl CicdAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `manifest_is_cicd`/`detect_provider` — real.
    /// - ScaffoldProvider: `mgc create-cicd <provider>` (the scaffold lane
    ///   the old write_manifest guidance pointed at) — real.
    /// - AuditProvider: the github-actions policy lane + polyglot engine
    ///   (audit body below) — real.
    ///
    /// DependencyResolver/ArtifactFetcher/ContentStoreProvider/
    /// LockfileProvider are NOT claimed — pipeline files are hand-owned
    /// and `mgc deploy` (dry-run default) is the CLI lane, not the
    /// adapter surface.
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        Capability::ScaffoldProvider,
        Capability::AuditProvider,
    ];
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
