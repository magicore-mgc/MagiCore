//! Local typed Web scaffold version errors and compatibility constructors.
//! Lỗi version Web scaffold có kiểu tại chỗ và constructor tương thích.

mod error;
pub use error::{
    web_scaffold_age_metadata_unavailable, web_scaffold_dist_tag_invalid,
    web_scaffold_dist_tag_missing, web_scaffold_dist_tag_not_eligible,
    web_scaffold_latest_tag_invalid, web_scaffold_latest_tag_missing,
    web_scaffold_no_eligible_release, web_scaffold_no_eligible_release_in_range,
    web_scaffold_registry_version_unavailable, web_scaffold_security_config_unreadable,
    web_scaffold_store_policy_path_invalid, web_scaffold_store_policy_unreadable,
};
