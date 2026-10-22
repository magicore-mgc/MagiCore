//! Web shared-cache public facade.
//! Facade public cho cache dùng chung của Web.

mod cache_state;
mod cache_types;
mod cached_resolution;
mod metadata_store;
mod package_identity;
mod project_refs;
mod runtime_policy;

pub use crate::cache_metadata::*;
pub use crate::cache_prune::*;
pub use cache_state::SharedWebCache;
pub use cache_types::{
    CachedResolutionEnvelope, ExtractedPackageMarker, SharedCacheProjectRef,
    TarballContentSignature,
};
pub use package_identity::{extracted_package_cache_key, shared_extracted_package_root};
pub use project_refs::read_shared_cache_pinned_package_roots;
pub(crate) use runtime_policy::bounded_positive_concurrency;
pub use runtime_policy::{
    current_unix_secs, download_concurrency_limit, extract_concurrency_limit,
    resolve_prefetch_enabled,
};
