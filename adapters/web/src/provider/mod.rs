//! Npm dependency provider owner tree / cây owner npm dependency provider.
mod dependency_graph;
mod dependency_provider;
mod eligible_versions;
mod enqueue_policy;
mod metadata;
mod metadata_prefetch;
mod npm_timestamp;
mod pinned_version;
mod platform_support;
mod prefetch_dependencies;
mod prefetch_versions;
mod provider;
mod release_age;
mod version_selection;

pub use eligible_versions::eligible_versions;
#[cfg(test)]
pub(crate) use npm_timestamp::parse_npm_time;
pub use pinned_version::check_pinned_version;
pub use provider::NpmDependencyProvider;
pub use release_age::AgePolicy;
