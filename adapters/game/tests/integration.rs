#![allow(clippy::unwrap_used)]
//! Game adapter integration tests; stable Cargo harness with focused behavior owners.
//! Integration test Game adapter; giữ harness Cargo ổn định và tách hành vi theo owner.

#[path = "integration/mod.rs"]
mod integration;

#[cfg(unix)]
#[test]
fn engine_detection_does_not_follow_external_mgc_config_symlink() {
    integration::symlink_safety::engine_detection_does_not_follow_external_mgc_config_symlink();
}

#[test]
fn detect_godot_via_project_godot() {
    integration::engine_markers::detect_godot_via_project_godot();
}

#[test]
fn detect_unity_via_packages_manifest() {
    integration::engine_markers::detect_unity_via_packages_manifest();
}

#[test]
fn detect_unreal_via_uproject() {
    integration::engine_markers::detect_unreal_via_uproject();
}

#[test]
fn detect_bevy_via_cargo_toml() {
    integration::engine_markers::detect_bevy_via_cargo_toml();
}

#[test]
fn detect_via_mgc_toml_overrides_file_marker() {
    integration::engine_config::detect_via_mgc_toml_overrides_file_marker();
}

#[test]
fn detect_returns_none_for_empty_dir() {
    integration::engine_config::detect_returns_none_for_empty_dir();
}

#[test]
fn adapter_for_returns_some_for_godot() {
    integration::adapter_discovery::adapter_for_returns_some_for_godot();
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
fn can_handle_returns_true_for_game_project() {
    integration::adapter_contract::can_handle_returns_true_for_game_project();
}

#[tokio::test]
async fn parse_manifest_godot_returns_game_manifest() {
    integration::manifest_parsing::parse_manifest_godot_returns_game_manifest().await;
}

#[tokio::test]
async fn parse_manifest_bevy_reads_cargo_toml_dependencies() {
    integration::manifest_parsing::parse_manifest_bevy_reads_cargo_toml_dependencies().await;
}

#[tokio::test]
async fn godot_install_fails_closed_pointing_to_editor() {
    integration::package_operations::godot_install_fails_closed_pointing_to_editor().await;
}

#[tokio::test]
async fn godot_add_fails_closed_until_mgc_owns_dependency_resolution() {
    integration::package_operations::godot_add_fails_closed_until_mgc_owns_dependency_resolution()
        .await;
}

#[tokio::test]
async fn godot_list_fails_closed_instead_of_returning_an_empty_native_list() {
    integration::package_operations::godot_list_fails_closed_instead_of_returning_an_empty_native_list().await;
}

#[tokio::test]
async fn bevy_list_does_not_report_manifest_ranges_as_installed_versions() {
    integration::package_operations::bevy_list_does_not_report_manifest_ranges_as_installed_versions().await;
}

#[tokio::test]
async fn bevy_adapter_dependency_operations_fail_closed_without_spawning_cargo() {
    integration::package_operations::bevy_adapter_dependency_operations_fail_closed_without_spawning_cargo().await;
}

#[tokio::test]
async fn unity_add_fails_closed_until_mgc_owns_upm_resolution() {
    integration::package_operations::unity_add_fails_closed_until_mgc_owns_upm_resolution().await;
}

#[tokio::test]
async fn audit_reports_unsupported_instead_of_a_clean_scan() {
    integration::audit_sbom::audit_reports_unsupported_instead_of_a_clean_scan().await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}
