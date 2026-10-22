//! Profile owner wiring and compatibility exports.
//! Nối owner profile và giữ export tương thích.

#[path = "install_profile/mod.rs"]
mod install_profile;
pub use install_profile::InstallProfile;
#[path = "resolve_profile/mod.rs"]
mod resolve_profile;
pub use resolve_profile::ResolveProfile;
#[path = "pipeline_profile/mod.rs"]
mod pipeline_profile;
pub use pipeline_profile::PipelineProfile;
#[path = "materialization_profile/mod.rs"]
mod materialization_profile;
pub use materialization_profile::MaterializationProfile;
#[path = "fetch_result/mod.rs"]
mod fetch_result;
pub use fetch_result::{TarballFetchResult, TarballPayload};
