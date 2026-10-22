//! Stable Web install download facade.
//! Facade ổn định cho luồng tải của Web.

mod dependency_pipeline;
mod prefetch;
mod task_concurrency;

pub use dependency_pipeline::pipeline_download_and_extract;
pub use prefetch::prefetch_tarballs;
pub use task_concurrency::pipeline_task_concurrency_limit;

// Re-export integrity helpers so existing callers keep their import paths.
// Giữ route import integrity hiện có cho caller.
pub use crate::install::integrity::{
    compute_sha256_b64_str, prepare_verified_tarball_for_cache, verify_sri_integrity,
    verify_tarball_integrity, verify_tarball_reader_integrity,
};

// Re-export fetch helpers so existing callers keep their import paths.
// Giữ route import fetch hiện có cho caller.
pub use crate::install::fetch::{get_tarball_bytes, package_tarball_url};
