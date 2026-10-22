// Computation-cache facade — mặt nối computation cache

mod cache_file;
mod computation_cache;
mod dependency_fingerprint;
mod file_hash;
mod hash_framing;
mod path_key;
mod record;
mod source_entries;
mod source_fingerprint;
mod source_walk;

pub use cache_file::{load_package_build_cache, save_package_build_cache};
pub use computation_cache::check_package_build_freshness;
pub use dependency_fingerprint::compute_composite_hash;
pub use file_hash::hash_file;
pub use record::PackageBuildCache;
pub use source_fingerprint::compute_package_source_hash;
