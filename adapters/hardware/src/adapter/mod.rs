//! Hardware adapter facade with stable public exports.
//! Facade adapter hardware giữ các public export ổn định.

mod adapter;
mod adapter_factory;
mod audit_provider;
mod capability_manifest;
mod core_identity;
mod optimizer_provider;
mod package_adapter;
mod project_detection;
mod unclaimed_capabilities;

pub use adapter::HardwareAdapter;
pub use adapter_factory::adapter_for;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
