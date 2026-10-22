//! Web shared cache references owner wiring.
//! File này chỉ nối các owner package-root, reference và thao tác cache Web.

mod package_roots;
mod references;
mod web_shared;

pub(in crate::commands::cache) use web_shared::{
    count_web_shared_unpinned_package_roots, prune_web_shared_unpinned_package_roots,
    web_shared_cache_stats,
};
