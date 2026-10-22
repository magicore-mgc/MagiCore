#![allow(clippy::unwrap_used)]
//! IoT adapter integration tests; stable Cargo harness with focused behavior owners.
//! Integration test IoT adapter; giữ harness Cargo ổn định và tách hành vi theo owner.

#[path = "integration/mod.rs"]
mod integration;

#[cfg(unix)]
#[test]
fn framework_detection_does_not_follow_external_mgc_config_symlink() {
    integration::symlink_safety::framework_detection_does_not_follow_external_mgc_config_symlink();
}

#[cfg(unix)]
#[test]
fn board_selection_does_not_follow_external_mgc_config_symlink() {
    integration::symlink_safety::board_selection_does_not_follow_external_mgc_config_symlink();
}

#[test]
fn detect_platformio_via_ini() {
    integration::framework_detection::detect_platformio_via_ini();
}

#[test]
fn detect_zephyr_via_west_yml() {
    integration::framework_detection::detect_zephyr_via_west_yml();
}

#[test]
fn detect_esp32_rust_via_mgc_toml() {
    integration::framework_config::detect_esp32_rust_via_mgc_toml();
}

#[test]
fn detect_returns_none_for_empty_dir() {
    integration::framework_detection::detect_returns_none_for_empty_dir();
}

#[test]
fn adapter_for_returns_some_for_platformio() {
    integration::adapter_discovery::adapter_for_returns_some_for_platformio();
}

#[test]
fn adapter_for_returns_none_for_plain_dir() {
    integration::adapter_discovery::adapter_for_returns_none_for_plain_dir();
}

#[test]
fn adapter_name_and_ecosystem() {
    integration::adapter_contract::adapter_name_and_ecosystem();
}

#[test]
fn can_handle_returns_true_for_iot_project() {
    integration::adapter_contract::can_handle_returns_true_for_iot_project();
}

#[tokio::test]
async fn platformio_add_fails_closed_outside_explicit_cli_compatibility() {
    integration::package_operations::platformio_add_fails_closed_outside_explicit_cli_compatibility().await;
}

#[tokio::test]
async fn zephyr_add_fails_closed_without_native_dependency_support() {
    integration::package_operations::zephyr_add_fails_closed_without_native_dependency_support()
        .await;
}

#[tokio::test]
async fn zephyr_list_fails_closed_instead_of_returning_an_empty_native_list() {
    integration::package_operations::zephyr_list_fails_closed_instead_of_returning_an_empty_native_list().await;
}

#[tokio::test]
async fn platformio_list_does_not_report_declared_ranges_as_installed_versions() {
    integration::package_operations::platformio_list_does_not_report_declared_ranges_as_installed_versions().await;
}

#[tokio::test]
async fn platformio_adapter_install_fails_closed_without_spawning_pio() {
    integration::package_operations::platformio_adapter_install_fails_closed_without_spawning_pio()
        .await;
}

#[tokio::test]
async fn audit_returns_clean_for_iot_project() {
    integration::audit_sbom::audit_returns_clean_for_iot_project().await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}

#[test]
fn embedded_board_registry_compatibility_api_matches_snapshot() {
    integration::board_registry::embedded_board_registry_compatibility_api_matches_snapshot();
}

#[test]
fn board_registry_filters_choices_by_framework_and_rejects_cross_framework_targets() {
    integration::board_registry::board_registry_filters_choices_by_framework_and_rejects_cross_framework_targets();
}

#[test]
fn platformio_esp32dev_registry_maps_to_its_esp32_target() {
    integration::board_registry::platformio_esp32dev_registry_maps_to_its_esp32_target();
}
