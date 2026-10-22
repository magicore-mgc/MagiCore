//! Game adapter integration test owners — các owner integration test của Game adapter.

#[path = "adapter_contract/mod.rs"]
pub mod adapter_contract;
#[path = "adapter_discovery/mod.rs"]
pub mod adapter_discovery;
#[path = "audit_sbom/mod.rs"]
pub mod audit_sbom;
#[path = "engine_config/mod.rs"]
pub mod engine_config;
#[path = "engine_markers/mod.rs"]
pub mod engine_markers;
#[path = "manifest_parsing/mod.rs"]
pub mod manifest_parsing;
#[path = "package_operations/mod.rs"]
pub mod package_operations;
#[path = "support/mod.rs"]
pub mod support;
#[path = "symlink_safety/mod.rs"]
pub mod symlink_safety;
