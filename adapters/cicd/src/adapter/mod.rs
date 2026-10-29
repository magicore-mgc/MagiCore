//! CI/CD adapter facade with stable public exports.
//! Facade adapter CICD giữ các public export ổn định.

mod adapter;
mod adapter_factory;
mod audit_provider;
mod capability_manifest;
mod core_identity;
mod optimizer_provider;
mod package_adapter;
mod project_detection;
mod provider_name;
mod scaffold_provider;
mod unclaimed_capabilities;

pub use adapter::CicdAdapter;
pub use adapter_factory::adapter_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
