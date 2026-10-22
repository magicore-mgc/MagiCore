//! Cache prune owner wiring.
//! Khai báo dispatcher prune và các chính sách theo loại cache.

mod cas_blobs;
mod files;
mod prune;
mod web_project;
mod web_shared;

pub(super) use cas_blobs::prune_cas_blobs_under;
pub(super) use files::{prune_files_under, prune_unlinked_files_under};
pub(super) use prune::prune;
#[cfg(test)]
pub(super) use prune::prune_generic_cache;
pub(super) use web_project::{prune_web_project_cache, web_project_cache_stats};
pub(super) use web_shared::{
    count_web_shared_unpinned_package_roots, prune_web_shared_unpinned_package_roots,
    web_shared_cache_stats,
};
