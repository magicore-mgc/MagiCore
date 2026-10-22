//! One resolved package edge payload, including peer links and lock flags. — Payload một package đã resolve, gồm peer link và lock flags.

use crate::package::PackageId;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResolvedPackage {
    pub id: PackageId,
    pub integrity: String,
    pub tarball_url: String,
    /// Regular (non-peer) dependencies.
    pub deps: Vec<PackageId>,
    /// Peer dependencies — resolved to actual graph members.
    /// Populated during the resolution phase so that the layout
    /// materializer does not need a secondary disk read of package.json.
    #[serde(default)]
    pub peer_deps: Vec<PackageId>,
    pub direct: bool,
    pub dev: bool,
}
