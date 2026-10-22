//! Exports the registry candidate builder.
//! Export owner xây candidate registry.

mod builder;

pub(super) use builder::registry_chain_with_npmrc;
pub use builder::{DEFAULT_NPM_REGISTRY, WEB_REGISTRY_URL_ENV};
