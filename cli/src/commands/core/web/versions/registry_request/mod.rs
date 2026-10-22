//! HTTP and npm metadata request ownership for scaffold resolution.
//! Owner request HTTP và metadata npm cho resolve scaffold.

mod registry_request;
#[cfg(all(test, feature = "web"))]
pub(in crate::commands::core::web::implementation) use registry_request::fallback_scaffold_version;
#[cfg(feature = "web")]
pub(super) use registry_request::fetch_age_gated_npm_metadata;
pub(crate) use registry_request::fetch_npm_latest_version;
