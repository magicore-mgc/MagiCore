//! Stable public facade for the native npm registry client.
//! Facade công khai ổn định cho npm registry client của native adapter.
pub use super::batch::batch::batch_fetch_metadata;
pub use super::batch::tarball::tarball::{
    batch_download_tarball, batch_download_tarball_with_auth,
};
pub use super::client::client::batch_http_client;
pub use super::metadata::publish_age::publish_age::check_publish_age;
pub use super::registry::tarball::automatic::automatic::LARGE_PKG_THRESHOLD_BYTES;
pub use super::security::credential_scope::credential_scope::scoped_registry_request;
pub use super::types::types::{
    DistInfo, DownloadedTarball, NpmRegistry, PackageMetadata, PeerDependencyMeta,
    ScopedRegistryClient, VersionInfo,
};
