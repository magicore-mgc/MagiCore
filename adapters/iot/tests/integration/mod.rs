//! IoT adapter integration-test owners — các owner integration test của IoT adapter.

#[path = "adapter_contract/mod.rs"]
pub mod adapter_contract;
#[path = "adapter_discovery/mod.rs"]
pub mod adapter_discovery;
#[path = "audit_sbom/mod.rs"]
pub mod audit_sbom;
#[path = "board_registry/mod.rs"]
pub mod board_registry;
#[path = "framework_config/mod.rs"]
pub mod framework_config;
#[path = "framework_detection/mod.rs"]
pub mod framework_detection;
#[path = "package_operations/mod.rs"]
pub mod package_operations;
#[path = "support/mod.rs"]
pub mod support;
#[path = "symlink_safety/mod.rs"]
pub mod symlink_safety;
