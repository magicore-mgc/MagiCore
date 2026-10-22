#![allow(clippy::unwrap_used, unsafe_code)]
//! Lib integration tests — integration test của Lib adapter.
//! Stable Cargo harness; behavior lives in owner modules. — Harness Cargo giữ ổn định; test ở owner.

#[path = "integration/mod.rs"]
mod integration;

#[cfg(unix)]
#[test]
fn language_detection_does_not_follow_external_mgc_config_symlink() {
    integration::language_detection::language_detection_does_not_follow_external_mgc_config_symlink(
    );
}

#[cfg(unix)]
#[test]
fn language_detection_does_not_follow_external_cargo_manifest_symlink() {
    integration::language_detection::language_detection_does_not_follow_external_cargo_manifest_symlink();
}

#[test]
fn detect_rust_project_via_cargo_toml_with_magicore_metadata() {
    integration::language_detection::detect_rust_project_via_cargo_toml_with_magicore_metadata();
}

#[test]
fn detect_ts_project_via_mgc_toml() {
    integration::language_detection::detect_ts_project_via_mgc_toml();
}

#[test]
fn ts_lane_claims_materializer_and_lifecycle_via_web_engine() {
    integration::capability_contract::ts_lane_claims_materializer_and_lifecycle_via_web_engine();
}

#[test]
fn toolchain_languages_do_not_claim_materializer_or_lifecycle() {
    integration::capability_contract::toolchain_languages_do_not_claim_materializer_or_lifecycle();
}

#[test]
fn java_gradle_does_not_claim_native_dependency_operations() {
    integration::capability_contract::java_gradle_does_not_claim_native_dependency_operations();
}

#[test]
fn detect_python_project_via_mgc_toml() {
    integration::language_detection::detect_python_project_via_mgc_toml();
}

#[test]
fn adapter_for_returns_none_for_empty_dir() {
    integration::language_detection::adapter_for_returns_none_for_empty_dir();
}

#[test]
fn pip_allowlist_cannot_enable_external_package_manager() {
    integration::python_ownership::pip_allowlist_cannot_enable_external_package_manager();
}

#[test]
fn adapter_name_and_ecosystem() {
    integration::capability_contract::adapter_name_and_ecosystem();
}

#[tokio::test]
async fn rust_manifest_roundtrip() {
    integration::manifest_io::rust_manifest_roundtrip().await;
}

#[tokio::test]
async fn rust_write_manifest_preserves_magicore_metadata() {
    integration::manifest_io::rust_write_manifest_preserves_magicore_metadata().await;
}

#[tokio::test]
async fn audit_lib_project_reports_scanner_truthfully() {
    integration::audit_sbom::audit_lib_project_reports_scanner_truthfully().await;
}

#[tokio::test]
async fn python_manifest_roundtrip_preserves_project_table() {
    integration::manifest_io::python_manifest_roundtrip_preserves_project_table().await;
}

#[tokio::test]
async fn direct_python_adapter_mutations_fail_closed_outside_mutation_gateway() {
    integration::lifecycle::direct_python_adapter_mutations_fail_closed_outside_mutation_gateway()
        .await;
}

#[tokio::test]
async fn rust_list_does_not_confuse_lock_pins_with_installed_packages() {
    integration::lifecycle::rust_list_does_not_confuse_lock_pins_with_installed_packages().await;
}

#[tokio::test]
async fn rust_list_is_unsupported_even_when_manifest_has_no_dependencies() {
    integration::lifecycle::rust_list_is_unsupported_even_when_manifest_has_no_dependencies().await;
}

#[tokio::test]
async fn python_list_does_not_claim_venv_packages_without_mgc_lock() {
    integration::lifecycle::python_list_does_not_claim_venv_packages_without_mgc_lock().await;
}

#[tokio::test]
async fn python_foreign_lock_keeps_native_mutation_and_install_unclaimed() {
    integration::python_ownership::python_foreign_lock_keeps_native_mutation_and_install_unclaimed(
    )
    .await;
}

#[tokio::test]
async fn ts_delegate_ignores_workspace_protocol_dependencies() {
    integration::manifest_io::ts_delegate_ignores_workspace_protocol_dependencies().await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}
