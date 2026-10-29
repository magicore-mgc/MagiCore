//! Cloud adapter module exports — export module adapter Cloud.

mod adapter;
mod artifact_fetcher;
mod audit_provider;
mod content_store;
mod core_identity;
mod dependency_resolver;
mod deploy_provider;
mod factory;
mod lifecycle_runner;
mod lockfile_provider;
mod optimizer_provider;
mod package_adapter;
mod project_detection;
mod scaffold_provider;
mod unclaimed_capabilities;

pub use adapter::CloudAdapter;
pub use factory::adapter_for;
