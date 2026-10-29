//! IoT adapter owners — các owner của IoT adapter.

mod adapter;
mod audit_provider;
mod content_store_provider;
mod dependency_resolver;
mod factory;
mod identity;
mod lockfile_provider;
mod optimizer_provider;
mod package_adapter;
mod platformio_manifest;
mod project_detection;
mod scaffold_provider;
mod unclaimed_capabilities;

pub use adapter::IotAdapter;
pub use factory::adapter_for;
