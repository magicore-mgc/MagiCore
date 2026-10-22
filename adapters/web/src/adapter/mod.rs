//! Web adapter owner wiring / wiring owner adapter web.
mod adapter;
pub use adapter::WebAdapter;
mod age_policy;
mod artifact_fetch;
mod audit;
mod capability_probes;
mod construction;
mod content_store;
mod identity;
mod lockfile_provider;
mod package_operations;
mod package_selection;
pub(crate) mod resolution;
mod state;
mod unclaimed_capabilities;
