//! Stable Web scaffold version routes over focused version owners.
//! Route version scaffold Web ổn định qua các owner chuyên trách.

mod age_policy;
mod dist_tag;
#[cfg(feature = "web")]
mod eligible_release;
pub mod error;
mod registry_request;
mod seed_resolution;
mod versions;

pub(crate) use age_policy::scaffold_age_policy;
pub(crate) use dist_tag::resolve_npm_dist_tag_version;
pub(crate) use registry_request::fetch_npm_latest_version;
pub(crate) use seed_resolution::resolve_seed_dependency_version;
pub(crate) use versions::ScaffoldAgePolicy;

#[cfg(all(test, feature = "web"))]
pub(in crate::commands::core::web::implementation) use age_policy::store_release_policy;
#[cfg(all(test, feature = "web"))]
pub(in crate::commands::core::web::implementation) use dist_tag::selected_dist_tag_version;
#[cfg(all(test, feature = "web"))]
pub(crate) use eligible_release::{
    select_latest_eligible_version, validate_seed_range_age_eligible,
};
#[cfg(all(test, feature = "web"))]
pub(in crate::commands::core::web::implementation) use registry_request::fallback_scaffold_version;
