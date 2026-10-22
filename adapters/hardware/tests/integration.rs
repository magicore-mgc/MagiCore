#![allow(clippy::unwrap_used)]
//! Integration tests for mgc-hardware-adapter — tests tích hợp adapter Hardware.

#[path = "integration/mod.rs"]
mod integration;

#[test]
fn adapter_for_returns_some_for_hardware_ecosystem() {
    integration::factory_detection::adapter_for_returns_some_for_hardware_ecosystem();
}

#[test]
fn adapter_for_returns_some_for_game_ecosystem_cross_core() {
    integration::factory_detection::adapter_for_returns_some_for_game_ecosystem_cross_core();
}

#[test]
fn adapter_for_returns_none_for_dir_without_mgc_toml() {
    integration::factory_detection::adapter_for_returns_none_for_dir_without_mgc_toml();
}

#[test]
fn adapter_for_returns_none_for_non_mgc_ecosystem() {
    integration::factory_detection::adapter_for_returns_none_for_non_mgc_ecosystem();
}

#[test]
fn adapter_name_and_ecosystem() {
    integration::adapter_contract::adapter_name_and_ecosystem();
}

#[test]
fn can_handle_returns_true_for_hardware_ecosystem() {
    integration::adapter_contract::can_handle_returns_true_for_hardware_ecosystem();
}

#[test]
fn can_handle_returns_false_for_plain_dir() {
    integration::adapter_contract::can_handle_returns_false_for_plain_dir();
}

#[tokio::test]
async fn adapter_package_list_rejects_template_inventory() {
    integration::package_operations::adapter_package_list_rejects_template_inventory().await;
}

#[tokio::test]
async fn audit_returns_clean_for_hardware_project() {
    integration::audit_sbom::audit_returns_clean_for_hardware_project().await;
}

#[tokio::test]
async fn parse_manifest_uses_dir_name() {
    integration::manifest_io::parse_manifest_uses_dir_name().await;
}

#[tokio::test]
async fn add_fails_for_hardware_adapter() {
    integration::package_operations::add_fails_for_hardware_adapter().await;
}

#[tokio::test]
async fn remove_fails_for_hardware_adapter() {
    integration::package_operations::remove_fails_for_hardware_adapter().await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}
