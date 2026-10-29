//! Template registry command facade and owner-local boundaries.
//! Facade registry template và ranh giới owner.

mod archive_extract;
mod cache_contract;
mod cache_filesystem;
mod cache_location;
mod cache_lock;
mod command_types;
mod fetch_resolution;
mod identity;
mod integrity;
mod layer_cache;
mod registry_config;
mod registry_download;
mod registry_url;
mod template_names;
pub(crate) use archive_extract::extract_tarball_into;
pub(crate) use cache_contract::read_template_cache_version;
pub(crate) use cache_contract::template_layer_has_contract;
use cache_filesystem::ensure_template_cache_parent;
pub(crate) use cache_filesystem::template_cache_is_link;
pub use cache_location::templates_cache_dir;
pub(crate) use cache_lock::lock_template_cache_layer;
pub(crate) use command_types::TemplateFetchArgs;
use fetch_resolution::resolve_layer_at;
use identity::validate_template_identity;
use integrity::verify_template_integrity;
#[cfg(test)]
use layer_cache::ensure_layer_in;
#[cfg(test)]
use layer_cache::template_resolution_error;
#[cfg(test)]
pub(crate) use layer_cache::validate_cached_layer_path_in;
pub use layer_cache::{ensure_cache_healthy, ensure_layer};
use registry_config::MAX_TEMPLATE_METADATA_BYTES;
use registry_config::MAX_TEMPLATE_REDIRECTS;
use registry_config::MAX_TEMPLATE_TARBALL_BYTES;
use registry_config::MAX_TEMPLATE_VERSION_BYTES;
use registry_config::TEMPLATE_FETCH_TIMEOUT_SECS;
use registry_config::register_fetch_auth;
use registry_config::select_registry;
use registry_download::download_template_bytes;
use registry_url::template_auth_allowed;
use registry_url::template_registry_url;
use registry_url::validate_template_download_url;
pub use template_names::layer_rel;
pub use template_names::registry_package_name;
#[cfg(test)]
mod test;
