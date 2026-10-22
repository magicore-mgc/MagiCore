#![allow(clippy::unwrap_used)]
//! CI/CD adapter integration tests; stable Cargo harness with focused behavior owners.
//! Integration test CICD adapter; giữ harness Cargo ổn định và tách hành vi theo owner.

#[path = "integration/mod.rs"]
mod integration;

#[cfg(unix)]
#[test]
fn provider_detection_does_not_follow_external_mgc_config_symlink() {
    integration::provider_detection::provider_detection_does_not_follow_external_mgc_config_symlink(
    );
}

#[test]
fn detect_cloudflare_via_wrangler_toml() {
    integration::provider_detection::detect_cloudflare_via_wrangler_toml();
}

#[test]
fn detect_github_actions_via_workflows_dir() {
    integration::provider_detection::detect_github_actions_via_workflows_dir();
}

#[test]
fn detect_gitlab_via_gitlab_ci_yml() {
    integration::provider_detection::detect_gitlab_via_gitlab_ci_yml();
}

#[test]
fn detect_circleci_via_config_yml() {
    integration::provider_detection::detect_circleci_via_config_yml();
}

#[test]
fn detect_argocd_via_application_yaml() {
    integration::provider_detection::detect_argocd_via_application_yaml();
}

#[test]
fn detect_aws_via_main_tf() {
    integration::provider_detection::detect_aws_via_main_tf();
}

#[test]
fn detect_aws_via_mgc_toml_overrides_file_marker() {
    integration::provider_detection::detect_aws_via_mgc_toml_overrides_file_marker();
}

#[test]
fn detect_gcp_via_mgc_toml() {
    integration::provider_detection::detect_gcp_via_mgc_toml();
}

#[test]
fn detect_returns_none_for_empty_dir() {
    integration::provider_detection::detect_returns_none_for_empty_dir();
}

#[test]
fn detect_returns_none_for_unknown_mgc_toml_provider() {
    integration::provider_detection::detect_returns_none_for_unknown_mgc_toml_provider();
}

#[test]
fn cicd_provider_as_str_values() {
    integration::provider_model::cicd_provider_as_str_values();
}

#[test]
fn adapter_for_returns_some_with_wrangler() {
    integration::adapter_discovery::adapter_for_returns_some_with_wrangler();
}

#[tokio::test]
async fn list_fails_closed_because_pipeline_files_are_not_packages() {
    integration::package_operations::list_fails_closed_because_pipeline_files_are_not_packages()
        .await;
}

#[test]
fn adapter_for_returns_none_without_markers() {
    integration::adapter_discovery::adapter_for_returns_none_without_markers();
}

#[test]
fn adapter_provider_method_returns_correct_str() {
    integration::adapter_discovery::adapter_provider_method_returns_correct_str();
}

#[test]
fn adapter_name_and_ecosystem() {
    integration::adapter_contract::adapter_name_and_ecosystem();
}

#[test]
fn can_handle_returns_true_for_known_marker() {
    integration::adapter_discovery::can_handle_returns_true_for_known_marker();
}

#[tokio::test]
async fn install_fails_closed_because_no_package_semantics() {
    integration::package_operations::install_fails_closed_because_no_package_semantics().await;
}

#[tokio::test]
async fn add_fails_closed_with_error_mentioning_deploy() {
    integration::package_operations::add_fails_closed_with_error_mentioning_deploy().await;
}

#[tokio::test]
async fn remove_fails_closed() {
    integration::package_operations::remove_fails_closed().await;
}

#[tokio::test]
async fn update_fails_closed() {
    integration::package_operations::update_fails_closed().await;
}

#[tokio::test]
async fn audit_returns_clean_for_empty_cicd_project() {
    integration::audit_sbom::audit_returns_clean_for_empty_cicd_project().await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}
