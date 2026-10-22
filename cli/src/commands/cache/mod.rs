//! CLI cache command owner wiring.
//! File này chỉ nối các owner và giữ route lệnh công khai.

mod cache;
mod clean;
mod filesystem;
mod prune;
mod status;
mod storage_budget;
mod target_selection;
mod types;

pub use cache::run;

use clean::clean;
use prune::prune;
use status::print_status;
use target_selection::cache_entries;
use types::{CacheAction, CacheEntry, CacheTarget};

#[cfg(test)]
use prune::{
    prune_web_project_cache, prune_web_shared_unpinned_package_roots, web_project_cache_stats,
    web_shared_cache_stats,
};
#[cfg(test)]
use storage_budget::{combined_path_size, measured_path_size};
#[cfg(test)]
use target_selection::{find_cargo_workspace_root, shared_cache_path};
#[cfg(test)]
use types::{WebProjectCacheStats, WebProjectPruneStats};
#[cfg(test)]
#[path = "../../test/cache_test.rs"]
mod tests;
