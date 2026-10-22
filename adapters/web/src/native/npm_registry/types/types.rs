//! NPM registry model index.
// Tập hợp các kiểu dữ liệu và trạng thái registry.

pub use super::downloaded_tarball::downloaded_tarball::DownloadedTarball;
pub use super::package_metadata::package_metadata::{
    DistInfo, PackageMetadata, PeerDependencyMeta, VersionInfo,
};
pub use super::registry_state::registry_state::NpmRegistry;
pub use super::scoped_client_state::scoped_client_state::ScopedRegistryClient;
