//! Stable public exports for shared JavaScript resolution helpers.
//! Export public ổn định cho helper resolve JavaScript dùng chung.

pub use super::dist_tag_spec::is_dist_tag_spec;
pub use super::graph_normalization::{normalize_resolved_graph, prune_unreachable_packages};
pub use super::resolution_cache_key::manifest_resolution_cache_key;
pub use super::resolved_manifest::{is_manifest_root_package, manifest_with_resolved_specs};
pub use super::version_selection::{exact_version_from_range, select_resolved_candidate};
