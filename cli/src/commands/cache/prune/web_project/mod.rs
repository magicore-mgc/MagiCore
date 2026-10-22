//! Web project cache stats and pruning owner wiring.
//! File này chỉ nối module web_project và export API nội bộ.

mod web_project;

pub(in crate::commands::cache) use web_project::{
    prune_web_project_cache, web_project_cache_stats,
};
