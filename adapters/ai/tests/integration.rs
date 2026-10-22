#![allow(clippy::unwrap_used)]
//! AI adapter integration tests — integration test của AI adapter.
//! Stable Cargo harness; behavior lives in focused owner modules. — Harness Cargo ổn định; hành vi nằm trong owner riêng.

#[path = "integration/mod.rs"]
mod integration;

#[cfg(unix)]
#[test]
fn framework_detection_does_not_follow_external_mgc_config_symlink() {
    integration::project_detection::framework_detection_does_not_follow_external_mgc_config_symlink(
    );
}

#[cfg(unix)]
#[test]
fn framework_detection_does_not_follow_external_pyproject_symlink() {
    integration::project_detection::framework_detection_does_not_follow_external_pyproject_symlink(
    );
}

#[test]
fn native_python_lane_does_not_take_over_foreign_lockfiles() {
    integration::python_lane::native_python_lane_does_not_take_over_foreign_lockfiles();
}

#[test]
fn native_python_lane_rejects_unowned_lockfiles_and_dependency_sources() {
    integration::python_lane::native_python_lane_rejects_unowned_lockfiles_and_dependency_sources();
}

#[test]
fn detect_python_agent_via_pyproject_tool_magicore() {
    integration::project_detection::detect_python_agent_via_pyproject_tool_magicore();
}

#[test]
fn detect_mcp_server_via_pyproject_tool_magicore() {
    integration::project_detection::detect_mcp_server_via_pyproject_tool_magicore();
}

#[test]
fn detect_python_agent_via_mgc_toml_ai_section() {
    integration::project_detection::detect_python_agent_via_mgc_toml_ai_section();
}

#[test]
fn detect_mcp_server_via_mgc_toml_ai_section() {
    integration::project_detection::detect_mcp_server_via_mgc_toml_ai_section();
}

#[test]
fn detect_returns_none_with_no_marker_files() {
    integration::project_detection::detect_returns_none_with_no_marker_files();
}

#[test]
fn detect_returns_none_for_unknown_framework_value() {
    integration::project_detection::detect_returns_none_for_unknown_framework_value();
}

#[test]
fn adapter_for_returns_some_with_valid_marker() {
    integration::project_detection::adapter_for_returns_some_with_valid_marker();
}

#[test]
fn adapter_for_returns_none_without_marker() {
    integration::project_detection::adapter_for_returns_none_without_marker();
}

#[test]
fn aiframework_as_str_matches_scaffold_key() {
    integration::adapter_contract::aiframework_as_str_matches_scaffold_key();
}

#[test]
fn aiframework_entry_script_matches_scaffold() {
    integration::adapter_contract::aiframework_entry_script_matches_scaffold();
}

#[test]
fn adapter_name_and_ecosystem() {
    integration::adapter_contract::adapter_name_and_ecosystem();
}

#[test]
fn adapter_can_handle_returns_true_for_marked_project() {
    integration::adapter_contract::adapter_can_handle_returns_true_for_marked_project();
}

#[test]
fn adapter_can_handle_returns_false_for_empty_dir() {
    integration::adapter_contract::adapter_can_handle_returns_false_for_empty_dir();
}

#[test]
fn non_lane_projects_do_not_claim_registry_capabilities() {
    integration::adapter_contract::non_lane_projects_do_not_claim_registry_capabilities();
}

#[tokio::test]
async fn pypi_lane_claims_registry_capabilities_and_resolves_natively() {
    integration::python_lane::pypi_lane_claims_registry_capabilities_and_resolves_natively().await;
}

#[tokio::test]
async fn parse_manifest_uses_dir_name_as_project_name() {
    integration::python_lane::parse_manifest_uses_dir_name_as_project_name().await;
}

#[tokio::test]
async fn resolve_fails_closed_without_native_python_lane() {
    integration::python_lane::resolve_fails_closed_without_native_python_lane().await;
}

#[tokio::test]
async fn install_fails_closed_with_descriptive_error() {
    integration::python_lane::install_fails_closed_with_descriptive_error().await;
}

#[tokio::test]
async fn add_fails_closed_with_descriptive_error() {
    integration::python_lane::add_fails_closed_with_descriptive_error().await;
}

#[tokio::test]
async fn update_fails_closed_with_descriptive_error() {
    integration::python_lane::update_fails_closed_with_descriptive_error().await;
}

#[tokio::test]
async fn remove_fails_closed_without_mutating_through_adapter_api() {
    integration::python_lane::remove_fails_closed_without_mutating_through_adapter_api().await;
}

#[tokio::test]
async fn audit_returns_clean_report_for_empty_project() {
    integration::audit_sbom::audit_returns_clean_report_for_empty_project().await;
}

#[tokio::test]
async fn ai_audit_scans_each_model_root_and_checkpoint_weights() {
    integration::audit_sbom::ai_audit_scans_each_model_root_and_checkpoint_weights().await;
}

#[tokio::test]
async fn list_refuses_to_claim_an_mgc_owned_set_for_foreign_lane() {
    integration::python_lane::list_refuses_to_claim_an_mgc_owned_set_for_foreign_lane().await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}
