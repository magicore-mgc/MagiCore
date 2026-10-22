#![allow(clippy::unwrap_used)]
//! Cloud adapter integration tests; stable Cargo harness with focused behavior owners.
//! Integration test Cloud adapter; giữ harness Cargo ổn định và tách hành vi theo owner.

#[path = "integration/mod.rs"]
mod integration;

#[cfg(unix)]
#[test]
fn type_detection_does_not_follow_external_mgc_config_symlink() {
    integration::symlink_safety::type_detection_does_not_follow_external_mgc_config_symlink();
}

#[cfg(unix)]
#[test]
fn type_detection_does_not_follow_external_package_json_symlink() {
    integration::symlink_safety::type_detection_does_not_follow_external_package_json_symlink();
}

#[test]
fn detect_cloudflare_via_wrangler_toml() {
    integration::file_markers::detect_cloudflare_via_wrangler_toml();
}

#[test]
fn detect_pulumi_via_pulumi_yaml() {
    integration::file_markers::detect_pulumi_via_pulumi_yaml();
}

#[test]
fn detect_terraform_via_dot_tf_file() {
    integration::file_markers::detect_terraform_via_dot_tf_file();
}

#[test]
fn detect_terraform_via_any_tf_extension() {
    integration::file_markers::detect_terraform_via_any_tf_extension();
}

#[test]
fn detect_cdk_via_package_json_aws_cdk_lib() {
    integration::file_markers::detect_cdk_via_package_json_aws_cdk_lib();
}

#[test]
fn detect_cdk_via_package_json_cdk_key() {
    integration::file_markers::detect_cdk_via_package_json_cdk_key();
}

#[test]
fn mgc_toml_overrides_file_marker_for_cloud_type() {
    integration::cloud_config::mgc_toml_overrides_file_marker_for_cloud_type();
}

#[test]
fn detect_cloudflare_via_mgc_toml() {
    integration::cloud_config::detect_cloudflare_via_mgc_toml();
}

#[test]
fn mgc_toml_recognizes_cdk_typescript_framework_alias() {
    integration::cloud_config::mgc_toml_recognizes_cdk_typescript_framework_alias();
}

#[test]
fn mgc_toml_recognizes_terraform_gcp_framework_alias() {
    integration::cloud_config::mgc_toml_recognizes_terraform_gcp_framework_alias();
}

#[test]
fn detect_returns_none_for_empty_dir() {
    integration::cloud_config::detect_returns_none_for_empty_dir();
}

#[test]
fn detect_returns_none_for_unknown_type_in_mgc_toml() {
    integration::cloud_config::detect_returns_none_for_unknown_type_in_mgc_toml();
}

#[test]
fn package_json_without_cdk_deps_not_detected_as_cdk() {
    integration::file_markers::package_json_without_cdk_deps_not_detected_as_cdk();
}

#[test]
fn cloud_type_as_str_values() {
    integration::cloud_type_model::cloud_type_as_str_values();
}

#[test]
fn adapter_for_returns_some_for_terraform() {
    integration::adapter_discovery::adapter_for_returns_some_for_terraform();
}

#[test]
fn adapter_for_returns_none_without_any_marker() {
    integration::adapter_discovery::adapter_for_returns_none_without_any_marker();
}

#[test]
fn adapter_cloud_type_method_returns_correct_str() {
    integration::adapter_discovery::adapter_cloud_type_method_returns_correct_str();
}

#[test]
fn adapter_name_and_ecosystem() {
    integration::adapter_contract::adapter_name_and_ecosystem();
}

#[test]
fn can_handle_returns_true_for_known_marker() {
    integration::adapter_contract::can_handle_returns_true_for_known_marker();
}

#[tokio::test]
async fn parse_manifest_uses_dir_name_for_terraform() {
    integration::package_operations::parse_manifest_uses_dir_name_for_terraform().await;
}

#[tokio::test]
async fn terraform_package_adapter_install_fails_closed_without_spawning_terraform() {
    integration::package_operations::terraform_package_adapter_install_fails_closed_without_spawning_terraform().await;
}

#[tokio::test]
async fn add_fails_closed_for_terraform() {
    integration::package_operations::add_fails_closed_for_terraform().await;
}

#[tokio::test]
async fn remove_fails_closed_for_terraform() {
    integration::package_operations::remove_fails_closed_for_terraform().await;
}

#[tokio::test]
async fn update_fails_closed_for_terraform() {
    integration::package_operations::update_fails_closed_for_terraform().await;
}

#[tokio::test]
async fn terraform_list_fails_closed_instead_of_returning_an_empty_native_list() {
    integration::package_operations::terraform_list_fails_closed_instead_of_returning_an_empty_native_list().await;
}

#[tokio::test]
async fn pulumi_without_package_json_list_fails_closed() {
    integration::package_operations::pulumi_without_package_json_list_fails_closed().await;
}

#[tokio::test]
async fn audit_returns_clean_for_terraform_project() {
    integration::audit_sbom::audit_returns_clean_for_terraform_project().await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}
