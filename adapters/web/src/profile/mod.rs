//! Public profiling and fetch-result types for the Web adapter.
//! Export public type profile và fetch result của Web adapter.

mod profile;
pub use profile::{
    InstallProfile, MaterializationProfile, PipelineProfile, ResolveProfile, TarballFetchResult,
    TarballPayload,
};
