#![allow(clippy::unwrap_used)]
//! App adapter integration tests — integration test App adapter.
//! Stable Cargo harness; test logic is in owner modules. — Harness Cargo giữ ổn định; test nằm trong module owner.

#[path = "integration/mod.rs"]
mod integration;

#[test]
fn lock_replay_manifest_preserves_explicit_non_sdk_requirements() {
    integration::lock_replay::lock_replay_manifest_preserves_explicit_non_sdk_requirements();
}

#[cfg(unix)]
#[test]
fn language_detection_does_not_follow_external_mgc_config_symlink() {
    integration::language_detection::language_detection_does_not_follow_external_mgc_config_symlink(
    );
}

#[cfg(unix)]
#[test]
fn language_detection_does_not_follow_external_package_json_symlink() {
    integration::language_detection::language_detection_does_not_follow_external_package_json_symlink();
}

#[test]
fn detect_flutter_via_pubspec_yaml() {
    integration::language_detection::detect_flutter_via_pubspec_yaml();
}

#[test]
fn detect_kotlin_via_build_gradle_kts() {
    integration::language_detection::detect_kotlin_via_build_gradle_kts();
}

#[test]
fn detect_kotlin_via_build_gradle() {
    integration::language_detection::detect_kotlin_via_build_gradle();
}

#[test]
fn detect_swift_via_package_swift() {
    integration::language_detection::detect_swift_via_package_swift();
}

#[test]
fn detect_maui_via_explicit_use_maui_csproj() {
    integration::language_detection::detect_maui_via_explicit_use_maui_csproj();
}

#[test]
fn maui_detection_refuses_ambiguous_root_projects() {
    integration::language_detection::maui_detection_refuses_ambiguous_root_projects();
}

#[test]
fn detect_tauri_via_rust_manifest_and_config() {
    integration::language_detection::detect_tauri_via_rust_manifest_and_config();
}

#[test]
fn detect_react_native_via_package_json_dep() {
    integration::language_detection::detect_react_native_via_package_json_dep();
}

#[test]
fn react_native_manifest_identity_uses_canonical_gate_id() {
    integration::language_detection::react_native_manifest_identity_uses_canonical_gate_id();
}

#[test]
fn detect_objc_via_bridge_header_and_impl_pair() {
    integration::language_detection::detect_objc_via_bridge_header_and_impl_pair();
}

#[test]
fn detect_via_mgc_toml_language_overrides_marker() {
    integration::language_detection::detect_via_mgc_toml_language_overrides_marker();
}

#[test]
fn detect_multi_via_mgc_toml() {
    integration::language_detection::detect_multi_via_mgc_toml();
}

#[test]
fn detect_react_native_via_explicit_mgc_toml_language() {
    integration::language_detection::detect_react_native_via_explicit_mgc_toml_language();
}

#[test]
fn detect_returns_none_for_empty_dir() {
    integration::language_detection::detect_returns_none_for_empty_dir();
}

#[test]
fn detect_returns_none_for_unknown_mgc_toml_language() {
    integration::language_detection::detect_returns_none_for_unknown_mgc_toml_language();
}

#[test]
fn package_json_without_react_native_is_not_detected_as_rn() {
    integration::language_detection::package_json_without_react_native_is_not_detected_as_rn();
}

#[test]
fn adapter_for_returns_some_with_pubspec_marker() {
    integration::language_detection::adapter_for_returns_some_with_pubspec_marker();
}

#[test]
fn adapter_for_returns_none_without_any_marker() {
    integration::language_detection::adapter_for_returns_none_without_any_marker();
}

#[test]
fn applanguage_as_str_values() {
    integration::adapter_contract::applanguage_as_str_values();
}

#[test]
fn adapter_name_and_ecosystem() {
    integration::adapter_contract::adapter_name_and_ecosystem();
}

#[test]
fn can_handle_returns_true_for_pubspec_project() {
    integration::adapter_contract::can_handle_returns_true_for_pubspec_project();
}

#[test]
fn can_handle_returns_false_for_empty_dir() {
    integration::adapter_contract::can_handle_returns_false_for_empty_dir();
}

#[tokio::test]
async fn parse_manifest_derives_name_from_dir() {
    integration::lifecycle::parse_manifest_derives_name_from_dir().await;
}

#[tokio::test]
async fn flutter_resolve_empty_deps_returns_empty_graph() {
    integration::lifecycle::flutter_resolve_empty_deps_returns_empty_graph().await;
}

#[tokio::test]
async fn kotlin_resolve_fails_closed_toolchain_owned() {
    integration::lifecycle::kotlin_resolve_fails_closed_toolchain_owned().await;
}

#[tokio::test]
async fn native_flutter_empty_install_writes_mgc_lock_without_provider_resolution() {
    integration::lifecycle::native_flutter_empty_install_writes_mgc_lock_without_provider_resolution().await;
}

#[tokio::test]
async fn add_fails_closed_directs_to_tooling() {
    integration::lifecycle::add_fails_closed_directs_to_tooling().await;
}

#[tokio::test]
async fn remove_fails_closed_directs_to_tooling() {
    integration::lifecycle::remove_fails_closed_directs_to_tooling().await;
}

#[tokio::test]
async fn update_fails_closed_directs_to_tooling() {
    integration::lifecycle::update_fails_closed_directs_to_tooling().await;
}

#[tokio::test]
async fn audit_returns_clean_for_empty_project() {
    integration::audit_sbom::audit_returns_clean_for_empty_project().await;
}

#[tokio::test]
async fn list_returns_empty_for_no_deps() {
    integration::lifecycle::list_returns_empty_for_no_deps().await;
}

#[tokio::test]
async fn list_does_not_claim_unresolved_manifest_ranges_are_installed_versions() {
    integration::lifecycle::list_does_not_claim_unresolved_manifest_ranges_are_installed_versions()
        .await;
}

#[test]
fn generate_sbom_uses_lockfile_v2_fixture() {
    integration::audit_sbom::generate_sbom_uses_lockfile_v2_fixture();
}
